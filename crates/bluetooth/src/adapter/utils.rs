use windows::Devices::Radios::{Radio, RadioKind, RadioState};

use super::AdapterState;

pub fn get_bluetooth_adapter_radio() -> Option<Radio> {
    let radios = Radio::GetRadiosAsync().ok()?.get().ok()?;

    radios.into_iter().find(|radio| match radio.Kind() {
        Ok(kind) => matches!(kind, RadioKind::Bluetooth),
        Err(_) => false,
    })
}

pub fn get_adapter_radios() -> Vec<Radio> {
    let radios = Radio::GetRadiosAsync()
        .unwrap_or_else(|_| panic!("Failed to get radios"))
        .get()
        .unwrap_or_else(|_| panic!("Failed to get radios list"));

    radios
        .into_iter()
        .filter(|radio| match radio.Kind() {
            Ok(kind) => matches!(kind, RadioKind::Bluetooth),
            Err(_) => false,
        })
        .collect()
}

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
