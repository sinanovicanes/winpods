use bluetooth::{
    apple_cp::{AppleDeviceModel, ProximityPairingMessage},
    AdvertisementReceivedData,
};
use serde::{Deserialize, Serialize};

use crate::{models::Battery, tray::Tooltip};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceProperties {
    pub rssi: i16,
    pub address: u64,
    pub model: AppleDeviceModel,
    pub left_battery: Battery,
    pub right_battery: Battery,
    pub case_battery: Option<Battery>,
    pub left_in_ear: bool,
    pub right_in_ear: bool,
}

impl DeviceProperties {
    pub fn from_advertisement(
        data: &AdvertisementReceivedData,
        protocol: &ProximityPairingMessage,
    ) -> Self {
        let model = protocol.get_model();
        let right_battery = Battery::new(
            protocol.get_right_battery().unwrap_or(0) * 10,
            protocol.is_right_charging(),
        );
        let left_battery = Battery::new(
            protocol.get_left_battery().unwrap_or(0) * 10,
            protocol.is_left_charging(),
        );
        let case_battery = protocol
            .get_case_battery()
            .map(|val| Battery::new(val * 10, protocol.is_case_charging()));
        let left_in_ear = protocol.is_left_in_ear();
        let right_in_ear = protocol.is_right_in_ear();

        Self {
            rssi: data.rssi,
            address: data.address,
            model,
            right_battery,
            left_battery,
            case_battery,
            left_in_ear,
            right_in_ear,
        }
    }

    pub fn is_within_update_limits(&self, other: &DeviceProperties) -> bool {
        const RSSI_UPDATE_LIMIT: u16 = 50;
        const BATTERY_UPDATE_LIMIT: u8 = 20;

        if self.model != other.model {
            return false;
        }

        let rssi_diff = self.rssi.abs_diff(other.rssi);

        if rssi_diff > RSSI_UPDATE_LIMIT {
            return false;
        }

        let battery_diff = self.left_battery.level.abs_diff(other.left_battery.level);

        if battery_diff > BATTERY_UPDATE_LIMIT {
            return false;
        }

        let battery_diff = self.right_battery.level.abs_diff(other.right_battery.level);

        if battery_diff > BATTERY_UPDATE_LIMIT {
            return false;
        }

        true
    }
}

impl Tooltip for DeviceProperties {
    fn to_tooltip(&self) -> String {
        let mut tooltip = format!(
            "Left: {}% {}{}\nRight: {}% {}{}\n",
            self.left_battery.level,
            if self.left_battery.charging {
                "⚡"
            } else {
                ""
            },
            if self.left_in_ear { "👂" } else { "" },
            self.right_battery.level,
            if self.right_battery.charging {
                "⚡"
            } else {
                ""
            },
            if self.right_in_ear { "👂" } else { "" },
        );

        if let Some(case_battery) = &self.case_battery {
            tooltip.push_str(&format!(
                "Case: {}% {}\n",
                case_battery.level,
                if case_battery.charging { "⚡" } else { "" }
            ));
        }

        tooltip
    }
}
