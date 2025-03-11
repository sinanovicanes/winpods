#![allow(dead_code)]

// AppleCP = Apple Continuity Protocols
pub mod apple_cp {
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum PacketType {
        AirPrint = 0x3,
        AirDrop = 0x5,
        HomeKit = 0x6,
        ProximityPairing = 0x7,
        HeySiri = 0x8,
        AirPlay = 0x9,
        MagicSwitch = 0xB,
        Handoff = 0xC,
        InstantHotspotTetheringTargetPresence = 0xD,
        InstantHotspotTetheringSourcePresence = 0xE,
        NearbyAction = 0xF,
        NearbyInfo = 0x10,
    }

    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Color {
        White = 0x0,
        Black = 0x1,
        Red = 0x2,
        Blue = 0x3,
        Pink = 0x4,
        Gray = 0x5,
        Silver = 0x6,
        Gold = 0x7,
        RoseGold = 0x8,
        SpaceGray = 0x9,
        DarkBlue = 0xA,
        LightBlue = 0xB,
        Yellow = 0xC,
    }

    #[repr(C, packed)]
    #[derive(Debug, Clone, Copy)]
    pub struct Header {
        pub packet_type: PacketType,
        pub remaining_length: u8, // Remaining length of this packet
    }

    pub const VENDOR_ID: u16 = 76;

    #[repr(C, packed)]
    #[derive(Debug, Clone, Copy)]
    pub struct AirPods {
        // Header fields
        pub packet_type: PacketType,
        pub remaining_length: u8,

        // AirPods-specific fields
        unk1: [u8; 1],
        pub model_id: u16,
        status_flags: u8,        // Will be accessed through methods
        battery_status: [u8; 2], // Will be accessed through methods
        lid_status: u8,          // Will be accessed through methods
        pub color: Color,
        unk11: [u8; 1],
        unk12: [u8; 16], // Hash or encrypted payload
    }

    // Rust doesn't support bit fields directly, so we'll use methods to access the bit fields
    impl AirPods {
        pub fn is_valid(data: &[u8]) -> bool {
            if data.len() != std::mem::size_of::<AirPods>() {
                return false;
            }

            // Calculate the expected remaining length (match the C++ code)
            const HEADER_REMAINING_LENGTH_OFFSET: usize = 1; // Offset of remainingLength in Header
            const SHOULD_REMAINING_LENGTH: u8 = std::mem::size_of::<AirPods>() as u8
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
                let airpods_ptr = data.as_ptr() as *const AirPods;
                Some(*airpods_ptr)
            }
        }

        // Methods to access the bit fields based on the C++ implementation

        pub fn get_broadcast_side(&self) -> AirPodsSide {
            if (self.status_flags & 0x20) == 0 {
                // broadcastFrom == 0
                AirPodsSide::Right
            } else {
                AirPodsSide::Left
            }
        }

        pub fn is_left_broadcasted(&self) -> bool {
            self.get_broadcast_side() == AirPodsSide::Left
        }

        pub fn is_right_broadcasted(&self) -> bool {
            self.get_broadcast_side() == AirPodsSide::Right
        }

        pub fn get_model(&self) -> AirPodsModel {
            Self::get_model_from_id(self.model_id)
        }

        pub fn get_model_from_id(model_id: u16) -> AirPodsModel {
            match model_id {
                0x2002 => AirPodsModel::AirPods1,
                0x200F => AirPodsModel::AirPods2,
                0x2013 => AirPodsModel::AirPods3,
                0x200E => AirPodsModel::AirPodsPro,
                0x2014 => AirPodsModel::AirPodsPro2,
                0x2024 => AirPodsModel::AirPodsPro2UsbC,
                0x200A => AirPodsModel::AirPodsMax,
                0x2012 => AirPodsModel::BeatsFitPro,
                _ => AirPodsModel::Unknown,
            }
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

        fn get_case_battery(&self) -> Option<u8> {
            let val = self.battery_status[1] & 0x0F;
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

        pub fn get_case_battery_value(&self) -> Option<u8> {
            self.get_case_battery()
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

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum AirPodsSide {
        Left,
        Right,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum AirPodsModel {
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

    // Example utility to print the analysis of a data packet
    pub fn analyze_airpods_data(data: &[u8]) {
        println!("Data length: {}", data.len());

        if data.len() >= std::mem::size_of::<AirPods>() {
            if let Some(airpods) = AirPods::from_bytes(data) {
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
                    airpods.get_case_battery_value().map(|v| v * 10)
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
}
