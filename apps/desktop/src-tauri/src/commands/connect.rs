use std::sync::Mutex;

use crate::device_manager::{Device, DeviceManagerState};

#[tauri::command]
pub fn connect(
    name: String,
    address: u64,
    device_manager: tauri::State<Mutex<DeviceManagerState>>,
) -> Result<(), &'static str> {
    tracing::info!("Connecting to device with address: {}", address);

    let new_device = Device {
        name,
        address,
        properties: None,
    };

    device_manager.lock().unwrap().connect(new_device);

    Ok(())
}
