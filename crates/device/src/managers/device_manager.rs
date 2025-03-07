use crate::models::Airpods;
use std::collections::HashMap;
use windows::Devices::{Bluetooth::BluetoothDevice, Enumeration::DeviceInformation};

#[derive(Debug, Clone, Default)]
pub struct DeviceManager {
    devices: HashMap<u64, Airpods>,
}

impl DeviceManager {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
        }
    }

    pub fn add_device(&mut self, device: Airpods) {
        self.devices.insert(device.address.clone(), device);
    }

    pub fn remove_device(&mut self, address: &u64) {
        self.devices.remove(address);
    }

    pub fn get_device(&self, address: &u64) -> Option<&Airpods> {
        self.devices.get(address)
    }

    pub fn size(&self) -> usize {
        self.devices.len()
    }

    pub fn scan(&mut self) -> Result<(), &'static str> {
        // Scan for devices

        let query =
            BluetoothDevice::GetDeviceSelector().map_err(|_| "Failed to create selector")?;

        let devices = DeviceInformation::FindAllAsyncAqsFilter(&query)
            .map_err(|_| "Failed to find devices")?
            .get()
            .map_err(|_| "Failed to get devices")?;

        for device_info in &devices {
            let airpods = Airpods::try_from(device_info);

            if let Ok(airpods) = airpods {
                self.add_device(airpods);
            }
        }

        Ok(())
    }
}
