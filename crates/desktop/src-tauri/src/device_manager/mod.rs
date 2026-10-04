use bluetooth::{
    DeviceConnectionState,
    apple_cp::{self, AppleDeviceExt, AppleDeviceModel},
    find_connected_device_with_vendor_id, get_connected_device_list,
};
use std::{
    sync::{Mutex, RwLock},
    time::{Duration, Instant},
};
use tauri::{App, AppHandle, Emitter, Manager};

use crate::{events, models::DeviceInfo};

mod device_properties;
mod manager;

pub use device_properties::DeviceProperties;
pub use manager::DeviceManagerState;

pub fn init(app: &mut App) {
    let state_lock = app.state::<RwLock<DeviceManagerState>>();
    let mut state = state_lock.write().unwrap();

    let app_handle: tauri::AppHandle = app.app_handle().clone();
    state.on_device_selected(move |device| {
        tracing::info!("Device selected: {:?}", device);
        app_handle
            .emit(events::DEVICE_SELECTED, DeviceInfo::from(device))
            .unwrap_or_else(|e| {
                tracing::error!("Failed to emit device selected event: {}", e);
            });
    });

    let app_handle: tauri::AppHandle = app.app_handle().clone();
    state.on_device_selection_cleared(move || {
        tracing::info!("Device selection cleared");
        app_handle
            .emit(events::DEVICE_SELECTION_CLEARED, "")
            .unwrap_or_else(|e| {
                tracing::error!("Failed to emit device selection cleared event: {}", e);
            });
    });

    let app_handle: tauri::AppHandle = app.app_handle().clone();
    state.on_device_name_changed(move |name| {
        tracing::info!("Device name changed: {}", name);
        app_handle
            .emit(events::DEVICE_NAME_UPDATED, name)
            .unwrap_or_else(|e| {
                tracing::error!("Failed to emit device name updated event: {}", e);
            });
    });

    let app_handle: tauri::AppHandle = app.app_handle().clone();
    state.on_device_connection_changed(move |state| {
        tracing::info!("Device connection state changed: {:?}", state);
        app_handle
            .emit(events::DEVICE_CONNECTION_STATE_UPDATED, state)
            .unwrap_or_else(|e| {
                tracing::error!(
                    "Failed to emit device connection state updated event: {}",
                    e
                );
            });

        // Clear device properties if disconnected
        if matches!(state, DeviceConnectionState::Disconnected) {
            let device_manager = app_handle.state::<RwLock<DeviceManagerState>>();
            let mut device_manager = device_manager.write().unwrap();
            device_manager.device_properties = None;
        }
    });

    if let Some(device) = find_connected_device_with_vendor_id(apple_cp::VENDOR_ID) {
        state.select_device(device);
    } else {
        tracing::info!("No connected Apple device found");
    }
}

/// How long to wait between attempts to switch away from a disconnected device.
///
/// Advertisements arrive several times a second and each attempt enumerates the connected devices.
const SWITCH_INTERVAL: Duration = Duration::from_secs(5);

static LAST_SWITCH_ATTEMPT: Mutex<Option<Instant>> = Mutex::new(None);

/// Selects another connected Apple device when the selected one has disconnected.
///
/// The selection used to be made only at startup, so switching to another pair left the dashboard
/// on the old one with no readings until the app was restarted (issue #10). A selection that is
/// still connected is never replaced, and a cleared selection is left alone.
///
/// Blocks on WinRT, so it must not be called with the device manager lock held.
pub fn switch_from_disconnected_device(app_handle: &AppHandle) {
    {
        let mut last_attempt = LAST_SWITCH_ATTEMPT.lock().unwrap();

        if last_attempt.is_some_and(|attempt| attempt.elapsed() < SWITCH_INTERVAL) {
            return;
        }

        *last_attempt = Some(Instant::now());
    }

    // Matched on the model, not just the vendor: Apple keyboards and mice pair under the same
    // vendor id, and switching to one would park the dashboard on a device that never reports a
    // battery.
    let Some(device) = get_connected_device_list().into_iter().find(|device| {
        device.get_vendor_id() == Ok(apple_cp::VENDOR_ID)
            && device.get_device_model() != AppleDeviceModel::Unknown
    }) else {
        return;
    };

    // Resolved before taking the lock, since reading the name is a WinRT round trip.
    let name = device.get_name().unwrap_or_default();

    let device_manager = app_handle.state::<RwLock<DeviceManagerState>>();
    let mut device_manager = device_manager.write().unwrap();

    // Re-check under the lock: the user may have cleared the selection, or the old device may
    // have come back, while the lock was released for the lookup.
    let Some(current) = &device_manager.device else {
        return;
    };

    if current.is_connected() {
        return;
    }

    tracing::info!("Selected device disconnected, switching to {}", name);
    device_manager.select_device(device);
}
