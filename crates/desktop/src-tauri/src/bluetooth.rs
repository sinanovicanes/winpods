//! Wires the bluetooth watchers into the app.
//!
//! Two tasks: one follows the adapter and toggles scanning with it, the other feeds received
//! advertisements into the device service and picks a device up when there is none.

use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use tokio::sync::broadcast::error::RecvError;
use winpods_apple_cp::{ProximityPairingMessage, VENDOR_ID};
use winpods_bluetooth::{AdapterState, Advertisement, DiscoveredDevice, device};

use crate::{events::AppEvent, state::AppState};

/// Starts watching bluetooth.
///
/// Returns immediately: attaching to the radio and enumerating devices are WinRT round trips, and
/// the setup hook must not block on them or the windows stay unresponsive while it waits.
pub fn init(state: &Arc<AppState>) {
    let state = Arc::clone(state);

    tauri::async_runtime::spawn(async move {
        start(&state).await;
    });
}

/// Attaches the watchers and picks up an already connected Apple device, if there is one.
async fn start(state: &Arc<AppState>) {
    spawn_adapter_task(state);
    spawn_advertisement_task(state);

    state.adapter.start().await;

    // Only scan when the adapter is on. If it is not reachable yet, the adapter watcher keeps
    // looking and the task above starts scanning as soon as it appears.
    if state.adapter.state().is_on() {
        if let Err(e) = state.advertisements.start() {
            tracing::error!("Failed to start scanning for advertisements: {e:#}");
        }
    }

    select_connected_device(state).await;
}

/// Selects a supported device that is connected right now, if the user has not forgotten it.
///
/// Runs at startup for a device that was already connected, and again from [`AutoSelect`] once one
/// turns up later, so a device that connects while the app is in the tray still shows its battery
/// without the dashboard being opened.
async fn select_connected_device(state: &Arc<AppState>) -> Outcome {
    let devices = match device::connected_devices().await {
        Ok(devices) => devices,
        Err(e) => {
            tracing::error!("Failed to look for a connected device: {e:#}");
            return Outcome::NothingToSelect;
        }
    };

    // Reconnecting a forgotten device is a fresh intent, so the suppression only lasts as long as
    // it stays connected. Re-selecting it while it never went away would just undo the "forget".
    let mut forgotten = state.devices.forgotten().await;

    if let Some(address) = forgotten {
        if !devices.iter().any(|device| device.address == address) {
            state.devices.release_forgotten().await;
            forgotten = None;
        }
    }

    let mut supported = devices.into_iter().filter(is_supported);

    let Some(found) = supported.find(|device| forgotten != Some(device.address)) else {
        return match forgotten {
            Some(_) => {
                tracing::debug!("The only supported connected device is the forgotten one");
                Outcome::OnlyForgotten
            }
            None => {
                tracing::debug!("No supported connected device to select");
                Outcome::NothingToSelect
            }
        };
    };

    let device = match found.open().await {
        Ok(device) => device,
        Err(e) => {
            tracing::error!("Failed to open `{}`: {e:#}", found.name);
            return Outcome::NothingToSelect;
        }
    };

    match state.devices.select(device).await {
        Ok(_) => Outcome::Selected,
        Err(e) => {
            tracing::error!("Failed to select `{}`: {e:#}", found.name);
            Outcome::NothingToSelect
        }
    }
}

/// Whether a connected device is one the app can actually report on.
///
/// The model has to be recognised, not merely the vendor: Apple keyboards, mice and phones pair
/// over bluetooth under the same vendor id, and selecting one would leave the dashboard on a device
/// that never reports a battery while the real AirPods sat there unselected.
fn is_supported(device: &DiscoveredDevice) -> bool {
    device.vendor_id == Some(VENDOR_ID) && device.model().is_supported()
}

/// What an attempt to select a device found, which decides how soon the next one is worth making.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Selected,
    /// Nothing connected that the app supports.
    NothingToSelect,
    /// The only supported device is the one the user forgot.
    OnlyForgotten,
}

/// Follows the adapter, toggling the advertisement scan and telling the UI.
fn spawn_adapter_task(state: &Arc<AppState>) {
    let state = Arc::clone(state);
    let mut adapter_events = state.adapter.subscribe();

    tauri::async_runtime::spawn(async move {
        while let Ok(adapter_state) = adapter_events.recv().await {
            // Scanning is pointless with the radio off, and Windows stops the watcher itself
            // anyway; doing it explicitly keeps our own view of it accurate.
            let result = match adapter_state {
                AdapterState::On => state.advertisements.start(),
                AdapterState::Off => state.advertisements.stop(),
            };

            if let Err(e) = result {
                tracing::error!("Failed to follow the adapter to {adapter_state:?}: {e:#}");
            }

            state.publish(AppEvent::AdapterStateChanged(adapter_state));
        }
    });
}

