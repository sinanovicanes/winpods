use bluetooth::{
    apple_cp::{ProximityPairingMessage, ProximityPairingModel},
    AdvertisementReceivedData,
};
use device::Device;
use serde::{ser::SerializeStruct, Deserialize, Serialize};

use crate::models::Battery;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectedDeviceProperties {
    pub rssi: i16,
    pub address: u64,
    pub model: ProximityPairingModel,
    pub left_battery: Battery,
    pub right_battery: Battery,
    pub case_battery: Option<Battery>,
    pub left_in_ear: bool,
    pub right_in_ear: bool,
}

#[derive(Debug, Clone)]
pub struct SelectedDevice {
    pub device: Device,
    pub properties: Option<SelectedDeviceProperties>,
}

impl SelectedDevice {
    pub fn new(device: Device) -> Self {
        Self {
            device,
            properties: None,
        }
    }

    fn update_properties(&mut self, properties: SelectedDeviceProperties) -> bool {
        const RSSI_UPDATE_LIMIT: u16 = 50;
        const BATTERY_UPDATE_LIMIT: u8 = 20;

        if let Some(ref old_properties) = self.properties {
            let Ok(product_id) = self.device.get_product_id() else {
                return false;
            };

            if ProximityPairingModel::from(product_id) != properties.model {
                return false;
            }

            if old_properties == &properties {
                return false;
            }

            let rssi_diff = old_properties.rssi.abs_diff(properties.rssi);

            if rssi_diff > RSSI_UPDATE_LIMIT {
                return false;
            }

            let battery_diff = old_properties
                .left_battery
                .level
                .abs_diff(properties.left_battery.level);

            if battery_diff > BATTERY_UPDATE_LIMIT {
                return false;
            }

            let battery_diff = old_properties
                .right_battery
                .level
                .abs_diff(properties.right_battery.level);

            if battery_diff > BATTERY_UPDATE_LIMIT {
                return false;
            }
        }

        self.properties = Some(properties);

        true
    }

    pub fn on_advertisement_received(
        &mut self,
        data: &AdvertisementReceivedData,
        protocol: &ProximityPairingMessage,
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
        let left_in_ear = protocol.is_left_in_ear();
        let right_in_ear = protocol.is_right_in_ear();

        self.update_properties(SelectedDeviceProperties {
            rssi: data.rssi,
            address: data.address,
            model,
            right_battery,
            left_battery,
            case_battery,
            left_in_ear,
            right_in_ear,
        })
    }
}

impl Serialize for SelectedDevice {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("SelectedDevice", 2)?;
        state.serialize_field(
            "name",
            &self
                .device
                .get_name()
                .unwrap_or("Connected Device".to_string()),
        )?;
        state.serialize_field("address", &self.device.get_address().unwrap_or(0))?;
        state.serialize_field("properties", &self.properties)?;
        state.end()
    }
}
