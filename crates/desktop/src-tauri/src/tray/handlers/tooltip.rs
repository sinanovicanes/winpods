use std::sync::RwLock;

use crate::{device_manager::DeviceManagerState, events, tray::menu::airpods_actions};
use tauri::{Listener, Manager, menu::Menu, tray::TrayIcon};

pub fn init_tooltip_listener(tray: &TrayIcon, menu: &Menu<tauri::Wry>) {
    let app_handle = tray.app_handle();
    update_status(tray, menu);

    let tray_handle = tray.clone();
    let menu_handle = menu.clone();
    app_handle.listen(events::DEVICE_PROPERTIES_UPDATED, move |_| {
        update_status(&tray_handle, &menu_handle);
    });

    let tray_handle = tray.clone();
    let menu_handle = menu.clone();
    app_handle.listen(events::DEVICE_SELECTED, move |_| {
        update_status(&tray_handle, &menu_handle);
    });

    let tray_handle = tray.clone();
    let menu_handle = menu.clone();
    app_handle.listen(events::DEVICE_NAME_UPDATED, move |_| {
        update_status(&tray_handle, &menu_handle);
    });

    let tray_handle = tray.clone();
    let menu_handle = menu.clone();
    app_handle.listen(events::DEVICE_CONNECTION_STATE_UPDATED, move |_| {
        update_status(&tray_handle, &menu_handle);
    });

    let tray_handle = tray.clone();
    let menu_handle = menu.clone();
    app_handle.listen(events::DEVICE_SELECTION_CLEARED, move |_| {
        update_status(&tray_handle, &menu_handle);
    });
}

pub trait Tooltip {
    fn to_tooltip(&self) -> String;
}

fn update_status(tray: &TrayIcon, menu: &Menu<tauri::Wry>) {
    let app_handle = tray.app_handle();
    let device_manager = app_handle.state::<RwLock<DeviceManagerState>>();
    let device_manager = device_manager.read().unwrap();
    let tooltip = device_manager.to_tooltip();

    let _ = tray.set_tooltip(Some(&tooltip));
    airpods_actions::update_menu_items(menu, &device_manager);
}
