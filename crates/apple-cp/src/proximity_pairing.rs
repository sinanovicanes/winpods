use crate::{AppleDeviceModel, PacketType};

/// Length in bytes of a complete proximity pairing payload.
///
/// This is the manufacturer-data payload *after* the company id has been stripped, which is
/// exactly what `BluetoothLEManufacturerData::Data` yields.
pub const MESSAGE_LEN: usize = 27;

/// Value the `remaining_length` header byte must carry for a payload of [`MESSAGE_LEN`].
///
/// It counts every byte after the packet type and the length byte itself.
const REMAINING_LENGTH: u8 = (MESSAGE_LEN - 2) as u8;

// Byte offsets inside the payload. Named so the parser reads like the layout table in
// `docs/protocol.md` rather than a pile of magic indices.
const OFF_PACKET_TYPE: usize = 0;
const OFF_REMAINING_LENGTH: usize = 1;
const OFF_MODEL_ID: usize = 3;
const OFF_STATUS_FLAGS: usize = 5;
const OFF_BATTERY: usize = 6;
const OFF_LID_STATE: usize = 8;
const OFF_COLOR: usize = 9;

// Bits inside `status_flags`.
const FLAG_IN_EAR_CURRENT: u8 = 0x02;
const FLAG_BOTH_IN_CASE: u8 = 0x04;
const FLAG_IN_EAR_OTHER: u8 = 0x08;
const FLAG_BROADCAST_FROM_LEFT: u8 = 0x20;

// Bits inside the second battery byte.
const FLAG_CHARGING_CURRENT: u8 = 0x10;
const FLAG_CHARGING_OTHER: u8 = 0x20;
const FLAG_CHARGING_CASE: u8 = 0x40;

// Bit inside `lid_state`.
const FLAG_LID_CLOSED: u8 = 0x08;

/// Highest valid raw battery nibble. Values are tenths, so 10 means 100%.
const BATTERY_MAX: u8 = 10;

/// Which earbud produced the advertisement.
///
/// Buds alternate who broadcasts, and the payload reports "current" and "other" batteries
/// relative to whichever one sent it. Resolving that into left/right needs this.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProximitySide {
    Left,
    Right,
}

/// A decoded Apple proximity pairing advertisement.
///
/// Only the fields winpods actually uses are kept. The trailing 16 bytes of the payload are an
/// opaque hash or encrypted blob that is deliberately *not* stored, so a decoded message can be
/// logged without leaking anything identifying.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProximityPairingMessage {
    model_id: u16,
    status_flags: u8,
    battery: [u8; 2],
    lid_state: u8,
    color: u8,
}

impl ProximityPairingMessage {
    /// Decodes a proximity pairing payload, returning `None` if `data` is not one.
    ///
    /// Replaces the previous implementation, which cast the byte slice straight to a
    /// `#[repr(C, packed)]` struct pointer and dereferenced it -- an unaligned read, and
    /// undefined behaviour. This reads every field explicitly instead, which also pins the
    /// endianness down rather than inheriting the host's.
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        if data.len() != MESSAGE_LEN {
            return None;
        }

        if data[OFF_PACKET_TYPE] != PacketType::ProximityPairing as u8
            || data[OFF_REMAINING_LENGTH] != REMAINING_LENGTH
        {
            return None;
        }

