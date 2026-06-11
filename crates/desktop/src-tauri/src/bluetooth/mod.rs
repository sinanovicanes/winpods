use std::sync::RwLock;

use bluetooth::{
    AdapterState, AdapterWatcher, AdvertisementWatcher,
    apple_cp::{self, AppleDeviceExt},
};
use tauri::{App, Emitter, Manager};

use crate::{
    device_manager::{DeviceManagerState, DeviceProperties},
    events,
};

pub struct BluetoothState {
    pub adapter_watcher: AdapterWatcher,
    pub adv_watcher: AdvertisementWatcher,
}

impl BluetoothState {
    pub fn new() -> Self {
        let adapter_watcher = AdapterWatcher::new();
        let adv_watcher =
            AdvertisementWatcher::new().expect("Failed to create AdvertisementWatcher");

        BluetoothState {
            adapter_watcher,
            adv_watcher,
        }
    }
}

pub fn init(app: &mut App) {
    let mut state = BluetoothState::new();

    let app_handle = app.app_handle().clone();
    state.adapter_watcher.on_state_changed(move |state| {
        tracing::info!("Bluetooth adapter state changed: {:?}", state);

        // Toggle the advertisement watcher based on the adapter state
        {
            let bluetooth_state = app_handle.state::<RwLock<BluetoothState>>();
            let bluetooth_state = bluetooth_state.read().unwrap();

            match state {
                AdapterState::On => bluetooth_state.adv_watcher.start().unwrap_or_else(|_| {
                    tracing::error!("Failed to start AdvertisementWatcher");
                }),
                AdapterState::Off => bluetooth_state.adv_watcher.stop().unwrap_or_else(|_| {
                    tracing::error!("Failed to stop AdvertisementWatcher");
                }),
            };
        }

        // Emit the adapter state updated event after device manager lock is released
        app_handle
            .emit(events::BLUETOOTH_ADAPTER_STATE_UPDATED, state)
            .unwrap_or_else(|e| {
                tracing::error!("Failed to emit device connected event: {}", e);
            });
    });

    let app_handle = app.app_handle().clone();
    state.adv_watcher.on_received(move |data| {
        // We could have just emit the adv recieved event and handle it on the device manager side,
        // but we don't need to serialize and deserialize the data again, so we handle it here directly
        // and emit the device properties updated event if needed.

        let Some(apple_data) = data.manufacturer_data_map.get(&apple_cp::VENDOR_ID) else {
            // tracing::info!("No Apple data found in received advertisement");
            return;
        };

        let Some(protocol) = apple_cp::proximity_pairing_message_from_bytes(apple_data) else {
            // tracing::info!("Received Apple data is not valid proximity pairing message");
            return;
        };

        let device_manager_lock: tauri::State<RwLock<DeviceManagerState>> =
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

    state.adv_watcher.on_stopped(move || {
        tracing::info!("AdvertisementWatcher stopped");
    });

    state.adapter_watcher.start();

    // Only start the advertisement watcher if the adapter is on
    if matches!(state.adapter_watcher.state(), AdapterState::On) {
        state.adv_watcher.start().unwrap_or_else(|_| {
            tracing::error!("Failed to start AdvertisementWatcher");
        });
    }

    app.manage(RwLock::new(state));
}
