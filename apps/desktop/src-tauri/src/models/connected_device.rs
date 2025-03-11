use device::apple_cp::AirPodsModel;
use serde::{Deserialize, Serialize};

use super::Battery;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectedDevice {
    pub name: String,
    pub model: AirPodsModel,
    pub battery_left: Battery,
    pub battery_right: Battery,
}

impl ConnectedDevice {
    pub fn new(
        name: String,
        model: AirPodsModel,
        battery_left: Battery,
        battery_right: Battery,
    ) -> Self {
        Self {
            name,
            model,
            battery_left,
            battery_right,
        }
    }
}

impl Default for ConnectedDevice {
    fn default() -> Self {
        Self {
            name: String::new(),
            model: AirPodsModel::Unknown,
            battery_right: Battery::default(),
            battery_left: Battery::default(),
        }
    }
}
