use std::sync::RwLock;

use bluetooth::AdapterState;

use crate::device_manager::DeviceManagerState;

#[tauri::command]
pub fn is_bluetooth_adapter_active(
    device_manager_state: tauri::State<RwLock<DeviceManagerState>>,
) -> bool {
    let device_manager = device_manager_state.read().unwrap();
    let adapter_state = device_manager.adapter_watcher.state();
    match adapter_state {
        AdapterState::On => true,
        AdapterState::Off => false,
    }
}
