use crate::utils::get_model_id_from_device;
use windows::Devices::Bluetooth::BluetoothDevice;

#[derive(Debug, Clone)]
pub enum AirpodsModel {
    Airpods1,
    Airpods2,
    Airpods3,
    AirpodsPro,
    AirpodsPro2,
    AirpodsPro2Usbc,
    AirpodsMax,
}

impl AirpodsModel {
    pub fn get_name(&self) -> String {
        match self {
            AirpodsModel::Airpods1 => "Airpods 1".to_string(),
            AirpodsModel::Airpods2 => "Airpods 2".to_string(),
            AirpodsModel::Airpods3 => "Airpods 3".to_string(),
            AirpodsModel::AirpodsPro => "Airpods Pro".to_string(),
            AirpodsModel::AirpodsPro2 => "Airpods Pro 2".to_string(),
            AirpodsModel::AirpodsPro2Usbc => "Airpods Pro 2 (USB-C)".to_string(),
            AirpodsModel::AirpodsMax => "Airpods Max".to_string(),
        }
    }

    pub fn get_model_from_id(model_id: u16) -> Option<Self> {
        match model_id {
            0x2002 => Some(AirpodsModel::Airpods1),
            0x200F => Some(AirpodsModel::Airpods2),
            0x2013 => Some(AirpodsModel::Airpods3),
            0x200E => Some(AirpodsModel::AirpodsPro),
            0x2014 => Some(AirpodsModel::AirpodsPro2),
            0x2024 => Some(AirpodsModel::AirpodsPro2Usbc),
            0x200A => Some(AirpodsModel::AirpodsMax),
            _ => None,
        }
    }
}

impl TryFrom<BluetoothDevice> for AirpodsModel {
    type Error = &'static str;

    fn try_from(device: BluetoothDevice) -> std::result::Result<Self, Self::Error> {
        let model_id = get_model_id_from_device(&device).ok_or("Model id not found")?;
        AirpodsModel::get_model_from_id(model_id).ok_or("Unknown model")
    }
}
