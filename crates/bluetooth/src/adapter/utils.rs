use std::sync::Once;

use windows::Devices::{
    Bluetooth::BluetoothAdapter,
    Radios::{Radio, RadioAccessStatus, RadioKind, RadioState},
};

use super::AdapterState;

static REQUEST_ACCESS: Once = Once::new();

/// Asks Windows for access to the radios of this machine.
///
/// This has to be done once per process before the radio APIs can be used, otherwise
/// they may report no radios at all depending on the Windows build and privacy settings.
fn request_radio_access() {
    REQUEST_ACCESS.call_once(|| {
        let status = Radio::RequestAccessAsync().and_then(|operation| operation.get());

        match status {
            Ok(RadioAccessStatus::Allowed) => tracing::debug!("Radio access granted"),
            Ok(status) => tracing::warn!("Radio access is not granted: {:?}", status),
            Err(e) => tracing::warn!("Failed to request radio access: {}", e),
        }
    });
}

/// Returns the radio of the default bluetooth adapter.
///
/// `BluetoothAdapter::GetRadioAsync` is unreliable outside of packaged apps (it may fail with
/// `REGDB_E_CLASSNOTREG`, and it reports nothing when the process architecture does not match the
/// machine architecture, e.g. an x64 build running emulated on an ARM64 device), so we fall back to
/// enumerating the radios directly.
pub fn get_bluetooth_adapter_radio() -> Option<Radio> {
    request_radio_access();

    match get_default_adapter_radio() {
        Ok(radio) => return Some(radio),
        Err(e) => tracing::warn!(
            "Failed to get the radio of the default bluetooth adapter: {}. Falling back to radio enumeration",
            e
        ),
    }

    let radio = get_adapter_radios().into_iter().next();

    if radio.is_none() {
        tracing::warn!("No bluetooth radio found on this machine");
    }

    radio
}

fn get_default_adapter_radio() -> windows::core::Result<Radio> {
    let adapter = BluetoothAdapter::GetDefaultAsync()?.get()?;

    adapter.GetRadioAsync()?.get()
}

pub fn get_adapter_radios() -> Vec<Radio> {
    request_radio_access();

    let radios = match Radio::GetRadiosAsync().and_then(|operation| operation.get()) {
        Ok(radios) => radios,
        Err(e) => {
            tracing::error!("Failed to get the radio list: {}", e);
            return vec![];
        }
    };

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
