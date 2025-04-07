use windows::Devices::Radios::{Radio, RadioKind, RadioState};

use super::AdapterState;

pub fn is_adapter_on() -> bool {
    let radios = match Radio::GetRadiosAsync() {
        Ok(r) => match r.get() {
            Ok(list) => list,
            Err(_) => return false,
        },
        Err(_) => return false,
    };

    radios.into_iter().any(|radio| {
        matches!(radio.Kind(), Ok(RadioKind::Bluetooth))
            && matches!(radio.State(), Ok(RadioState::On))
    })
}

pub fn get_adapter_state() -> AdapterState {
    if is_adapter_on() {
        AdapterState::On
    } else {
        AdapterState::Off
    }
}
