mod proximity_pairing;

pub use proximity_pairing::*;
use serde::{Deserialize, Serialize};

pub const VENDOR_ID: u16 = 76;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AppleDeviceModel {
    AirPods1,
    AirPods2,
    AirPods3,
    AirPodsPro,
    AirPodsPro2,
    AirPodsPro2UsbC,
    AirPodsMax,
    BeatsFitPro,
    #[default]
    Unknown,
}

impl AppleDeviceModel {
    pub fn from_model_id(model_id: u16) -> Self {
        match model_id {
            0x2002 => AppleDeviceModel::AirPods1,
            0x200F => AppleDeviceModel::AirPods2,
            0x2013 => AppleDeviceModel::AirPods3,
            0x200E => AppleDeviceModel::AirPodsPro,
            0x2014 => AppleDeviceModel::AirPodsPro2,
            0x2024 => AppleDeviceModel::AirPodsPro2UsbC,
            0x200A => AppleDeviceModel::AirPodsMax,
            0x2012 => AppleDeviceModel::BeatsFitPro,
            _ => AppleDeviceModel::Unknown,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
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
#[allow(dead_code)]
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

pub fn proximity_pairing_message_from_bytes(
    data: &[u8],
) -> Option<proximity_pairing::ProximityPairingMessage> {
    ProximityPairingMessage::from_bytes(data)
}
