use bluetooth::{
    Device, DeviceConnectionState,
    apple_cp::{AppleDeviceExt, AppleDeviceModel},
};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DeviceInfo {
    pub address: u64,
    pub name: String,
    pub connection_state: DeviceConnectionState,
    pub model: AppleDeviceModel,
}

impl From<&Device> for DeviceInfo {
    fn from(device: &Device) -> Self {
        let address = device.get_address().unwrap_or(0);
        let name = device.get_name().unwrap_or_else(|_| "Unknown".to_string());
        let connection_state = device.get_connection_state();
        let model = device.get_device_model();

        DeviceInfo {
            address,
            name,
            connection_state,
            model,
        }
    }
}
