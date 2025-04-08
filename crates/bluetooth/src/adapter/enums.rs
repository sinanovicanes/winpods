use serde::{Deserialize, Serialize};
use windows::Devices::Radios::{Radio, RadioState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AdapterState {
    On,
    Off,
}

impl From<RadioState> for AdapterState {
    fn from(state: RadioState) -> Self {
        match state {
            RadioState::On => AdapterState::On,
            _ => AdapterState::Off,
        }
    }
}

impl From<&Radio> for AdapterState {
    fn from(radio: &Radio) -> Self {
        match radio.State() {
            Ok(state) => state.into(),
            Err(_) => AdapterState::Off,
        }
    }
}
