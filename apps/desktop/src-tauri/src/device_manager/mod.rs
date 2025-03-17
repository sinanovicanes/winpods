use bluetooth::{
    apple_cp::{self},
    find_connected_device,
};
use std::sync::Mutex;
use tauri::{App, Emitter, Manager};

use crate::events;

mod device;
mod device_manager;

pub use device::Device;
pub use device_manager::DeviceManagerState;

pub fn init(app: &mut App) {
    let mut state = DeviceManagerState::new();

    // Get app_handle for the callback
    let app_handle = app.app_handle().clone();
    state.adv_watcher.on_received(move |data| {
        let device_manager = app_handle.state::<Mutex<DeviceManagerState>>();
        let mut device_manager = device_manager.lock().unwrap();

        if !device_manager.is_connected() {
            tracing::info!("Got advertisement but no device connected");
            return;
        }

        let Some(apple_data) = data.manufacturer_data_map.get(&apple_cp::VENDOR_ID) else {
            tracing::info!("No Apple data found in received advertisement");
            return;
        };

        let Some(protocol) = apple_cp::proximity_pairing_message_from_bytes(apple_data) else {
            tracing::info!("Received Apple data is not valid proximity pairing message");
            return;
        };

        device_manager.on_advertisement_received(data, &protocol);
    });

    let app_handle: tauri::AppHandle = app.app_handle().clone();
    state.on_device_connected(move |device| {
        app_handle
            .emit(events::DEVICE_CONNECTED, device)
            .unwrap_or_else(|e| {
                tracing::error!("Failed to emit device connected event: {}", e);
            });
    });

    let app_handle: tauri::AppHandle = app.app_handle().clone();
    state.on_device_disconnected(move || {
        app_handle
            .emit(events::DEVICE_DISCONNECTED, "")
            .unwrap_or_else(|e| {
                tracing::error!("Failed to emit device disconnected event: {}", e);
            });
    });

    let app_handle = app.app_handle().clone();
    state.on_device_updated(move |device| {
        app_handle
            .emit(events::DEVICE_UPDATED, device)
            .unwrap_or_else(|e| {
                tracing::error!("Failed to emit device updated event: {}", e);
            });
    });

    let device = find_connected_device().map(|info| {
        let name = info
            .Name()
            .map(|name| name.to_string())
            .unwrap_or("Unknown".to_string());

        let address = info.BluetoothAddress().unwrap_or(0);

        Device::new(address, name)
    });

    if let Some(device) = device {
        state.connect(device);
    }

    // Store the watcher in the app state to keep it alive
    app.manage(Mutex::new(state));

    // Start the AdvertisementWatcher after storing it in the app state
    let state = app.state::<Mutex<DeviceManagerState>>();
    let state = state.lock().unwrap();

    state
        .adv_watcher
        .start()
        .expect("Failed to start AdvertisementWatcher");
}
