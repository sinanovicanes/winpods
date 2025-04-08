use std::sync::RwLock;

use bluetooth::AdapterState;

use crate::bluetooth::BluetoothState;

#[tauri::command]
pub fn is_bluetooth_adapter_active(bluetooth_state: tauri::State<RwLock<BluetoothState>>) -> bool {
    let bluetooth_state = bluetooth_state.read().unwrap();

    match bluetooth_state.adapter_watcher.state() {
        AdapterState::On => true,
        AdapterState::Off => false,
    }
}
