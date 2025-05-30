use bluetooth::{DeviceConnectionState, apple_cp, find_connected_device_with_vendor_id};
use std::sync::RwLock;
use tauri::{App, Emitter, Manager};

use crate::{events, models::DeviceInfo};

mod device_properties;
mod manager;

pub use device_properties::DeviceProperties;
pub use manager::DeviceManagerState;

pub fn init(app: &mut App) {
    let mut state = DeviceManagerState::new();

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
    }

    app.manage(RwLock::new(state));
}