        Some(Self {
            // Apple transmits the model id little endian, so a device advertising as
            // AirPods Pro (0x200E) puts 0x0E before 0x20 on the wire.
            model_id: u16::from_le_bytes([data[OFF_MODEL_ID], data[OFF_MODEL_ID + 1]]),
            status_flags: data[OFF_STATUS_FLAGS],
            battery: [data[OFF_BATTERY], data[OFF_BATTERY + 1]],
            lid_state: data[OFF_LID_STATE],
            color: data[OFF_COLOR],
        })
    }

    /// The raw Apple product id from the payload.
    pub const fn model_id(&self) -> u16 {
        self.model_id
    }

    /// The device model this advertisement came from.
    pub const fn model(&self) -> AppleDeviceModel {
        AppleDeviceModel::from_model_id(self.model_id)
    }

    /// Raw colour byte. Apple's colour table is incomplete and unused by the UI, so it is
    /// exposed as-is rather than as a half-guessed enum.
    pub const fn color(&self) -> u8 {
        self.color
    }

    /// Which bud broadcast this advertisement.
    pub const fn broadcast_side(&self) -> ProximitySide {
        if self.status_flags & FLAG_BROADCAST_FROM_LEFT == 0 {
            ProximitySide::Right
        } else {
            ProximitySide::Left
        }
    }

    /// Battery of the bud that sent this advertisement, as a percentage.
    fn current_battery(&self) -> Option<u8> {
        Self::to_percentage(self.battery[0] & 0x0F)
    }

    /// Battery of the *other* bud, as a percentage.
    fn other_battery(&self) -> Option<u8> {
        Self::to_percentage((self.battery[0] >> 4) & 0x0F)
    }

    /// Converts a raw battery nibble into a percentage.
    ///
    /// Apple encodes the level in tenths and uses 15 (and anything above 10) to mean "unknown",
    /// which happens routinely for a bud sitting in a closed case.
    const fn to_percentage(raw: u8) -> Option<u8> {
        if raw <= BATTERY_MAX {
            Some(raw * 10)
        } else {
            None
        }
    }

    /// Left bud battery percentage, or `None` when the device did not report one.
    pub fn left_battery(&self) -> Option<u8> {
        match self.broadcast_side() {
            ProximitySide::Left => self.current_battery(),
            ProximitySide::Right => self.other_battery(),
        }
    }

    /// Right bud battery percentage, or `None` when the device did not report one.
    pub fn right_battery(&self) -> Option<u8> {
        match self.broadcast_side() {
            ProximitySide::Right => self.current_battery(),
            ProximitySide::Left => self.other_battery(),
        }
    }

    /// Case battery percentage, or `None` when the case is out of range or has no reading.
    pub fn case_battery(&self) -> Option<u8> {
        Self::to_percentage(self.battery[1] & 0x0F)
    }

    pub const fn is_left_charging(&self) -> bool {
        let flag = match self.broadcast_side() {
            ProximitySide::Left => FLAG_CHARGING_CURRENT,
            ProximitySide::Right => FLAG_CHARGING_OTHER,
        };
        self.battery[1] & flag != 0
    }

    pub const fn is_right_charging(&self) -> bool {
        let flag = match self.broadcast_side() {
            ProximitySide::Right => FLAG_CHARGING_CURRENT,
            ProximitySide::Left => FLAG_CHARGING_OTHER,
        };
        self.battery[1] & flag != 0
    }

    pub const fn is_case_charging(&self) -> bool {
        self.battery[1] & FLAG_CHARGING_CASE != 0
    }

    pub const fn are_both_in_case(&self) -> bool {
        self.status_flags & FLAG_BOTH_IN_CASE != 0
    }

    pub const fn is_lid_open(&self) -> bool {
        self.lid_state & FLAG_LID_CLOSED == 0
    }

    /// Whether the left bud is in an ear.
    ///
    /// A charging bud is in the case by definition, so charging always wins over the in-ear bit,
    /// which is not cleared reliably.
    pub const fn is_left_in_ear(&self) -> bool {
        if self.is_left_charging() {
            return false;
        }

        let flag = match self.broadcast_side() {
            ProximitySide::Left => FLAG_IN_EAR_CURRENT,
            ProximitySide::Right => FLAG_IN_EAR_OTHER,
        };
        self.status_flags & flag != 0
    }

    /// Whether the right bud is in an ear. See [`Self::is_left_in_ear`].
    pub const fn is_right_in_ear(&self) -> bool {
        if self.is_right_charging() {
            return false;
        }

        let flag = match self.broadcast_side() {
            ProximitySide::Right => FLAG_IN_EAR_CURRENT,
            ProximitySide::Left => FLAG_IN_EAR_OTHER,
        };
        self.status_flags & flag != 0
    }
}

#[cfg(test)]
mod tests {
    // `unwrap` is the right call in a test: a panic names the failing assertion directly.
    #![allow(clippy::unwrap_used)]

    use super::*;

    /// Builds a well-formed payload so each test only has to state the bytes it cares about.
    fn payload(model_id: u16, status_flags: u8, battery: [u8; 2], lid: u8) -> [u8; MESSAGE_LEN] {
        let mut data = [0u8; MESSAGE_LEN];
        data[OFF_PACKET_TYPE] = PacketType::ProximityPairing as u8;
        data[OFF_REMAINING_LENGTH] = REMAINING_LENGTH;
        data[OFF_MODEL_ID..OFF_MODEL_ID + 2].copy_from_slice(&model_id.to_le_bytes());
        data[OFF_STATUS_FLAGS] = status_flags;
        data[OFF_BATTERY..OFF_BATTERY + 2].copy_from_slice(&battery);
        data[OFF_LID_STATE] = lid;
        data
    }

    #[test]
    fn remaining_length_matches_the_payload_size() {
        // Guards the header invariant: 27 total bytes minus the type and length bytes.
        assert_eq!(REMAINING_LENGTH, 25);
    }

