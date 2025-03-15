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

pub fn get_connected_device_list() -> windows::core::Result<Vec<BluetoothDevice>> {
    let devices = get_connected_device_informations()?;
    let mut connected_devices = Vec::new();

    for i in 0..devices.Size()? {
        let device = devices.GetAt(i)?;
        let bluetooth_device = BluetoothDevice::FromIdAsync(&device.Id()?)?.get()?;
        connected_devices.push(bluetooth_device);
    }

    Ok(connected_devices)
}

pub fn find_connected_device() -> Option<BluetoothDevice> {
    let devices = get_connected_device_informations().ok()?;
    let device = devices.GetAt(0).ok()?;
    let bluetooth_device = BluetoothDevice::FromIdAsync(&device.Id().ok()?)
        .ok()?
        .get()
        .ok()?;

    Some(bluetooth_device)
}

pub fn get_device_name_by_address(address: u64) -> windows::core::Result<String> {
    let device: BluetoothDevice = BluetoothDevice::FromBluetoothAddressAsync(address)?.get()?;
    let device_name: String = device.DeviceInformation()?.Name()?.to_string();

    Ok(device_name)
}
