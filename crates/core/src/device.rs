use serde::{Deserialize, Serialize};
use winpods_apple_cp::{AppleDeviceModel, ProximityPairingMessage};

use crate::{Battery, ConnectionState};

/// Identity of the currently selected device, as shown in the UI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    pub address: u64,
    pub name: String,
    pub connection_state: ConnectionState,
    pub model: AppleDeviceModel,
}

/// Live readings decoded from a device's most recent advertisement.
///
/// Every battery is optional. A bud sitting in a closed case reports no level at all, which is
/// different from reporting 0%.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceProperties {
    /// Signal strength of the advertisement these readings came from, in dBm.
    pub rssi: i16,
    pub address: u64,
    pub model: AppleDeviceModel,
    pub left_battery: Option<Battery>,
    pub right_battery: Option<Battery>,
    pub case_battery: Option<Battery>,
    pub left_in_ear: bool,
    pub right_in_ear: bool,
}

impl DeviceProperties {
    /// Builds the readings from a decoded proximity pairing advertisement.
    ///
    /// Takes `rssi` and `address` as plain values rather than the advertisement itself, which is
    /// what keeps this crate free of any Windows types.
    pub fn from_advertisement(rssi: i16, address: u64, message: &ProximityPairingMessage) -> Self {
        let model = message.model();

        Self {
            rssi,
            address,
            model,
            left_battery: message
                .left_battery()
                .map(|level| Battery::new(level, message.is_left_charging())),
            right_battery: message
                .right_battery()
                .map(|level| Battery::new(level, message.is_right_charging())),
            // Over-ear models have no case, but still put something in the case nibble. Reporting
            // it would draw a case that does not exist.
            case_battery: model
                .has_case()
                .then(|| {
                    message
                        .case_battery()
                        .map(|level| Battery::new(level, message.is_case_charging()))
                })
                .flatten(),
            left_in_ear: message.is_left_in_ear(),
            right_in_ear: message.is_right_in_ear(),
        }
    }

    /// The level shown as *the* battery of the device: the lower of the two buds.
    ///
    /// Falls back to whichever bud reported a level when only one did, and to `None` when neither
    /// did. The previous version encoded "no reading" as 0 and so returned the *other* bud's level
    /// whenever one bud was genuinely empty.
    pub fn overall_level(&self) -> Option<u8> {
        match (self.left_battery, self.right_battery) {
            (Some(left), Some(right)) => Some(left.level.min(right.level)),
            (Some(only), None) | (None, Some(only)) => Some(only.level),
            (None, None) => None,
        }
    }

    /// Whether the device as a whole counts as charging.
    ///
    /// Both buds have to be charging when both report, so a single bud in the case does not make
    /// the widget claim the device is charging.
    pub fn is_charging(&self) -> bool {
        match (self.left_battery, self.right_battery) {
            (Some(left), Some(right)) => left.charging && right.charging,
            (Some(only), None) | (None, Some(only)) => only.charging,
            (None, None) => false,
        }
    }

    /// Whether either bud is at or below `threshold` while not charging.
    pub fn has_low_battery(&self, threshold: u8) -> bool {
        [self.left_battery, self.right_battery]
            .into_iter()
            .flatten()
            .any(|battery| battery.is_low(threshold))
    }