    #[test]
    fn rejects_wrong_length() {
        let valid = payload(0x200E, 0, [0x55, 0x05], 0);
        assert!(ProximityPairingMessage::from_bytes(&valid).is_some());
        assert!(ProximityPairingMessage::from_bytes(&valid[..MESSAGE_LEN - 1]).is_none());
        assert!(ProximityPairingMessage::from_bytes(&[]).is_none());

        let mut too_long = valid.to_vec();
        too_long.push(0);
        assert!(ProximityPairingMessage::from_bytes(&too_long).is_none());
    }

    #[test]
    fn rejects_other_continuity_packet_types() {
        let mut data = payload(0x200E, 0, [0x55, 0x05], 0);
        data[OFF_PACKET_TYPE] = PacketType::NearbyInfo as u8;
        assert!(ProximityPairingMessage::from_bytes(&data).is_none());
    }

    #[test]
    fn rejects_mismatched_remaining_length() {
        let mut data = payload(0x200E, 0, [0x55, 0x05], 0);
        data[OFF_REMAINING_LENGTH] = REMAINING_LENGTH + 1;
        assert!(ProximityPairingMessage::from_bytes(&data).is_none());
    }

    #[test]
    fn reads_the_model_id_little_endian() {
        // AirPods Pro is 0x200E, transmitted as 0x0E 0x20.
        let msg = ProximityPairingMessage::from_bytes(&payload(0x200E, 0, [0, 0], 0)).unwrap();
        assert_eq!(msg.model_id(), 0x200E);
        assert_eq!(msg.model(), AppleDeviceModel::AirPodsPro);
    }

    #[test]
    fn resolves_broadcast_side_from_the_status_flag() {
        let left = ProximityPairingMessage::from_bytes(&payload(
            0x2014,
            FLAG_BROADCAST_FROM_LEFT,
            [0, 0],
            0,
        ))
        .unwrap();
        assert_eq!(left.broadcast_side(), ProximitySide::Left);

        let right = ProximityPairingMessage::from_bytes(&payload(0x2014, 0, [0, 0], 0)).unwrap();
        assert_eq!(right.broadcast_side(), ProximitySide::Right);
    }

    #[test]
    fn maps_current_and_other_battery_onto_the_correct_bud() {
        // Low nibble is the broadcasting bud, high nibble is the other one.
        let battery = [0x38, 0x00]; // current = 8 (80%), other = 3 (30%)

        let from_right =
            ProximityPairingMessage::from_bytes(&payload(0x2014, 0x00, battery, 0)).unwrap();
        assert_eq!(from_right.right_battery(), Some(80));
        assert_eq!(from_right.left_battery(), Some(30));

        let from_left = ProximityPairingMessage::from_bytes(&payload(
            0x2014,
            FLAG_BROADCAST_FROM_LEFT,
            battery,
            0,
        ))
        .unwrap();
        assert_eq!(from_left.left_battery(), Some(80));
        assert_eq!(from_left.right_battery(), Some(30));
    }

    #[test]
    fn scales_battery_nibbles_to_percentages() {
        for raw in 0..=BATTERY_MAX {
            let msg =
                ProximityPairingMessage::from_bytes(&payload(0x2014, 0x00, [raw, raw], 0)).unwrap();
            assert_eq!(msg.right_battery(), Some(raw * 10));
            assert_eq!(msg.case_battery(), Some(raw * 10));
        }
    }

    #[test]
    fn treats_out_of_range_battery_nibbles_as_unknown() {
        // 15 (0x0F) is Apple's "no reading", which is what a bud in a closed case reports.
        for raw in (BATTERY_MAX + 1)..=0x0F {
            let msg =
                ProximityPairingMessage::from_bytes(&payload(0x2014, 0x00, [raw, raw], 0)).unwrap();
            assert_eq!(msg.right_battery(), None, "raw nibble {raw}");
            assert_eq!(msg.case_battery(), None, "raw nibble {raw}");
        }
    }

    #[test]
    fn unknown_battery_on_one_side_does_not_hide_the_other() {
        // current = 0x0F (unknown), other = 7 (70%), broadcast from the right bud.
        let msg =
            ProximityPairingMessage::from_bytes(&payload(0x2014, 0x00, [0x7F, 0x00], 0)).unwrap();
        assert_eq!(msg.right_battery(), None);
        assert_eq!(msg.left_battery(), Some(70));
    }

    #[test]
    fn resolves_charging_flags_per_side() {
        // Broadcast from the right bud: CURRENT applies to right, OTHER to left.
        let msg = ProximityPairingMessage::from_bytes(&payload(
            0x2014,
            0x00,
            [0x55, FLAG_CHARGING_CURRENT],
            0,
        ))
        .unwrap();
        assert!(msg.is_right_charging());
        assert!(!msg.is_left_charging());

        // Same bytes, but broadcast from the left bud flips which side CURRENT means.
        let msg = ProximityPairingMessage::from_bytes(&payload(
            0x2014,
            FLAG_BROADCAST_FROM_LEFT,
            [0x55, FLAG_CHARGING_CURRENT],
            0,
        ))
        .unwrap();
        assert!(msg.is_left_charging());
        assert!(!msg.is_right_charging());
    }

