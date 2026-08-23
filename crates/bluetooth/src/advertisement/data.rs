use std::collections::HashMap;

use anyhow::Result;
use windows::{
    Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementReceivedEventArgs,
    Storage::Streams::DataReader,
};

/// A received BLE advertisement, reduced to the parts winpods uses.
///
/// The advertisement timestamp is deliberately dropped: nothing consumed it, and keeping it meant
/// carrying a WinRT `DateTime` into the app's own types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Advertisement {
    /// Signal strength in dBm. Used to reject readings from a second, more distant device that
    /// happens to be the same model.
    pub rssi: i16,
    pub address: u64,
    /// Manufacturer specific payloads, keyed by Bluetooth SIG company id.
    pub manufacturer_data: HashMap<u16, Vec<u8>>,
}

impl Advertisement {
    /// Returns the manufacturer payload for `company_id`, if the advertisement carried one.
    pub fn manufacturer_data_for(&self, company_id: u16) -> Option<&[u8]> {
        self.manufacturer_data.get(&company_id).map(Vec::as_slice)
    }

    /// Reads an advertisement out of its WinRT event args.
    ///
    /// When `company_filter` is set, payloads from other companies are skipped before their bytes
    /// are copied. This matters: the handler runs for every advertisement in radio range, and the
    /// app only ever looks at Apple's, so filtering here avoids an allocation per unrelated
    /// device per advertisement.
    pub(crate) fn from_args(
        args: &BluetoothLEAdvertisementReceivedEventArgs,
        company_filter: Option<u16>,
    ) -> Result<Self> {
        let rssi = args.RawSignalStrengthInDBm()?;
        let address = args.BluetoothAddress()?;
        let sections = args.Advertisement()?.ManufacturerData()?;
        let mut manufacturer_data = HashMap::new();

        for section in sections {
            let company_id = section.CompanyId()?;

            if company_filter.is_some_and(|filter| filter != company_id) {
                continue;
            }

            let buffer = section.Data()?;
            let mut bytes = vec![0u8; buffer.Length()? as usize];
            DataReader::FromBuffer(&buffer)?.ReadBytes(&mut bytes)?;
            manufacturer_data.insert(company_id, bytes);
        }

        Ok(Self {
            rssi,
            address,
            manufacturer_data,
        })
    }
}
