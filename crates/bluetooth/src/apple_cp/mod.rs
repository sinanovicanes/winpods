mod proximity_pairing;

pub use proximity_pairing::*;

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

pub const VENDOR_ID: u16 = 76;

pub fn proximity_pairing_message_from_bytes(
    data: &[u8],
) -> Option<proximity_pairing::ProximityPairingMessage> {
    ProximityPairingMessage::from_bytes(data)
}