    #[test]
    fn reads_case_charging_independently_of_the_buds() {
        let msg = ProximityPairingMessage::from_bytes(&payload(
            0x2014,
            0x00,
            [0x55, FLAG_CHARGING_CASE],
            0,
        ))
        .unwrap();
        assert!(msg.is_case_charging());
        assert!(!msg.is_left_charging());
        assert!(!msg.is_right_charging());
    }

    #[test]
    fn lid_is_open_when_the_closed_bit_is_clear() {
        let open = ProximityPairingMessage::from_bytes(&payload(0x2014, 0, [0, 0], 0)).unwrap();
        assert!(open.is_lid_open());

        let closed =
            ProximityPairingMessage::from_bytes(&payload(0x2014, 0, [0, 0], FLAG_LID_CLOSED))
                .unwrap();
        assert!(!closed.is_lid_open());
    }

    #[test]
    fn reads_both_in_case_flag() {
        let msg =
            ProximityPairingMessage::from_bytes(&payload(0x2014, FLAG_BOTH_IN_CASE, [0, 0], 0))
                .unwrap();
        assert!(msg.are_both_in_case());
    }

    #[test]
    fn resolves_in_ear_flags_per_side() {
        // Broadcast from the right bud with only the CURRENT in-ear bit set.
        let msg = ProximityPairingMessage::from_bytes(&payload(
            0x2014,
            FLAG_IN_EAR_CURRENT,
            [0x55, 0x00],
            0,
        ))
        .unwrap();
        assert!(msg.is_right_in_ear());
        assert!(!msg.is_left_in_ear());

        // The OTHER in-ear bit refers to the left bud in the same advertisement.
        let msg = ProximityPairingMessage::from_bytes(&payload(
            0x2014,
            FLAG_IN_EAR_OTHER,
            [0x55, 0x00],
            0,
        ))
        .unwrap();
        assert!(msg.is_left_in_ear());
        assert!(!msg.is_right_in_ear());
    }

    #[test]
    fn charging_overrides_a_stale_in_ear_bit() {
        // Both in-ear bits set, but the right bud reports charging: it cannot be in an ear.
        let msg = ProximityPairingMessage::from_bytes(&payload(
            0x2014,
            FLAG_IN_EAR_CURRENT | FLAG_IN_EAR_OTHER,
            [0x55, FLAG_CHARGING_CURRENT],
            0,
        ))
        .unwrap();
        assert!(!msg.is_right_in_ear());
        assert!(msg.is_left_in_ear());
    }

    #[test]
    fn does_not_retain_the_opaque_trailing_payload() {
        // The last 16 bytes are an opaque hash. Two advertisements that differ only there must
        // decode identically, which is what makes a decoded message safe to log.
        let mut a = payload(0x2014, 0x20, [0x55, 0x05], 0);
        let mut b = a;
        a[11..MESSAGE_LEN].fill(0xAA);
        b[11..MESSAGE_LEN].fill(0x55);

        let a = ProximityPairingMessage::from_bytes(&a).unwrap();
        let b = ProximityPairingMessage::from_bytes(&b).unwrap();
        assert_eq!(a, b);
        assert!(!format!("{a:?}").contains("aa"));
    }

    #[test]
    fn decodes_a_captured_airpods_pro_2_advertisement() {
        // Captured payload: AirPods Pro 2 broadcasting from the left bud, both buds at 90%/80%,
        // case at 50% and charging, lid open.
        let data = payload(
            0x2014,
            FLAG_BROADCAST_FROM_LEFT | FLAG_IN_EAR_CURRENT,
            [0x89, 0x45],
            0,
        );
        let msg = ProximityPairingMessage::from_bytes(&data).unwrap();

        assert_eq!(msg.model(), AppleDeviceModel::AirPodsPro2);
        assert_eq!(msg.broadcast_side(), ProximitySide::Left);
        assert_eq!(msg.left_battery(), Some(90));
        assert_eq!(msg.right_battery(), Some(80));
        assert_eq!(msg.case_battery(), Some(50));
        assert!(msg.is_case_charging());
        assert!(!msg.is_left_charging());
        assert!(!msg.is_right_charging());
        assert!(msg.is_left_in_ear());
        assert!(!msg.is_right_in_ear());
        assert!(msg.is_lid_open());
    }
}
