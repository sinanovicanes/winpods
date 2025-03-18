use device::DeviceConnectionState;
use windows::Devices::{
    Bluetooth::{BluetoothConnectionStatus, BluetoothDevice},
    Enumeration::{DeviceInformation, DeviceInformationCollection},
};

pub fn get_connected_device_informations() -> windows::core::Result<DeviceInformationCollection> {
    let query = BluetoothDevice::GetDeviceSelectorFromConnectionStatus(
        BluetoothConnectionStatus::Connected,
    )?;
    let devices = DeviceInformation::FindAllAsyncAqsFilter(&query)?.get()?;

    Ok(devices)
}

pub fn get_connected_device_list() -> Vec<device::Device> {
    let Ok(aqsfilter) = BluetoothDevice::GetDeviceSelectorFromConnectionStatus(
        BluetoothConnectionStatus::Connected,
    ) else {
        return vec![];
    };

    let Ok(devices) = DeviceInformation::FindAllAsyncAqsFilter(&aqsfilter) else {
        return vec![];
    };

    let Ok(devices) = devices.get() else {
        return vec![];
    };

    let devices = devices.into_iter().filter_map(|device| {
        let device = device::Device::try_from(device).ok()?;
        Some(device)
    });

    devices.collect()
}

pub fn find_connected_device_with_vendor_id(vendor_id: u16) -> Option<device::Device> {
    let devices = get_connected_device_list();
    let device = devices.iter().find(|device| {
        device.get_vendor_id() == Ok(vendor_id)
            && device.get_connection_state() == DeviceConnectionState::Connected
    })?;

    Some(device.clone())
}
