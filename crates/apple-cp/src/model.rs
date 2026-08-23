use serde::{Deserialize, Serialize};

/// The Apple and Beats models winpods knows how to name and draw.
///
/// Variants map to Apple product ids, which appear both in the proximity pairing payload and in
/// the `System.DeviceInterface.Bluetooth.ProductId` property of a paired Windows device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum AppleDeviceModel {
    AirPods1,
    AirPods2,
    AirPods3,
    AirPods4,
    AirPods4Anc,
    AirPodsPro,
    AirPodsPro2,
    AirPodsPro2UsbC,
    AirPodsPro3,
    AirPodsMax,
    AirPodsMaxUsbC,
    PowerbeatsPro,
    PowerbeatsPro2,
    BeatsFitPro,
    BeatsStudioBuds,
    BeatsStudioBudsPlus,
    BeatsSoloBuds,
    #[default]
    Unknown,
}

impl AppleDeviceModel {
    /// Maps an Apple product id onto a known model, falling back to
    /// [`AppleDeviceModel::Unknown`] for anything unrecognised.
    pub const fn from_model_id(model_id: u16) -> Self {
        match model_id {
            0x2002 => Self::AirPods1,
            0x200F => Self::AirPods2,
            0x2013 => Self::AirPods3,
            0x2019 => Self::AirPods4,
            0x201B => Self::AirPods4Anc,
            0x200E => Self::AirPodsPro,
            0x2014 => Self::AirPodsPro2,
            0x2024 => Self::AirPodsPro2UsbC,
            0x2027 => Self::AirPodsPro3,
            0x200A => Self::AirPodsMax,
            0x201F => Self::AirPodsMaxUsbC,
            0x200B => Self::PowerbeatsPro,
            0x201D => Self::PowerbeatsPro2,
            0x2012 => Self::BeatsFitPro,
            0x2011 => Self::BeatsStudioBuds,
            0x2016 => Self::BeatsStudioBudsPlus,
            0x2026 => Self::BeatsSoloBuds,
            _ => Self::Unknown,
        }
    }

    /// Whether this model charges in a case.
    ///
    /// Over-ear models have no case, so a case battery reading coming from one is meaningless
    /// and gets dropped rather than shown as an empty case.
    pub const fn has_case(self) -> bool {
        !matches!(self, Self::AirPodsMax | Self::AirPodsMaxUsbC)
    }

    /// Whether the model is a single unit rather than a pair of independent buds.
    ///
    /// Over-ear models report one battery, so the left/right split does not apply to them.
    pub const fn is_single_unit(self) -> bool {
        matches!(self, Self::AirPodsMax | Self::AirPodsMaxUsbC)
    }
}

#[cfg(test)]
mod tests {
    // `unwrap` is the right call in a test: a panic names the failing assertion directly.
    #![allow(clippy::unwrap_used)]

    use super::*;

    #[test]
    fn maps_known_product_ids() {
        let cases = [
            (0x2002, AppleDeviceModel::AirPods1),
            (0x200F, AppleDeviceModel::AirPods2),
            (0x2013, AppleDeviceModel::AirPods3),
            (0x2019, AppleDeviceModel::AirPods4),
            (0x201B, AppleDeviceModel::AirPods4Anc),
            (0x200E, AppleDeviceModel::AirPodsPro),
            (0x2014, AppleDeviceModel::AirPodsPro2),
            (0x2024, AppleDeviceModel::AirPodsPro2UsbC),
            (0x2027, AppleDeviceModel::AirPodsPro3),
            (0x200A, AppleDeviceModel::AirPodsMax),
            (0x201F, AppleDeviceModel::AirPodsMaxUsbC),
            (0x200B, AppleDeviceModel::PowerbeatsPro),
            (0x201D, AppleDeviceModel::PowerbeatsPro2),
            (0x2012, AppleDeviceModel::BeatsFitPro),
            (0x2011, AppleDeviceModel::BeatsStudioBuds),
            (0x2016, AppleDeviceModel::BeatsStudioBudsPlus),
            (0x2026, AppleDeviceModel::BeatsSoloBuds),
        ];

        for (id, expected) in cases {
            assert_eq!(
                AppleDeviceModel::from_model_id(id),
                expected,
                "product id {id:#06X}"
            );
        }
    }

    #[test]
    fn unknown_product_ids_fall_back() {
        for id in [0x0000, 0x1234, 0x2001, 0xFFFF] {
            assert_eq!(
                AppleDeviceModel::from_model_id(id),
                AppleDeviceModel::Unknown
            );
        }
    }

    #[test]
    fn over_ear_models_have_no_case() {
        assert!(!AppleDeviceModel::AirPodsMax.has_case());
        assert!(!AppleDeviceModel::AirPodsMaxUsbC.has_case());
        assert!(AppleDeviceModel::AirPodsPro2.has_case());
        assert!(AppleDeviceModel::BeatsFitPro.has_case());
    }

    #[test]
    fn over_ear_models_are_single_unit() {
        assert!(AppleDeviceModel::AirPodsMax.is_single_unit());
        assert!(!AppleDeviceModel::AirPods4.is_single_unit());
    }

    #[test]
    fn serialises_as_the_variant_name() {
        // The Svelte UI keys its model metadata table off these exact strings.
        let json = serde_json::to_string(&AppleDeviceModel::AirPodsPro2UsbC).unwrap();
        assert_eq!(json, "\"AirPodsPro2UsbC\"");
        assert_eq!(
            serde_json::to_string(&AppleDeviceModel::Unknown).unwrap(),
            "\"Unknown\""
        );
    }
}
