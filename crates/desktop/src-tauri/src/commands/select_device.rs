use std::sync::RwLock;

use bluetooth::Device;
use bluetooth::apple_cp::AppleDeviceExt;

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

    let model = device.get_device_model();
    if !model.is_supported_audio_device() {
        tracing::warn!(
            "Rejected unsupported Bluetooth device selection: address={} model={:?}",
            address,
            model
        );
        return Err("Unsupported device");
    }

    device_manager.write().unwrap().select_device(device);

    Ok(())
}
