use std::sync::RwLock;

use bluetooth::Device;
use serde::Serialize;

use crate::device_manager::{DeviceManagerState, DeviceProperties};

#[derive(Debug, Clone, Serialize)]
pub struct CurrentDeviceInfo {
    device: Option<Device>,
    properties: Option<DeviceProperties>,
}

#[tauri::command]
pub fn get_current_device(
    device_manager: tauri::State<RwLock<DeviceManagerState>>,
) -> CurrentDeviceInfo {
    tracing::info!("UI requested current device information");
    let device_manager = device_manager.read().unwrap();
    let device = device_manager.device.clone();
    let properties = device_manager.device_properties.clone();

    CurrentDeviceInfo { device, properties }
}
