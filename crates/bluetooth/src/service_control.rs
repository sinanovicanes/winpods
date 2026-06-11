use std::mem::size_of;

use serde::Serialize;
use windows::{
    Win32::{
        Devices::Bluetooth::{
            BLUETOOTH_DEVICE_INFO, BLUETOOTH_DEVICE_SEARCH_PARAMS, BLUETOOTH_FIND_RADIO_PARAMS,
            BLUETOOTH_SERVICE_DISABLE, BLUETOOTH_SERVICE_ENABLE,
            BluetoothEnumerateInstalledServices, BluetoothFindDeviceClose,
            BluetoothFindFirstDevice, BluetoothFindFirstRadio, BluetoothFindNextDevice,
            BluetoothFindNextRadio, BluetoothFindRadioClose, BluetoothSetServiceState,
        },
        Foundation::{CloseHandle, HANDLE},
    },
    core::GUID,
};

#[derive(Debug, Clone, Copy)]
pub enum ServiceState {
    Enabled,
    Disabled,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceToggleSummary {
    pub attempted: u32,
    pub succeeded: u32,
    pub failed: u32,
}

struct DeviceContext {
    device_info: BLUETOOTH_DEVICE_INFO,
    radio: HANDLE,
}

impl Drop for DeviceContext {
    fn drop(&mut self) {
        let _ = unsafe { CloseHandle(self.radio) };
    }
}

pub fn set_device_service_state(
    address: u64,
    state: ServiceState,
) -> Result<ServiceToggleSummary, String> {
    let mut context = find_device_context(address)
        .ok_or_else(|| format!("Bluetooth device not found for address {address}"))?;
    let services = installed_services(&mut context.device_info, context.radio)?;

    if services.is_empty() {
        return Err("No installed Bluetooth services found for selected device".to_string());
    }

    let service_flag = match state {
        ServiceState::Enabled => BLUETOOTH_SERVICE_ENABLE,
        ServiceState::Disabled => BLUETOOTH_SERVICE_DISABLE,
    };

    let mut summary = ServiceToggleSummary {
        attempted: services.len() as u32,
        succeeded: 0,
        failed: 0,
    };

    for service in services {
        let result = unsafe {
            BluetoothSetServiceState(
                Some(context.radio),
                &context.device_info,
                &service,
                service_flag,
            )
        };

        if result == 0 {
            summary.succeeded += 1;
        } else {
            summary.failed += 1;
        }
    }

    Ok(summary)
}

fn find_device_context(address: u64) -> Option<DeviceContext> {
    for radio in bluetooth_radios() {
        if let Some(device_info) = find_device_info(address, radio) {
            return Some(DeviceContext { device_info, radio });
        }

        let _ = unsafe { CloseHandle(radio) };
    }

    None
}

fn bluetooth_radios() -> Vec<HANDLE> {
    let radio_params = BLUETOOTH_FIND_RADIO_PARAMS {
        dwSize: size_of::<BLUETOOTH_FIND_RADIO_PARAMS>() as u32,
    };
    let mut radio = HANDLE::default();
    let Ok(find_handle) = (unsafe { BluetoothFindFirstRadio(&radio_params, &mut radio) }) else {
        return vec![];
    };

    let mut radios = vec![radio];

    loop {
        let mut next_radio = HANDLE::default();
        if unsafe { BluetoothFindNextRadio(find_handle, &mut next_radio) }.is_err() {
            break;
        }
        radios.push(next_radio);
    }

    let _ = unsafe { BluetoothFindRadioClose(find_handle) };
    radios
}

fn find_device_info(address: u64, radio: HANDLE) -> Option<BLUETOOTH_DEVICE_INFO> {
    let search_params = BLUETOOTH_DEVICE_SEARCH_PARAMS {
        dwSize: size_of::<BLUETOOTH_DEVICE_SEARCH_PARAMS>() as u32,
        fReturnAuthenticated: true.into(),
        fReturnRemembered: true.into(),
        fReturnUnknown: false.into(),
        fReturnConnected: true.into(),
        fIssueInquiry: false.into(),
        cTimeoutMultiplier: 0,
        hRadio: radio,
    };
    let mut device_info = BLUETOOTH_DEVICE_INFO {
        dwSize: size_of::<BLUETOOTH_DEVICE_INFO>() as u32,
        ..Default::default()
    };

    let find_handle = unsafe { BluetoothFindFirstDevice(&search_params, &mut device_info) }.ok()?;

    loop {
        let current_address = unsafe { device_info.Address.Anonymous.ullLong };

        if current_address == address {
            let _ = unsafe { BluetoothFindDeviceClose(find_handle) };
            return Some(device_info);
        }

        device_info.dwSize = size_of::<BLUETOOTH_DEVICE_INFO>() as u32;
        if unsafe { BluetoothFindNextDevice(find_handle, &mut device_info) }.is_err() {
            break;
        }
    }

    let _ = unsafe { BluetoothFindDeviceClose(find_handle) };
    None
}

fn installed_services(
    device_info: &mut BLUETOOTH_DEVICE_INFO,
    radio: HANDLE,
) -> Result<Vec<GUID>, String> {
    const MAX_SERVICES: usize = 32;

    let mut services = vec![GUID::zeroed(); MAX_SERVICES];
    let mut service_count = services.len() as u32;
    let result = unsafe {
        BluetoothEnumerateInstalledServices(
            Some(radio),
            device_info,
            &mut service_count,
            Some(services.as_mut_ptr()),
        )
    };

    if result != 0 {
        return Err(format!(
            "BluetoothEnumerateInstalledServices failed with code {result}"
        ));
    }

    services.truncate(service_count as usize);
    Ok(services)
}
