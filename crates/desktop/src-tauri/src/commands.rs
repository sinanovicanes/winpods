//! The commands the UI invokes.
//!
//! All of them are `async`. Every one either touches a `tokio::sync` lock or an underlying WinRT
//! call that is now awaited, so running them on Tauri's blocking pool would tie up a thread for
//! the duration of a bluetooth round trip.

use std::sync::Arc;

use serde::Serialize;
use tauri::{AppHandle, State};
use winpods_apple_cp::AppleDeviceModel;
use winpods_bluetooth::{AdapterState, Device, device};
use winpods_core::{DeviceInfo, Settings, SettingsPatch};

use crate::{device::DeviceSnapshot, error::CommandResult, state::AppState, updater::UpdateStatus};

/// A device offered in the picker.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableDevice {
    pub address: u64,
    pub name: String,
    pub model: AppleDeviceModel,
}

/// Whether bluetooth is currently usable.
#[tauri::command]
pub async fn get_adapter_state(state: State<'_, Arc<AppState>>) -> CommandResult<AdapterState> {
    Ok(state.adapter.state())
}

/// Lists the connected bluetooth devices for the picker.
#[tauri::command]
pub async fn list_devices(_state: State<'_, Arc<AppState>>) -> CommandResult<Vec<AvailableDevice>> {
    let devices = device::connected_devices().await?;

    Ok(devices
        .into_iter()
        .map(|found| AvailableDevice {
            address: found.address,
            model: found.model(),
            name: found.name,
        })
        .collect())
}

/// The selected device and its latest readings.
///
/// The UI calls this on load and whenever a window is shown again: a window that was still
/// loading when a device was selected missed the event, and without this it would sit on
/// "No device selected" until the next advertisement.
#[tauri::command]
pub async fn get_current_device(state: State<'_, Arc<AppState>>) -> CommandResult<DeviceSnapshot> {
    Ok(state.devices.snapshot().await)
}

/// Selects the device at `address`.
#[tauri::command]
pub async fn select_device(
    address: u64,
    state: State<'_, Arc<AppState>>,
) -> CommandResult<DeviceInfo> {
    let device = Device::from_address(address).await?;

    Ok(state.devices.select(device).await?)
}

/// Clears the current selection.
#[tauri::command]
pub async fn clear_device_selection(state: State<'_, Arc<AppState>>) -> CommandResult<()> {
    state.devices.clear().await;

    Ok(())
}

/// The current settings.
#[tauri::command]
pub async fn get_settings(state: State<'_, Arc<AppState>>) -> CommandResult<Settings> {
    Ok(state.settings.get().await)
}

/// Applies a partial settings change and returns the result.
///
/// Returning the settings is what lets the UI reconcile with what was actually stored, including
/// a value that got clamped -- something the previous fire-and-forget `emit` could not do.
#[tauri::command]
pub async fn update_settings(
    patch: SettingsPatch,
    state: State<'_, Arc<AppState>>,
) -> CommandResult<Settings> {
    Ok(state.settings.update(patch).await?)
}

/// The running version, whether a newer one is available, and any install in progress.
#[tauri::command]
pub async fn get_update_status(state: State<'_, Arc<AppState>>) -> CommandResult<UpdateStatus> {
    Ok(state.updates.status())
}

/// Downloads and installs the available update, then restarts the app.
///
/// A no-op when an install is already running, so pressing the button while the automatic path is
/// mid-install cannot start a second one.
#[tauri::command]
pub async fn install_update(app: AppHandle, state: State<'_, Arc<AppState>>) -> CommandResult<()> {
    state.updates.install(&app).await?;

    Ok(())
}
