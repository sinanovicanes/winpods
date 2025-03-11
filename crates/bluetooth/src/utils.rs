use windows::Devices::Bluetooth::BluetoothDevice;

pub fn get_device_name_by_address(address: u64) -> windows::core::Result<String> {
    let device: BluetoothDevice = BluetoothDevice::FromBluetoothAddressAsync(address)?.get()?;
    let device_name: String = device.DeviceInformation()?.Name()?.to_string();

    Ok(device_name)
}
