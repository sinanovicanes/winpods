use serde::{Deserialize, Serialize};
use windows::Devices::Radios::{Radio, RadioState};

/// Whether bluetooth is usable.
///
/// Windows distinguishes several "not on" radio states (off, disabled, unknown). None of them
/// mean anything different to the app, so they all collapse into [`AdapterState::Off`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AdapterState {
    On,
    Off,
}

impl AdapterState {
    pub const fn is_on(self) -> bool {
        matches!(self, Self::On)
    }

    pub const fn from_powered(powered: bool) -> Self {
        if powered { Self::On } else { Self::Off }
    }
}

impl From<RadioState> for AdapterState {
    fn from(state: RadioState) -> Self {
        Self::from_powered(state == RadioState::On)
    }
}

impl From<&Radio> for AdapterState {
    fn from(radio: &Radio) -> Self {
        match radio.State() {
            Ok(state) => state.into(),
            Err(e) => {
                tracing::warn!("Could not read the radio state: {e}");
                Self::Off
            }
        }
    }
}
