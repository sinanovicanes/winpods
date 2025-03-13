use serde::{Deserialize, Serialize};
use windows::{
    Devices::Enumeration::{DeviceInformation, DeviceInformationUpdate},
    core::HSTRING,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub name: String,
}

impl Device {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }

    pub fn try_from_id(id: String) -> windows::core::Result<Self> {
        let device_id = HSTRING::from(id.clone());
        let device =
            windows::Devices::Enumeration::DeviceInformation::CreateFromIdAsync(&device_id)?
                .get()?;
        let name = device.Name()?.to_string_lossy();

        Ok(Self { id, name })
    }
}

impl TryFrom<DeviceInformation> for Device {
    type Error = windows::core::Error;

    fn try_from(device_info: DeviceInformation) -> windows::core::Result<Self> {
        let id = device_info.Id()?.to_string_lossy();
        let name = device_info.Name()?.to_string_lossy();

        Ok(Self { id, name })
    }
}

impl TryFrom<DeviceInformationUpdate> for Device {
    type Error = windows::core::Error;

    fn try_from(device_info: DeviceInformationUpdate) -> windows::core::Result<Self> {
        let id = device_info.Id()?.to_string_lossy();

        Self::try_from_id(id)
    }
}
