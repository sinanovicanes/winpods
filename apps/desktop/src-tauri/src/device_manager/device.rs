use bluetooth::{
    apple_cp::{AirPodsModel, AppleCP},
    AdvertisementReceivedData,
};
use serde::{Deserialize, Serialize};

use crate::models::Battery;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceProperties {
    pub rssi: i16,
    pub address: u64,
    pub model: AirPodsModel,
    pub left_battery: Battery,
    pub right_battery: Battery,
    pub case_battery: Option<Battery>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub address: u64,
    pub name: String,
    pub properties: Option<DeviceProperties>,
}

impl Device {
    fn update_properties(&mut self, properties: DeviceProperties) -> bool {
        if let Some(ref old_properties) = self.properties {
            if old_properties == &properties {
                return false;
            }
        }

        self.properties = Some(properties);

        return true;
    }

    pub fn on_advertisement_received(
        &mut self,
        data: &AdvertisementReceivedData,
        protocol: &AppleCP,
    ) -> bool {
        let model = protocol.get_model();
        let right_battery = Battery::new(
            protocol.get_right_battery().unwrap_or(0) * 10,
            protocol.is_right_charging(),
        );
        let left_battery = Battery::new(
            protocol.get_left_battery().unwrap_or(0) * 10,
            protocol.is_left_charging(),
        );
        let case_battery = protocol.get_case_battery().map(|val| Battery {
            level: val * 10,
            charging: protocol.is_case_charging(),
        });

        self.update_properties(DeviceProperties {
            rssi: data.rssi,
            address: data.address,
            model,
            right_battery,
            left_battery,
            case_battery,
        })
    }
}
