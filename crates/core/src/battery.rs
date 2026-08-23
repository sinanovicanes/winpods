use serde::{Deserialize, Serialize};

/// A single battery reading.
///
/// Only ever constructed for a battery that actually reported a level. An absent reading is
/// modelled as `Option<Battery>` rather than as a `Battery` with `level: 0`, which is what the
/// previous version did -- and which forced the UI into treating 0% as "unknown", so a genuinely
/// empty battery was indistinguishable from a missing one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Battery {
    /// Charge level as a percentage, `0..=100`.
    pub level: u8,
    pub charging: bool,
}

impl Battery {
    pub const fn new(level: u8, charging: bool) -> Self {
        Self {
            level: if level > 100 { 100 } else { level },
            charging,
        }
    }

    /// Whether the level is at or below `threshold`, ignoring a battery that is charging.
    ///
    /// Used for the low battery notification: a charging bud is on its way up, so warning about
    /// it is noise.
    pub const fn is_low(self, threshold: u8) -> bool {
        !self.charging && self.level <= threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamps_levels_above_one_hundred() {
        assert_eq!(Battery::new(150, false).level, 100);
        assert_eq!(Battery::new(100, false).level, 100);
        assert_eq!(Battery::new(0, false).level, 0);
    }

    #[test]
    fn a_charging_battery_is_never_low() {
        assert!(!Battery::new(5, true).is_low(20));
        assert!(Battery::new(5, false).is_low(20));
    }

    #[test]
    fn low_is_inclusive_of_the_threshold() {
        assert!(Battery::new(20, false).is_low(20));
        assert!(!Battery::new(21, false).is_low(20));
    }

    #[test]
    fn a_zero_threshold_only_matches_an_empty_battery() {
        // Threshold 0 is how the UI disables the notification; callers check for that before
        // asking, but an empty battery still legitimately reports low.
        assert!(Battery::new(0, false).is_low(0));
        assert!(!Battery::new(10, false).is_low(0));
    }
}
