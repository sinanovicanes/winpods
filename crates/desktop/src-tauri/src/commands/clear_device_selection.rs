use std::sync::RwLock;

use crate::device_manager::DeviceManagerState;

#[tauri::command]
pub fn clear_device_selection(device_manager: tauri::State<RwLock<DeviceManagerState>>) {
    tracing::info!("Clearing device selection");
    device_manager.write().unwrap().clear_device_selection();
}
