use std::sync::RwLock;

use bluetooth::{ServiceState, ServiceToggleSummary, set_device_service_state};

use crate::device_manager::DeviceManagerState;

#[tauri::command]
pub fn connect_selected_device_services(
    device_manager: tauri::State<RwLock<DeviceManagerState>>,
) -> Result<ServiceToggleSummary, String> {
    set_selected_device_services(device_manager, ServiceState::Enabled)
}

#[tauri::command]
pub fn disconnect_selected_device_services(
    device_manager: tauri::State<RwLock<DeviceManagerState>>,
) -> Result<ServiceToggleSummary, String> {
    set_selected_device_services(device_manager, ServiceState::Disabled)
}

fn set_selected_device_services(
    device_manager: tauri::State<RwLock<DeviceManagerState>>,
    state: ServiceState,
) -> Result<ServiceToggleSummary, String> {
    let device_manager = device_manager.read().unwrap();
    let device = device_manager
        .device
        .as_ref()
        .ok_or_else(|| "No selected device".to_string())?;
    let address = device
        .get_address()
        .map_err(|_| "Selected device has no readable Bluetooth address".to_string())?;
    drop(device_manager);

    let summary = set_device_service_state(address, state)?;
    tracing::info!(
        "Bluetooth service state update ({:?}): attempted={}, succeeded={}, failed={}",
        state,
        summary.attempted,
        summary.succeeded,
        summary.failed
    );

    Ok(summary)
}