/// How long to wait between attempts to pick a device up.
///
/// Advertisements arrive several times a second and each attempt enumerates the connected devices,
/// so the trigger has to be rate limited rather than followed.
const AUTO_SELECT_INTERVAL: Duration = Duration::from_secs(5);

/// The wait after an attempt that found only the device the user forgot.
///
/// Nothing can change until it disconnects, and the app notices that within this interval; without
/// the longer wait, forgetting a device that stays connected would scan every few seconds forever.
const AUTO_SELECT_BACKOFF: Duration = Duration::from_secs(60);

/// Picks a device up on its own when there is nothing selected.
///
/// Advertisements are the trigger rather than a bluetooth connection event, for two reasons: they
/// are the signal the app actually needs, since a connected device that is not advertising has no
/// battery to report, and the stream is already running, so no second WinRT watcher has to be
/// attached and torn down.
#[derive(Default)]
struct AutoSelect {
    last_attempt: Option<Instant>,
    /// Grows to [`AUTO_SELECT_BACKOFF`] while the only supported device is a forgotten one.
    wait: Option<Duration>,
}

impl AutoSelect {
    async fn on_advertisement(&mut self, state: &Arc<AppState>, advertisement: &Advertisement) {
        // The watcher already filters to Apple's company id, so the payload only has to prove it
        // came from the kind of hardware the app supports rather than from a phone or a watch.
        if !is_proximity_pairing(advertisement) || self.waiting() {
            return;
        }

        if state.devices.is_selected().await {
            return;
        }

        self.last_attempt = Some(Instant::now());
        self.wait = match select_connected_device(state).await {
            Outcome::OnlyForgotten => Some(AUTO_SELECT_BACKOFF),
            _ => None,
        };
    }

    fn waiting(&self) -> bool {
        let wait = self.wait.unwrap_or(AUTO_SELECT_INTERVAL);

        self.last_attempt
            .is_some_and(|attempt| attempt.elapsed() < wait)
    }
}

/// Whether an advertisement carries a proximity pairing payload, so AirPods class hardware is in
/// range. Its readings are not used here -- the device to select is the connected one, not
/// whichever pair happens to be broadcasting nearby.
fn is_proximity_pairing(advertisement: &Advertisement) -> bool {
    advertisement
        .manufacturer_data_for(VENDOR_ID)
        .and_then(ProximityPairingMessage::from_bytes)
        .is_some()
}

/// Feeds advertisements into the device service, selecting a device when there is none.
fn spawn_advertisement_task(state: &Arc<AppState>) {
    let state = Arc::clone(state);
    let mut advertisements = state.advertisements.subscribe();

    tauri::async_runtime::spawn(async move {
        let mut auto_select = AutoSelect::default();

        loop {
            match advertisements.recv().await {
                Ok(advertisement) => {
                    state.devices.apply_advertisement(&advertisement).await;
                    auto_select.on_advertisement(&state, &advertisement).await;
                }
                // Advertisements are a continuous stream of the current state, so dropping the
                // backlog costs nothing -- the next one carries fresher readings than any of the
                // ones that were skipped.
                Err(RecvError::Lagged(skipped)) => {
                    tracing::debug!("Skipped {skipped} advertisement(s) while busy");
                }
                Err(RecvError::Closed) => break,
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn device(vendor_id: Option<u16>, product_id: Option<u16>) -> DiscoveredDevice {
        DiscoveredDevice {
            address: 0xf004e1c3d4e5,
            name: "Anes's AirPods Pro".to_string(),
            vendor_id,
            product_id,
        }
    }

    #[test]
    fn selects_a_recognised_apple_model() {
        assert!(is_supported(&device(Some(VENDOR_ID), Some(0x2024))));
    }

    #[test]
    fn ignores_apple_devices_that_are_not_headphones() {
        // A keyboard or a mouse: Apple's vendor id, but a product id that means nothing to us, so
        // selecting it would park the dashboard on a device with no battery to report.
        assert!(!is_supported(&device(Some(VENDOR_ID), Some(0x0000))));
        assert!(!is_supported(&device(Some(VENDOR_ID), None)));
    }

    #[test]
    fn ignores_other_vendors() {
        // Product ids are Apple's id space, so a collision from another vendor means nothing.
        assert!(!is_supported(&device(Some(0x0006), Some(0x2024))));
        assert!(!is_supported(&device(None, Some(0x2024))));
    }
}
