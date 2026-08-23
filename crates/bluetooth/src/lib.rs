//! Windows bluetooth access for winpods.
//!
//! Three pieces, each usable on its own:
//!
//! - [`adapter`] watches the bluetooth radio so the app knows whether bluetooth is usable at all.
//! - [`advertisement`] watches BLE advertisements and hands out their manufacturer payloads.
//!   Decoding those payloads is the job of the `winpods-apple-cp` crate.
//! - [`device`] wraps a paired bluetooth device, reports connection and name changes, and
//!   enumerates the connected devices.
//!
//! # Threading
//!
//! Every WinRT call in here is `async`. WinRT requires a COM apartment on the calling thread and
//! tokio worker threads do not have one, so [`com::ensure_mta`] must run once during startup
//! before anything else in this crate is used. The entry points call it defensively, but relying
//! on that means paying for a `OnceLock` check on every call.
//!
//! Watchers deliver their events over [`tokio::sync::broadcast`] channels. WinRT invokes the
//! underlying handlers on its own threadpool threads, so a `broadcast::Sender::send` is the only
//! work done there -- nothing blocks a WinRT callback.

pub mod adapter;
pub mod advertisement;
pub mod com;
pub mod device;

mod properties;

pub use adapter::{AdapterState, AdapterWatcher};
pub use advertisement::{Advertisement, AdvertisementWatcher};
pub use device::{
    Device, DeviceEvent, DeviceIdentity, DiscoveredDevice, display_name, format_address,
};
pub use winpods_core::ConnectionState;
