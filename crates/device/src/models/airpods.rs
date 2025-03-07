use crate::utils::is_headphones;
use windows::{Devices::Bluetooth::BluetoothDevice, core::HSTRING};

use super::AirpodsModel;

#[derive(Debug, Clone)]
pub struct Airpods {
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

        let name = device
            .Name()
            .unwrap_or(HSTRING::from("Unknown"))
            .to_string_lossy();
        let model = AirpodsModel::try_from(device).map_err(|_| "Invalid device")?;

        println!("Model: {:?}", model);

        Ok(Self {
            name,
            battery: 0,
            is_connected: false,
            model,
        })
    }
}
