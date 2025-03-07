use windows::{
    Devices::{
        Bluetooth::{BluetoothDevice, BluetoothMajorClass, BluetoothMinorClass},
        Enumeration::DeviceInformation,
    },
    Foundation::IPropertyValue,
    core::{HSTRING, Interface},
};

pub fn is_headphones(device: &BluetoothDevice) -> bool {
    if let Ok(class_of_device) = device.ClassOfDevice() {
        let major_class = class_of_device
            .MajorClass()
            .unwrap_or(BluetoothMajorClass::Miscellaneous);
        let minor_class = class_of_device
            .MinorClass()
            .unwrap_or(BluetoothMinorClass::Uncategorized);

        return matches!(major_class, BluetoothMajorClass::AudioVideo)
            && matches!(minor_class, BluetoothMinorClass::AudioVideoHeadphones);
    }
    false
}

pub fn get_model_id_from_device(device: &BluetoothDevice) -> Option<u16> {
    let model_key = HSTRING::from("System.DeviceInterface.Bluetooth.ProductId");
    let device_id = device.DeviceId().ok()?;
    let properties = vec![model_key.clone()];
    let properties = windows_collections::IIterable::from(properties);
    let device_info =
        DeviceInformation::CreateFromIdAsyncAdditionalProperties(&device_id, &properties)
            .ok()?
            .get()
            .ok()?;
    let properties = device_info.Properties().unwrap();
    let model_id: u16 = properties
        .Lookup(&model_key)
        .ok()?
        .cast::<IPropertyValue>()
        .ok()?
        .GetUInt16()
        .ok()?;

    Some(model_id)
}
