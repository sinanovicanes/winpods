use windows::Devices::{
    Bluetooth::{BluetoothConnectionStatus, BluetoothDevice},
    Enumeration::DeviceInformation,
};

pub fn find_connected_device() -> Option<BluetoothDevice> {
    let query = BluetoothDevice::GetDeviceSelectorFromConnectionStatus(
        BluetoothConnectionStatus::Connected,
    )
    .ok()?;
    let devices = DeviceInformation::FindAllAsyncAqsFilter(&query)
        .ok()?
        .get()
        .ok()?;

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
