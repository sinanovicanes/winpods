use std::sync::RwLock;

use crate::{
    device_manager::{DeviceManagerState, SelectedDevice},
    events,
};
use tauri::{tray::TrayIcon, Listener, Manager};

pub fn init_tooltip_listener(tray: &TrayIcon) {
    let app_handle = tray.app_handle();
    let app_name = crate::utils::get_app_name_by_handle(app_handle);

    let tray_handle = tray.clone();
    let handle = app_handle.clone();
    let name = app_name.clone();
    app_handle.listen(events::DEVICE_UPDATED, move |_| {
        let device_manager = handle.state::<RwLock<DeviceManagerState>>();
        let device_manager = device_manager.read().unwrap();
        let Some(selected_device) = &device_manager.device else {
            return;
        };

        let tooltip = format!("{}\n{}", name, selected_device.to_tooltip());
        let _ = tray_handle.set_tooltip(Some(&tooltip));
    });

    let tray_handle = tray.clone();
    let name: String = app_name.clone();
    app_handle.listen(events::DEVICE_DISCONNECTED, move |_| {
        let _ = tray_handle.set_tooltip(Some(&name));
    });
}

impl SelectedDevice {
    fn to_tooltip(&self) -> String {
        if let Some(properties) = &self.properties {
            format!(
                "{}\nLeft: {}%\nRight: {}%",
                self.device.get_name().unwrap_or("Unknown".to_string()),
                properties.left_battery.level,
                properties.right_battery.level
            )
        } else {
            "".to_string()
        }
    }
}
