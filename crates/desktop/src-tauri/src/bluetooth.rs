//! Wires the bluetooth watchers into the app.
//!
//! Two tasks: one follows the adapter and toggles scanning with it, the other feeds received
//! advertisements into the device service.

use std::sync::Arc;

use tokio::sync::broadcast::error::RecvError;
use winpods_apple_cp::VENDOR_ID;
use winpods_bluetooth::{AdapterState, device};

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

/// Picks up an Apple device that was already connected when the app started.
async fn select_connected_device(state: &Arc<AppState>) {
    match device::find_connected_device_by_vendor(VENDOR_ID).await {
        Ok(Some(found)) => match found.open().await {
            Ok(device) => {
                if let Err(e) = state.devices.select(device).await {
                    tracing::error!("Failed to select the connected device: {e:#}");
                }
            }
            Err(e) => tracing::error!("Failed to open the connected device: {e:#}"),
        },
        Ok(None) => tracing::info!("No connected Apple device found"),
        Err(e) => tracing::error!("Failed to look for a connected Apple device: {e:#}"),
    }
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

/// Feeds advertisements into the device service.
fn spawn_advertisement_task(state: &Arc<AppState>) {
    let state = Arc::clone(state);
    let mut advertisements = state.advertisements.subscribe();

    tauri::async_runtime::spawn(async move {
        loop {
            match advertisements.recv().await {
                Ok(advertisement) => state.devices.apply_advertisement(&advertisement).await,
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
