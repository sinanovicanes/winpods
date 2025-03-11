use device::apple_cp::AirPodsModel;
use serde::Serialize;

use super::Battery;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectedDevice {
    name: String,
    model: AirPodsModel,
    battery_right: Battery,
    battery_left: Battery,
}

impl ConnectedDevice {
    pub fn new(
        name: String,
        model: AirPodsModel,
        battery_right: Battery,
        battery_left: Battery,
    ) -> Self {
        Self {
            name,
            model,
            battery_right,
            battery_left,
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
