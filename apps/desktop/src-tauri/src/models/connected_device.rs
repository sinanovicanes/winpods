use device::apple_cp::AirPodsModel;
use serde::{Deserialize, Serialize};

use super::Battery;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectedDevice {
    pub name: String,
    pub model: AirPodsModel,
    pub left_battery: Battery,
    pub right_battery: Battery,
    pub case_battery: Option<Battery>,
}

impl ConnectedDevice {
    pub fn new(
        name: String,
        model: AirPodsModel,
        left_battery: Battery,
        right_battery: Battery,
        case_battery: Option<Battery>,
    ) -> Self {
        Self {
            name,
            model,
            left_battery,
            right_battery,
            case_battery,
        }
    }
}

impl Default for ConnectedDevice {
    fn default() -> Self {
        Self {
            name: String::new(),
            model: AirPodsModel::Unknown,
            right_battery: Battery::default(),
            left_battery: Battery::default(),
            case_battery: None,
        }
    }
}
