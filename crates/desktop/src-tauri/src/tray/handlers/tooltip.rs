use std::sync::RwLock;

use crate::{device_manager::DeviceManagerState, events};
use tauri::{tray::TrayIcon, Listener, Manager};

pub fn init_tooltip_listener(tray: &TrayIcon) {
    let app_handle = tray.app_handle();
    let app_name = crate::utils::get_app_name_by_handle(app_handle);

    let tray_handle = tray.clone();
    let handle = app_handle.clone();
    let name = app_name.clone();
    app_handle.listen(events::DEVICE_PROPERTIES_UPDATED, move |_| {
        let device_manager = handle.state::<RwLock<DeviceManagerState>>();
        let device_manager = device_manager.read().unwrap();

        let tooltip = format!("{}\n{}", name, device_manager.to_tooltip());
        let _ = tray_handle.set_tooltip(Some(&tooltip));
    });

    let tray_handle = tray.clone();
    let name: String = app_name.clone();
    app_handle.listen(events::DEVICE_DISCONNECTED, move |_| {
        let _ = tray_handle.set_tooltip(Some(&name));
    });
}

pub trait Tooltip {
    fn to_tooltip(&self) -> String;
}
