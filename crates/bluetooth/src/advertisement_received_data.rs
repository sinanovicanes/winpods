use std::collections::HashMap;

use windows::{
    Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementReceivedEventArgs,
    Foundation::DateTime, Storage::Streams::DataReader,
};

use crate::AirPods;

#[derive(Debug, Clone)]
pub struct AdvirtesementReceivedData {
    pub rssi: i16,
    pub timestamp: DateTime,
    pub address: u64,
    pub manufacturer_data_map: HashMap<u16, Vec<u8>>,
}

impl TryFrom<BluetoothLEAdvertisementReceivedEventArgs> for AdvirtesementReceivedData {
    type Error = windows::core::Error;

    fn try_from(
        args: BluetoothLEAdvertisementReceivedEventArgs,
    ) -> std::result::Result<Self, Self::Error> {
        let rssi = args.RawSignalStrengthInDBm()?;
        let timestamp = args.Timestamp()?;
        let address = args.BluetoothAddress()?;
        let mut manufacturer_data_map = HashMap::new();

        let manufacturer_data_vector = args.Advertisement()?.ManufacturerData()?;

        for i in 0..manufacturer_data_vector.Size()? {
            let manufacturer_data = manufacturer_data_vector.GetAt(i)?;
            let company_id = manufacturer_data.CompanyId()?;
            let data = manufacturer_data.Data()?;
            let buffer_length = data.Length()?;
            let mut buffer = vec![0; buffer_length as usize];
            let reader = DataReader::FromBuffer(&data)?;
            reader.ReadBytes(&mut buffer)?;

            manufacturer_data_map.insert(company_id, buffer);
        }

        let data = AdvirtesementReceivedData {
            rssi,
            timestamp,
            address,
            manufacturer_data_map,
        };

        Ok(data)
    }
}
