use crate::utils::is_headphones;
use windows::{
    Devices::{
        Bluetooth::{BluetoothConnectionStatus, BluetoothDevice},
        Enumeration::DeviceInformation,
    },
    core::HSTRING,
};

use super::AirpodsModel;

#[derive(Debug, Clone)]
pub struct Airpods {
    pub address: u64,
    pub name: String,
    pub battery: u8,
    pub is_connected: bool,
    pub model: AirpodsModel,
}

impl Airpods {}

impl TryFrom<BluetoothDevice> for Airpods {
    type Error = &'static str;

    fn try_from(device: BluetoothDevice) -> Result<Self, Self::Error> {
        if !is_headphones(&device) {
            return Err("Invalid device");
        }

        let address = device
            .BluetoothAddress()
            .map_err(|_| "Failed to get address")?;
        let name = device
            .Name()
            .unwrap_or(HSTRING::from("Unknown"))
            .to_string_lossy();
        let is_connected = matches!(
            device.ConnectionStatus().unwrap_or_default(),
            BluetoothConnectionStatus::Connected
        );
        let model = AirpodsModel::try_from(device).map_err(|_| "Invalid device")?;

        println!("Model: {:?}", model);

        Ok(Self {
            address,
            name,
            battery: 0,
            is_connected,
            model,
        })
    }
}

impl TryFrom<DeviceInformation> for Airpods {
    type Error = &'static str;

    fn try_from(device_info: DeviceInformation) -> Result<Self, Self::Error> {
        let device_id = device_info
            .Id()
            .map_err(|_| "Failed to retrieve device id")?;

        let device = BluetoothDevice::FromIdAsync(&device_id)
            .map_err(|_| "Failed to get device")?
            .get()
            .map_err(|_| "Failed to get device")?;

        Self::try_from(device)
    }
}
