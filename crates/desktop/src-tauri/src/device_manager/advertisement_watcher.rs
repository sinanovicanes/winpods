use super::{DeviceManagerState, DeviceProperties};
use crate::events;
use bluetooth::{
    AdapterState,
    apple_cp::{self, AppleDeviceExt},
    get_adapter_state,
};
use std::sync::RwLock;
use tauri::{App, Emitter, Manager};

pub fn init(app: &mut App) {
    let state = app.state::<RwLock<DeviceManagerState>>();
    let state = state.read().unwrap();

    // Get app_handle for the callback
    let app_handle = app.app_handle().clone();
    state.adv_watcher.on_received(move |data| {
        let Some(apple_data) = data.manufacturer_data_map.get(&apple_cp::VENDOR_ID) else {
            // tracing::info!("No Apple data found in received advertisement");
            return;
        };

        let Some(protocol) = apple_cp::proximity_pairing_message_from_bytes(apple_data) else {
            // tracing::info!("Received Apple data is not valid proximity pairing message");
            return;
        };

        let device_manager_lock: tauri::State<'_, RwLock<DeviceManagerState>> =
            app_handle.state::<RwLock<DeviceManagerState>>();
        let mut device_manager = device_manager_lock.write().unwrap();

        let Some(device) = &device_manager.device else {
            // tracing::info!("No device selected, ignoring advertisement");
            return;
        };

        let properties = DeviceProperties::from_advertisement(data, &protocol);

        if device.get_device_model() != properties.model {
            // tracing::info!(
            //     "Received advertisement for a different device model: {:?}",
            //     properties.model
            // );
            return;
        }

        if let Some(device_properties) = &device_manager.device_properties {
            if !device_properties.is_within_update_limits(&properties) {
                // tracing::info!(
                //     "Received advertisement with properties outside update limits: {:?}",
                //     properties
                // );
                return;
            }
        }

        device_manager.device_properties = Some(properties.clone());

        // Drop write lock before emitting the event
        // This is necessary because the event handlers may need to read the device manager state
        // and we cannot have a write lock while doing that
        drop(device_manager);
        app_handle
            .emit(events::DEVICE_PROPERTIES_UPDATED, properties)
            .unwrap_or_else(|e| {
                tracing::error!("Failed to emit device updated event: {}", e);
            });
    });

    let adapter_state = get_adapter_state();

    if adapter_state == AdapterState::On {
        state.adv_watcher.start().unwrap_or_else(|_| {
            tracing::error!("Failed to start AdvertisementWatcher");
        });
    }
}
