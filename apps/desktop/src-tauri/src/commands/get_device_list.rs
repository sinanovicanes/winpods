use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedDevice {
    name: String,
    address: u64,
}

#[tauri::command]
pub fn get_bluetooth_device_list() -> Vec<ConnectedDevice> {
    let devices = bluetooth::get_connected_device_list().unwrap_or_default();

    devices
        .iter()
        .filter_map(|device| {
            let name = device.Name().ok()?;
            let address = device.BluetoothAddress().ok()?;
            Some(ConnectedDevice {
                name: name.to_string(),
                address,
            })
        })
        .collect()
}
