use bluetooth::{AdapterState, apple_cp, find_connected_device_with_vendor_id};
use std::sync::RwLock;
use tauri::{App, Emitter, Manager};

use crate::events;

mod advertisement_watcher;
mod device_properties;
mod manager;

pub use device_properties::DeviceProperties;
pub use manager::DeviceManagerState;

pub fn init(app: &mut App) {
    let mut state = DeviceManagerState::new();

    let app_handle = app.app_handle().clone();
    state.adapter_watcher.on_state_changed(move |state| {
        tracing::info!("Bluetooth adapter state changed: {:?}", state);
        let device_manager = app_handle.state::<RwLock<DeviceManagerState>>();
        let device_manager = device_manager.read().unwrap();

        match state {
            AdapterState::On => device_manager.adv_watcher.start().unwrap_or_else(|_| {
                tracing::error!("Failed to start AdvertisementWatcher");
            }),
            AdapterState::Off => device_manager.adv_watcher.stop().unwrap_or_else(|_| {
                tracing::error!("Failed to stop AdvertisementWatcher");
            }),
        };

        app_handle
            .emit(events::BLUETOOTH_ADAPTER_STATE_UPDATED, state)
            .unwrap_or_else(|e| {
                tracing::error!("Failed to emit device connected event: {}", e);
            });
    });

    let app_handle: tauri::AppHandle = app.app_handle().clone();
    state.on_device_connected(move |device| {
        tracing::info!("Device connected: {:?}", device);
        app_handle
            .emit(events::DEVICE_CONNECTED, device)
            .unwrap_or_else(|e| {
                tracing::error!("Failed to emit device connected event: {}", e);
            });
    });

    let app_handle: tauri::AppHandle = app.app_handle().clone();
    state.on_device_disconnected(move || {
        tracing::info!("Device disconnected");
        app_handle
            .emit(events::DEVICE_DISCONNECTED, "")
            .unwrap_or_else(|e| {
                tracing::error!("Failed to emit device disconnected event: {}", e);
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
    });

    if let Some(device) = find_connected_device_with_vendor_id(apple_cp::VENDOR_ID) {
        state.select_device(device);
    }

    // Store the watcher in the app state to keep it alive
    app.manage(RwLock::new(state));

    advertisement_watcher::init(app);
    let state = app.state::<RwLock<DeviceManagerState>>();
    let mut state = state.write().unwrap();

    state.adapter_watcher.start();
}
