use std::sync::RwLock;

use media::{AudioEndpoint, AudioRouteSummary, list_render_endpoints, route_render_audio_to_device};

use crate::device_manager::DeviceManagerState;

#[tauri::command]
pub fn get_audio_output_endpoints(
    device_manager: tauri::State<RwLock<DeviceManagerState>>,
) -> Result<Vec<AudioEndpoint>, String> {
    let selected_device_name = selected_device_name(device_manager);

    list_render_endpoints(selected_device_name.as_deref())
}

#[tauri::command]
pub fn route_audio_to_selected_device(
    device_manager: tauri::State<RwLock<DeviceManagerState>>,
) -> Result<AudioRouteSummary, String> {
    let selected_device_name =
        selected_device_name(device_manager).ok_or_else(|| "No selected device".to_string())?;

    let summary = route_render_audio_to_device(&selected_device_name)?;
    tracing::info!(
        "Audio route update: selected_device={}, roles_succeeded={}/{}",
        selected_device_name,
        summary.roles_succeeded,
        summary.roles_attempted
    );

    Ok(summary)
}

fn selected_device_name(
    device_manager: tauri::State<RwLock<DeviceManagerState>>,
) -> Option<String> {
    let device_manager = device_manager.read().ok()?;
    let device = device_manager.device.as_ref()?;

    device.get_name().ok()
}
