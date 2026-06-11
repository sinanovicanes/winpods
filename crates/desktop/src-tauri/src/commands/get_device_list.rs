use bluetooth::apple_cp::AppleDeviceExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedDevice {
    name: String,
    address: u64,
}

#[tauri::command]
pub fn get_bluetooth_device_list() -> Vec<ConnectedDevice> {
    let devices = bluetooth::get_connected_device_list();
    tracing::info!(
        "UI requested Bluetooth device list: {} connected devices found",
        devices.len()
    );

    devices
        .iter()
        .filter_map(|device| {
            let name = match device.get_name() {
                Ok(name) => name,
                Err(error) => {
                    tracing::warn!(
                        "Skipping Bluetooth device without a readable name: {:?}",
                        error
                    );
                    return None;
                }
            };
            let address = match device.get_address() {
                Ok(address) => address,
                Err(error) => {
                    tracing::warn!(
                        "Skipping Bluetooth device without a readable address: {:?}",
                        error
                    );
                    return None;
                }
            };
            let model = device.get_device_model();

            if !model.is_supported_audio_device() {
                tracing::info!(
                    "Skipping unsupported Bluetooth dropdown device: {} ({}) model={:?}",
                    name,
                    address,
                    model
                );
                return None;
            }

            tracing::info!(
                "Bluetooth dropdown candidate: {} ({}) model={:?}",
                name,
                address,
                model
            );

            Some(ConnectedDevice {
                name: name.to_string(),
                address,
            })
        })
        .collect()
}
