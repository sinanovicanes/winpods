use std::sync::Mutex;

use crate::device_manager::DeviceManagerState;

#[tauri::command]
pub fn disconnect(device_manager: tauri::State<Mutex<DeviceManagerState>>) {
    tracing::info!("Disconnecting");
    device_manager.lock().unwrap().disconnect();
}
