use std::sync::RwLock;

use bluetooth::Device;

use crate::device_manager::DeviceManagerState;

#[tauri::command]
pub fn select_device(
    address: u64,
    device_manager: tauri::State<RwLock<DeviceManagerState>>,
) -> Result<(), &'static str> {
    tracing::info!("Connecting to device with address: {}", address);

    let Ok(device) = Device::from_bluetooth_address(address) else {
        tracing::error!("Failed to create device with address: {}", address);
        return Err("Failed to create device");
    };

    device_manager.write().unwrap().select_device(device);

    Ok(())
}
