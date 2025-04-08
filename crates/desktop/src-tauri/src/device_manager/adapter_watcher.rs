use super::DeviceManagerState;
use crate::events;
use bluetooth::AdapterState;
use std::sync::RwLock;
use tauri::{App, Emitter, Manager};

pub fn init(app: &mut App) {
    let state = app.state::<RwLock<DeviceManagerState>>();
    let mut state = state.write().unwrap();

    let app_handle = app.app_handle().clone();
    state.adapter_watcher.on_state_changed(move |state| {
        tracing::info!("Bluetooth adapter state changed: {:?}", state);

        // Toggle the advertisement watcher based on the adapter state
        {
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
        }

        // Emit the adapter state updated event after device manager lock is released
        app_handle
            .emit(events::BLUETOOTH_ADAPTER_STATE_UPDATED, state)
            .unwrap_or_else(|e| {
                tracing::error!("Failed to emit device connected event: {}", e);
            });
    });

    state.adapter_watcher.start();
}
