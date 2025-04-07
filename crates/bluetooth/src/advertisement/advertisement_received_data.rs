use std::collections::HashMap;
use windows::{
    Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementReceivedEventArgs,
    Foundation::DateTime, Storage::Streams::DataReader,
};

use crate::{Error, Result};

#[derive(Debug, Clone)]
pub struct AdvertisementReceivedData {
    pub rssi: i16,
    pub timestamp: DateTime,
    pub address: u64,
    pub manufacturer_data_map: HashMap<u16, Vec<u8>>,
}

impl TryFrom<BluetoothLEAdvertisementReceivedEventArgs> for AdvertisementReceivedData {
    type Error = Error;

    fn try_from(args: BluetoothLEAdvertisementReceivedEventArgs) -> Result<Self> {
        let rssi = args
            .RawSignalStrengthInDBm()
            .map_err(|_| Error::WindowsError)?;
        let timestamp = args.Timestamp().map_err(|_| Error::WindowsError)?;
        let address = args.BluetoothAddress().map_err(|_| Error::WindowsError)?;
        let mut manufacturer_data_map = HashMap::new();
        let manufacturer_data_vector = args
            .Advertisement()
            .map_err(|_| Error::WindowsError)?
            .ManufacturerData()
            .map_err(|_| Error::WindowsError)?;

        for i in 0..manufacturer_data_vector
            .Size()
            .map_err(|_| Error::WindowsError)?
        {
            let manufacturer_data = manufacturer_data_vector
                .GetAt(i)
                .map_err(|_| Error::WindowsError)?;
            let company_id = manufacturer_data
                .CompanyId()
                .map_err(|_| Error::WindowsError)?;
            let data = manufacturer_data.Data().map_err(|_| Error::WindowsError)?;
            let buffer_length = data.Length().map_err(|_| Error::WindowsError)?;
            let mut buffer = vec![0; buffer_length as usize];
            let reader = DataReader::FromBuffer(&data).map_err(|_| Error::WindowsError)?;
            reader
                .ReadBytes(&mut buffer)
                .map_err(|_| Error::WindowsError)?;

            manufacturer_data_map.insert(company_id, buffer);
        }

        let data = AdvertisementReceivedData {
            rssi,
            timestamp,
            address,
            manufacturer_data_map,
        };

        Ok(data)
    }
}