    /// Whether `next` plausibly describes the same physical device as `self`.
    ///
    /// Several devices of the same model can be in radio range -- a colleague's AirPods, or the
    /// user's second pair -- and they all advertise identically. A reading that jumps too far in
    /// signal strength or battery level almost certainly came from a different pair, so it is
    /// dropped rather than shown.
    pub fn is_plausible_update(&self, next: &Self) -> bool {
        const RSSI_LIMIT: u16 = 50;
        const BATTERY_LIMIT: u8 = 20;

        if self.model != next.model {
            return false;
        }

        if self.rssi.abs_diff(next.rssi) > RSSI_LIMIT {
            return false;
        }

        // A battery that was not reported before or is not reported now carries no information
        // about whether this is the same device, so it cannot rule the update out.
        let within_limit = |before: Option<Battery>, after: Option<Battery>| match (before, after) {
            (Some(before), Some(after)) => before.level.abs_diff(after.level) <= BATTERY_LIMIT,
            _ => true,
        };

        within_limit(self.left_battery, next.left_battery)
            && within_limit(self.right_battery, next.right_battery)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn properties(left: Option<u8>, right: Option<u8>, rssi: i16) -> DeviceProperties {
        DeviceProperties {
            rssi,
            address: 0x1122334455,
            model: AppleDeviceModel::AirPodsPro2,
            left_battery: left.map(|level| Battery::new(level, false)),
            right_battery: right.map(|level| Battery::new(level, false)),
            case_battery: None,
            left_in_ear: false,
            right_in_ear: false,
        }
    }

    #[test]
    fn overall_level_is_the_lower_bud() {
        assert_eq!(
            properties(Some(80), Some(60), -50).overall_level(),
            Some(60)
        );
        assert_eq!(
            properties(Some(30), Some(90), -50).overall_level(),
            Some(30)
        );
    }

    #[test]
    fn overall_level_falls_back_to_the_reporting_bud() {
        assert_eq!(properties(Some(70), None, -50).overall_level(), Some(70));
        assert_eq!(properties(None, Some(40), -50).overall_level(), Some(40));
    }

    #[test]
    fn overall_level_is_unknown_when_neither_bud_reports() {
        assert_eq!(properties(None, None, -50).overall_level(), None);
    }

    #[test]
    fn an_empty_bud_is_not_mistaken_for_a_missing_one() {
        // This is the regression the `Option<Battery>` model exists to prevent: a bud at 0% used
        // to be indistinguishable from a bud with no reading, so the overall level jumped to the
        // other bud instead of reporting 0%.
        assert_eq!(properties(Some(0), Some(90), -50).overall_level(), Some(0));
    }

    #[test]
    fn charging_requires_both_reporting_buds_to_charge() {
        let mut props = properties(Some(50), Some(50), -50);
        props.left_battery = Some(Battery::new(50, true));
        assert!(
            !props.is_charging(),
            "one bud charging is not the device charging"
        );

        props.right_battery = Some(Battery::new(50, true));
        assert!(props.is_charging());
    }

    #[test]
    fn charging_uses_the_only_reporting_bud() {
        let mut props = properties(Some(50), None, -50);
        props.left_battery = Some(Battery::new(50, true));
        assert!(props.is_charging());
    }

    #[test]
    fn low_battery_checks_either_bud() {
        assert!(properties(Some(15), Some(90), -50).has_low_battery(20));
        assert!(properties(Some(90), Some(15), -50).has_low_battery(20));
        assert!(!properties(Some(90), Some(90), -50).has_low_battery(20));
    }

    #[test]
    fn low_battery_ignores_a_charging_bud() {
        let mut props = properties(Some(5), Some(90), -50);
        props.left_battery = Some(Battery::new(5, true));
        assert!(!props.has_low_battery(20));
    }

    #[test]
    fn rejects_updates_from_a_different_model() {
        let current = properties(Some(80), Some(80), -50);
        let mut other = properties(Some(80), Some(80), -50);
        other.model = AppleDeviceModel::AirPods4;

        assert!(!current.is_plausible_update(&other));
    }

    #[test]
    fn rejects_updates_with_an_implausible_signal_jump() {
        let current = properties(Some(80), Some(80), -50);

        assert!(current.is_plausible_update(&properties(Some(80), Some(80), -90)));
        assert!(!current.is_plausible_update(&properties(Some(80), Some(80), -101)));
    }

    #[test]
    fn rejects_updates_with_an_implausible_battery_jump() {
        let current = properties(Some(80), Some(80), -50);

        assert!(current.is_plausible_update(&properties(Some(60), Some(80), -50)));
        assert!(!current.is_plausible_update(&properties(Some(50), Some(80), -50)));
        assert!(!current.is_plausible_update(&properties(Some(80), Some(50), -50)));
    }

    #[test]
    fn an_absent_battery_reading_cannot_reject_an_update() {
        // Buds routinely stop reporting when they go into the case. That must not be read as
        // "this is a different device" and freeze the UI on stale readings.
        let current = properties(Some(80), Some(80), -50);

        assert!(current.is_plausible_update(&properties(None, Some(80), -50)));
        assert!(current.is_plausible_update(&properties(None, None, -50)));
        assert!(properties(None, None, -50).is_plausible_update(&current));
    }

    #[test]
    fn over_ear_models_never_report_a_case() {
        // AirPods Max still populate the case nibble; drawing it would invent a case.
        let mut data = [0u8; winpods_apple_cp::MESSAGE_LEN];
        data[0] = 0x07;
        data[1] = (winpods_apple_cp::MESSAGE_LEN - 2) as u8;
        data[3..5].copy_from_slice(&0x200Au16.to_le_bytes()); // AirPods Max
        data[6] = 0x88;
        data[7] = 0x05; // case nibble = 5
        let message = ProximityPairingMessage::from_bytes(&data).expect("valid payload");

        let props = DeviceProperties::from_advertisement(-50, 1, &message);
        assert_eq!(props.model, AppleDeviceModel::AirPodsMax);
        assert_eq!(
            message.case_battery(),
            Some(50),
            "the payload does carry a case nibble"
        );
        assert_eq!(props.case_battery, None, "but it must not be surfaced");
    }

    #[test]
    fn in_ear_models_do_report_a_case() {
        let mut data = [0u8; winpods_apple_cp::MESSAGE_LEN];
        data[0] = 0x07;
        data[1] = (winpods_apple_cp::MESSAGE_LEN - 2) as u8;
        data[3..5].copy_from_slice(&0x2014u16.to_le_bytes()); // AirPods Pro 2
        data[6] = 0x88;
        data[7] = 0x05;
        let message = ProximityPairingMessage::from_bytes(&data).expect("valid payload");

        let props = DeviceProperties::from_advertisement(-50, 1, &message);
        assert_eq!(props.case_battery, Some(Battery::new(50, false)));
    }
}
