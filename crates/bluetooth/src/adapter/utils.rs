use windows::Devices::{
    Bluetooth::BluetoothAdapter,
    Radios::{Radio, RadioKind, RadioState},
};

use super::AdapterState;

pub fn get_bluetooth_adapter_radio() -> Option<Radio> {
    let adapter = BluetoothAdapter::GetDefaultAsync().ok()?.get().ok()?;

    adapter.GetRadioAsync().ok()?.get().ok()
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
    let Some(radio) = get_bluetooth_adapter_radio() else {
        return false;
    };

    matches!(radio.State(), Ok(RadioState::On))
}

pub fn get_adapter_state() -> AdapterState {
    if is_adapter_on() {
        AdapterState::On
    } else {
        AdapterState::Off
    }
}
