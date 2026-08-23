//! Shared application state.

use std::sync::Arc;

use anyhow::{Context, Result};
use tokio::sync::broadcast;
use winpods_bluetooth::{AdapterWatcher, AdvertisementWatcher};

use crate::{device::DeviceService, events::AppEvent, settings::SettingsService};

/// Room for a burst of events while a feature task is busy.
const EVENT_CAPACITY: usize = 64;

/// Everything the commands and background tasks share.
///
/// Managed by Tauri as `Arc<AppState>` so commands, feature tasks and the tray all reach the same
/// instance.
pub struct AppState {
    pub events: broadcast::Sender<AppEvent>,
    pub settings: SettingsService,
    pub devices: Arc<DeviceService>,
    pub adapter: AdapterWatcher,
    pub advertisements: AdvertisementWatcher,
}

impl AppState {
    /// Builds the state.
    ///
    /// Constructed before the Tauri app runs, because the windows declared in
    /// `tauri.conf.json` exist before the setup hook and their webviews can invoke commands
    /// while setup is still in progress. Managing the state up front is what keeps those early
    /// calls from failing with "state not managed".
    pub fn new(settings_path: std::path::PathBuf) -> Result<Arc<Self>> {
        let (events, _) = broadcast::channel(EVENT_CAPACITY);

        // Filter to Apple's company id inside the WinRT callback. The handler runs for every
        // advertisement in radio range, and this is the only vendor the app decodes.
        let advertisements = AdvertisementWatcher::for_company(winpods_apple_cp::VENDOR_ID)
            .context("failed to create the BLE advertisement watcher")?;

        Ok(Arc::new(Self {
            settings: SettingsService::new(settings_path, events.clone()),
            devices: Arc::new(DeviceService::new(events.clone())),
            adapter: AdapterWatcher::new(),
            advertisements,
            events,
        }))
    }

    /// Publishes an event on the internal bus.
    ///
    /// A send failing only means nothing is subscribed yet, which is normal during startup.
    pub fn publish(&self, event: AppEvent) {
        let _ = self.events.send(event);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<AppEvent> {
        self.events.subscribe()
    }
}
