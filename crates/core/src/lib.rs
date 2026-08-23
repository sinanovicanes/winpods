//! Platform independent core of winpods: the domain models the UI renders and the settings the
//! app persists.
//!
//! Deliberately free of any Windows dependency so the logic that has actually gone wrong in the
//! past -- battery aggregation, advertisement filtering, settings defaults -- can be unit tested
//! on any machine instead of only in CI.

mod battery;
mod connection;
mod device;
mod settings;

pub use battery::Battery;
pub use connection::ConnectionState;
pub use device::{DeviceInfo, DeviceProperties};
pub use settings::{Settings, SettingsPatch, SettingsStore};
