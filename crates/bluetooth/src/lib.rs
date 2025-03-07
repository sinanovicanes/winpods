use windows::Devices::{
    Bluetooth::{BluetoothConnectionStatus, BluetoothDevice},
    Enumeration::{DeviceInformation, DeviceInformationCollection},
};

pub async fn find_airpods() -> Option<BluetoothDevice> {
    let query = BluetoothDevice::GetDeviceSelector().ok()?; // Get query string for Bluetooth LE devices
    let devices: DeviceInformationCollection = DeviceInformation::FindAllAsyncAqsFilter(&query)
        .ok()?
        .get()
        .ok()?;

    for device in &devices {
        if let Ok(name) = device.Name() {
            if name.to_string_lossy().contains("AirPods") {
                let device = BluetoothDevice::FromIdAsync(&device.Id().ok()?)
                    .ok()?
                    .get()
                    .ok()?;
                let connection_status = device.ConnectionStatus().ok()?;

                if matches!(connection_status, BluetoothConnectionStatus::Connected) {
                    return Some(device);
                }
            }
        }
    }

    None
}

pub async fn get_bluetooth_device() -> Result<BluetoothDevice, &'static str> {
    let device = find_airpods().await.ok_or("No airpods found")?;

    Ok(device)
}
