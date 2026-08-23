//! Decoder for the Apple Continuity Protocol (ACP) payloads that AirPods and Beats devices
//! broadcast over BLE.
//!
//! Apple devices advertise unauthenticated manufacturer specific data under Bluetooth SIG
//! company id [`VENDOR_ID`]. For AirPods style devices the interesting payload is the
//! *proximity pairing* message, which carries battery levels, charging flags, lid state and
//! in-ear detection. This is where winpods gets every battery reading it shows.
//!
//! None of this is documented by Apple. The field layout below is the result of community
//! reverse engineering, so every accessor is covered by unit tests built from real captured
//! payloads -- a regression shows up as a failing test instead of a silently wrong battery
//! percentage.
//!
//! This crate is deliberately platform independent and depends on nothing but `serde`, so the
//! decoding can be tested anywhere even though the app itself only runs on Windows.

mod model;
mod proximity_pairing;

pub use model::AppleDeviceModel;
pub use proximity_pairing::{MESSAGE_LEN, ProximityPairingMessage, ProximitySide};

/// Bluetooth SIG company identifier assigned to Apple, Inc.
pub const VENDOR_ID: u16 = 76;

/// The Continuity message types.
///
/// Only [`PacketType::ProximityPairing`] is decoded by this crate; the rest are listed to
/// document what else turns up under Apple's company id.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketType {
    AirPrint = 0x03,
    AirDrop = 0x05,
    HomeKit = 0x06,
    ProximityPairing = 0x07,
    HeySiri = 0x08,
    AirPlay = 0x09,
    MagicSwitch = 0x0B,
    Handoff = 0x0C,
    InstantHotspotTetheringTargetPresence = 0x0D,
    InstantHotspotTetheringSourcePresence = 0x0E,
    NearbyAction = 0x0F,
    NearbyInfo = 0x10,
}
