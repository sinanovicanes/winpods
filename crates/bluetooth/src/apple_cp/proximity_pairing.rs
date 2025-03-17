use serde::{Deserialize, Serialize};

use super::{Color, PacketType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProximitySide {
    Left,
    Right,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProximityPairingModel {
    AirPods1,
    AirPods2,
    AirPods3,
    AirPodsPro,
    AirPodsPro2,
    AirPodsPro2UsbC,
    AirPodsMax,
    BeatsFitPro,
    Unknown,
}

impl From<u16> for ProximityPairingModel {
    fn from(value: u16) -> Self {
        match value {
            0x2002 => ProximityPairingModel::AirPods1,
            0x200F => ProximityPairingModel::AirPods2,
            0x2013 => ProximityPairingModel::AirPods3,
            0x200E => ProximityPairingModel::AirPodsPro,
            0x2014 => ProximityPairingModel::AirPodsPro2,
            0x2024 => ProximityPairingModel::AirPodsPro2UsbC,
            0x200A => ProximityPairingModel::AirPodsMax,
            0x2012 => ProximityPairingModel::BeatsFitPro,
            _ => ProximityPairingModel::Unknown,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct ProximityPairingMessage {
    // Header fields
    packet_type: PacketType,
    pub remaining_length: u8,

    // AirPods-specific fields
    unk1: [u8; 1],
    pub model_id: u16,
    status_flags: u8,        // Will be accessed through methods
    battery_status: [u8; 2], // Will be accessed through methods
    lid_status: u8,          // Will be accessed through methods
    color: Color,
    unk11: [u8; 1],
    unk12: [u8; 16], // Hash or encrypted payload
}

// Rust doesn't support bit fields directly, so we'll use methods to access the bit fields
impl ProximityPairingMessage {
    pub const VENDOR_ID: u16 = super::VENDOR_ID;

    pub fn is_valid(data: &[u8]) -> bool {
        if data.len() != std::mem::size_of::<ProximityPairingMessage>() {
            return false;
        }

        // Calculate the expected remaining length (match the C++ code)
        const HEADER_REMAINING_LENGTH_OFFSET: usize = 1; // Offset of remainingLength in Header
        const SHOULD_REMAINING_LENGTH: u8 = std::mem::size_of::<ProximityPairingMessage>() as u8
            - (HEADER_REMAINING_LENGTH_OFFSET + std::mem::size_of::<u8>()) as u8;

        let packet_type = data[0];
        let remaining_length = data[1];

        // Check if this is a ProximityPairing packet with the expected length
        if packet_type != PacketType::ProximityPairing as u8
            || remaining_length != SHOULD_REMAINING_LENGTH
        {
            return false;
        }

        true
    }

    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        if !Self::is_valid(data) {
            return None;
        }

        // Safety: We've validated that the data is valid for our struct
        unsafe {
            let airpods_ptr = data.as_ptr() as *const ProximityPairingMessage;
            Some(*airpods_ptr)
        }
    }

    // Methods to access the bit fields based on the C++ implementation

    pub fn get_broadcast_side(&self) -> ProximitySide {
        if (self.status_flags & 0x20) == 0 {
            // broadcastFrom == 0
            ProximitySide::Right
        } else {
            ProximitySide::Left
        }
    }

    pub fn is_left_broadcasted(&self) -> bool {
        self.get_broadcast_side() == ProximitySide::Left
    }

    pub fn is_right_broadcasted(&self) -> bool {
        self.get_broadcast_side() == ProximitySide::Right
    }

    pub fn get_model(&self) -> ProximityPairingModel {
        Self::get_model_from_id(self.model_id)
    }

    pub fn get_model_from_id(model_id: u16) -> ProximityPairingModel {
        ProximityPairingModel::from(model_id)
    }

    pub fn get_model_as_string(&self) -> String {
        format!("{:?}", self.get_model())
    }

    fn get_curr_battery(&self) -> Option<u8> {
        let val = self.battery_status[0] & 0x0F;
        if val <= 10 { Some(val) } else { None }
    }

    fn get_anot_battery(&self) -> Option<u8> {
        let val = (self.battery_status[0] >> 4) & 0x0F;
        if val <= 10 { Some(val) } else { None }
    }

    pub fn get_left_battery(&self) -> Option<u8> {
        if self.is_left_broadcasted() {
            self.get_curr_battery()
        } else {
            self.get_anot_battery()
        }
    }

    pub fn get_right_battery(&self) -> Option<u8> {
        if self.is_right_broadcasted() {
            self.get_curr_battery()
        } else {
            self.get_anot_battery()
        }
    }

    pub fn get_case_battery(&self) -> Option<u8> {
        let val = self.battery_status[1] & 0x0F;
        if val <= 10 { Some(val) } else { None }
    }

    pub fn is_left_charging(&self) -> bool {
        if self.is_left_broadcasted() {
            (self.battery_status[1] & 0x10) != 0 // currCharging
        } else {
            (self.battery_status[1] & 0x20) != 0 // anotCharging
        }
    }

    pub fn is_right_charging(&self) -> bool {
        if self.is_right_broadcasted() {
            (self.battery_status[1] & 0x10) != 0 // currCharging
        } else {
            (self.battery_status[1] & 0x20) != 0 // anotCharging
        }
    }

    pub fn is_case_charging(&self) -> bool {
        (self.battery_status[1] & 0x40) != 0 // caseCharging
    }

    pub fn is_both_pods_in_case(&self) -> bool {
        (self.status_flags & 0x04) != 0 // bothInCase
    }

    pub fn is_lid_opened(&self) -> bool {
        (self.lid_status & 0x08) == 0 // lid.closed == 0
    }

    pub fn is_left_in_ear(&self) -> bool {
        // Match C++ implementation with the charging check
        if self.is_left_charging() {
            return false;
        }

        if self.is_left_broadcasted() {
            (self.status_flags & 0x02) != 0 // currInEar
        } else {
            (self.status_flags & 0x08) != 0 // anotInEar
        }
    }

    pub fn is_right_in_ear(&self) -> bool {
        // Match C++ implementation with the charging check
        if self.is_right_charging() {
            return false;
        }

        if self.is_right_broadcasted() {
            (self.status_flags & 0x02) != 0 // currInEar
        } else {
            (self.status_flags & 0x08) != 0 // anotInEar
        }
    }

    pub fn desensitize(&self) -> Self {
        // Create a copy with sensitive data removed/zeroed
        let mut result = *self;
        result.unk12 = [0; 16]; // Zero out the hash/encrypted payload
        result
    }
}

// Example utility to print the analysis of a data packet
pub fn analyze_proximity_message_data(data: &[u8]) {
    println!("Data length: {}", data.len());

    if data.len() >= std::mem::size_of::<ProximityPairingMessage>() {
        if let Some(airpods) = ProximityPairingMessage::from_bytes(data) {
            println!("Valid AirPods data detected:");
            println!("Model: {:?})", airpods.get_model());
            println!("Broadcast side: {:?}", airpods.get_broadcast_side());
            println!(
                "Left battery: {:?}%",
                airpods.get_left_battery().map(|v| v * 10)
            );
            println!(
                "Right battery: {:?}%",
                airpods.get_right_battery().map(|v| v * 10)
            );
            println!(
                "Case battery: {:?}%",
                airpods.get_case_battery().map(|v| v * 10)
            );
            println!("Left charging: {}", airpods.is_left_charging());
            println!("Right charging: {}", airpods.is_right_charging());
            println!("Case charging: {}", airpods.is_case_charging());
            println!("Lid opened: {}", airpods.is_lid_opened());
            println!("Left in ear: {}", airpods.is_left_in_ear());
            println!("Right in ear: {}", airpods.is_right_in_ear());
            println!("Both pods in case: {}", airpods.is_both_pods_in_case());
        } else {
            println!("Invalid AirPods data format");
        }
    } else {
        println!("Data is too short to be an AirPods packet");
    }
}
