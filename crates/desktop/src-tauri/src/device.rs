//! Device selection and live readings.

use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use anyhow::{Context, Result};
use serde::Serialize;
use tokio::{
    sync::{RwLock, broadcast},
    task::JoinHandle,
};
use winpods_apple_cp::{ProximityPairingMessage, VENDOR_ID};
use winpods_bluetooth::{Advertisement, ConnectionState, Device, DeviceEvent, display_name};
use winpods_core::{DeviceInfo, DeviceProperties};

use crate::events::AppEvent;

/// What the UI needs to render the current device on first paint.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceSnapshot {
    pub device: Option<DeviceInfo>,
    pub properties: Option<DeviceProperties>,
}

/// Owns the selected device, its cached identity and its most recent readings.
pub struct DeviceService {
    selection: RwLock<Selection>,
    /// Watches the selected device's events. Replaced whenever the selection changes.
    watcher: Mutex<Option<JoinHandle<()>>>,
    events: broadcast::Sender<AppEvent>,
}

#[derive(Default)]
struct Selection {
    device: Option<Device>,
    /// Cached so reads do not need a WinRT round trip for the name and model.
    info: Option<DeviceInfo>,
    properties: Option<DeviceProperties>,
    /// Address the user cleared, which auto-selection must not pick straight back up.
    ///
    /// Deliberately not persisted: it suppresses one selection, it is not a preference. A
    /// forgotten device becomes eligible again once it disconnects, or once the app restarts.
    forgotten: Option<u64>,
}

impl DeviceService {
    pub fn new(events: broadcast::Sender<AppEvent>) -> Self {
        Self {
            selection: RwLock::new(Selection::default()),
            watcher: Mutex::new(None),
            events,
        }
    }

    /// Selects `device` and starts reporting its changes.
    ///
    /// Replaces any previous selection, cancelling the task that watched it.
    pub async fn select(self: &Arc<Self>, device: Device) -> Result<DeviceInfo> {
        let address = device
            .address()
            .context("device has no bluetooth address")?;

        let info = DeviceInfo {
            address,
            name: display_name(device.name().ok(), address),
            connection_state: device.connection_state(),
            model: device.model().await,
        };

        let mut selection = self.selection.write().await;
        selection.device = Some(device.clone());
        selection.info = Some(info.clone());
        // Readings belong to the previous device; keep none rather than showing its batteries.
        selection.properties = None;
        // Choosing a device is the user engaging with the picker, so an earlier "forget" has had
        // its effect and should not outlive it.
        selection.forgotten = None;
        drop(selection);

        self.spawn_watcher(&device);

        tracing::info!("Selected device: {info:?}");
        let _ = self.events.send(AppEvent::DeviceSelected(info.clone()));

        Ok(info)
    }

    /// Clears the selection and stops watching.
    pub async fn clear(&self) {
        let mut selection = self.selection.write().await;

        if selection.device.is_none() {
            return;
        }

        // Remembered across the reset: without it the next advertisement would pick the same
        // device straight back up and "Forget this device" would appear to do nothing.
        let forgotten = selection.info.as_ref().map(|info| info.address);
        *selection = Selection {
            forgotten,
            ..Selection::default()
        };
        drop(selection);

        self.abort_watcher();

        tracing::info!("Device selection cleared");
        let _ = self.events.send(AppEvent::DeviceSelectionCleared);
    }

    /// Whether a device is currently selected.
    pub async fn is_selected(&self) -> bool {
        self.selection.read().await.device.is_some()
    }

    /// The address the user cleared, which auto-selection skips.
    pub async fn forgotten(&self) -> Option<u64> {
        self.selection.read().await.forgotten
    }

    /// Makes a forgotten device eligible again, once it is no longer connected.
    pub async fn release_forgotten(&self) {
        self.selection.write().await.forgotten = None;
    }

