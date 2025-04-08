use serde::Serialize;
use std::sync::RwLock;

use crate::{
    device_manager::{DeviceManagerState, DeviceProperties},
    models::DeviceInfo,
};

#[derive(Debug, Clone, Serialize)]
pub struct CurrentDeviceInfo {
    device: Option<DeviceInfo>,
    properties: Option<DeviceProperties>,
}

#[tauri::command]
pub fn get_current_device(
    device_manager: tauri::State<RwLock<DeviceManagerState>>,
) -> CurrentDeviceInfo {
    tracing::info!("UI requested current device information");
    let device_manager = device_manager.read().unwrap();
    let device = device_manager.device.clone().map(|d| DeviceInfo::from(&d));
    let properties = device_manager.device_properties.clone();

    CurrentDeviceInfo { device, properties }
}
