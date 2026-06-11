use windows::Devices::{
    Bluetooth::{BluetoothConnectionStatus, BluetoothDevice},
    Enumeration::{DeviceInformation, DeviceInformationCollection},
};

use crate::{Device, apple_cp::AppleDeviceExt};

pub fn get_connected_device_informations() -> windows::core::Result<DeviceInformationCollection> {
    let query = BluetoothDevice::GetDeviceSelectorFromConnectionStatus(
        BluetoothConnectionStatus::Connected,
    )?;
    let devices = DeviceInformation::FindAllAsyncAqsFilter(&query)?.get()?;

    Ok(devices)
}

pub fn get_connected_device_list() -> Vec<Device> {
    let connected_devices = get_device_list_from_selector(
        BluetoothDevice::GetDeviceSelectorFromConnectionStatus(
            BluetoothConnectionStatus::Connected,
        )
        .ok(),
    );

    if connected_devices
        .iter()
        .any(|device| device.get_device_model().is_supported_audio_device())
    {
        return connected_devices;
    }

    get_device_list_from_selector(BluetoothDevice::GetDeviceSelector().ok())
}

fn get_device_list_from_selector(aqsfilter: Option<windows::core::HSTRING>) -> Vec<Device> {
    let Some(aqsfilter) = aqsfilter else {
        return vec![];
    };

    let Ok(devices) = DeviceInformation::FindAllAsyncAqsFilter(&aqsfilter) else {
        return vec![];
    };

    let Ok(devices) = devices.get() else {
        return vec![];
    };

    devices
        .into_iter()
        .filter_map(|device| {
            let device = Device::try_from(device).ok()?;
            Some(device)
        })
        .collect()
}

pub fn find_connected_device_with_vendor_id(vendor_id: u16) -> Option<Device> {
    let devices = get_connected_device_list();
    let device = devices.iter().find(|device| {
        device.get_vendor_id() == Ok(vendor_id)
            && device.get_device_model().is_supported_audio_device()
    })?;

    Some(device.clone())
}
