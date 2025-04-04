use std::sync::RwLock;

use crate::device_manager::DeviceManagerState;

#[tauri::command]
pub fn disconnect(device_manager: tauri::State<RwLock<DeviceManagerState>>) {
    tracing::info!("Disconnecting");
    device_manager.write().unwrap().disconnect();
}