    /// The current device and readings.
    pub async fn snapshot(&self) -> DeviceSnapshot {
        let selection = self.selection.read().await;

        DeviceSnapshot {
            device: selection.info.clone(),
            properties: selection.properties,
        }
    }

    /// Feeds an advertisement in, updating the readings if it belongs to the selected device.
    ///
    /// Advertisements arrive constantly, so this is the hottest path in the app and returns as
    /// early as it can. Each rejection reason is distinct and worth keeping straight:
    ///
    /// - no Apple payload, or one that is not a proximity pairing message
    /// - no device selected, or the selected device is disconnected
    /// - a different model, so plainly another device
    /// - the same model but implausibly different readings, so probably another pair nearby
    pub async fn apply_advertisement(&self, advertisement: &Advertisement) {
        let Some(payload) = advertisement.manufacturer_data_for(VENDOR_ID) else {
            return;
        };

        let Some(message) = ProximityPairingMessage::from_bytes(payload) else {
            return;
        };

        let incoming = DeviceProperties::from_advertisement(
            advertisement.rssi,
            advertisement.address,
            &message,
        );

        // Read first: the overwhelming majority of advertisements are rejected, and taking a
        // write lock for each one would serialise the whole hot path.
        {
            let selection = self.selection.read().await;

            let Some(device) = &selection.device else {
                return;
            };

            if !device.is_connected() {
                return;
            }

            let Some(info) = &selection.info else {
                return;
            };

            if info.model != incoming.model {
                return;
            }

            if let Some(current) = selection.properties {
                if !current.is_plausible_update(&incoming) {
                    tracing::debug!(
                        "Ignored an advertisement with implausible readings: {incoming:?}"
                    );
                    return;
                }

                if current == incoming {
                    // Identical readings; nothing for the UI to redraw.
                    return;
                }
            }
        }

        let mut selection = self.selection.write().await;

        // Re-check under the write lock: the selection may have changed while it was released.
        if selection.device.is_none() {
            return;
        }

        selection.properties = Some(incoming);
        drop(selection);

        let _ = self
            .events
            .send(AppEvent::DevicePropertiesUpdated(incoming));
    }

    /// Watches one device's connection and name changes until the selection changes.
    fn spawn_watcher(self: &Arc<Self>, device: &Device) {
        let mut events = device.subscribe();
        let service = Arc::clone(self);

        let handle = tokio::spawn(async move {
            while let Ok(event) = events.recv().await {
                match event {
                    DeviceEvent::ConnectionChanged(state) => {
                        service.on_connection_changed(state).await;
                    }
                    DeviceEvent::NameChanged(name) => service.on_name_changed(name).await,
                }
            }
        });

        if let Some(previous) = lock(&self.watcher).replace(handle) {
            previous.abort();
        }
    }

    fn abort_watcher(&self) {
        if let Some(handle) = lock(&self.watcher).take() {
            handle.abort();
        }
    }

    async fn on_connection_changed(&self, state: ConnectionState) {
        tracing::info!("Device connection state changed: {state:?}");

        let mut selection = self.selection.write().await;

        if let Some(info) = &mut selection.info {
            info.connection_state = state;
        }

        // A disconnected device reports nothing, so its last readings are stale immediately.
        if !state.is_connected() {
            selection.properties = None;
        }

        drop(selection);

        let _ = self.events.send(AppEvent::DeviceConnectionChanged(state));
    }

    async fn on_name_changed(&self, name: String) {
        let mut selection = self.selection.write().await;

        let Some(info) = &mut selection.info else {
            return;
        };

        // Resolved here rather than at the source so the event and the snapshot cannot disagree
        // about what the device is called.
        let name = display_name(Some(name), info.address);
        info.name = name.clone();
        drop(selection);

        tracing::info!("Device name changed: {name}");
        let _ = self.events.send(AppEvent::DeviceNameChanged(name));
    }
}

impl Drop for DeviceService {
    fn drop(&mut self) {
        self.abort_watcher();
    }
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}
