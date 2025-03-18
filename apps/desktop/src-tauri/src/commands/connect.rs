use std::sync::RwLock;

use device::Device;

use crate::device_manager::DeviceManagerState;

#[tauri::command]
pub fn connect(
    address: u64,
    device_manager: tauri::State<RwLock<DeviceManagerState>>,
) -> Result<(), &'static str> {
    tracing::info!("Connecting to device with address: {}", address);

    let Ok(device) = Device::new(address) else {
        tracing::error!("Failed to create device with address: {}", address);
        return Err("Failed to create device");
    };

    device_manager.write().unwrap().connect(device);

    Ok(())
}
