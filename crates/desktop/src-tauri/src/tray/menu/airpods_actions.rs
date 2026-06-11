use std::sync::RwLock;

use tauri::{
    menu::{MenuEvent, MenuItem},
    App, AppHandle, Emitter, Manager, Wry,
};

use crate::{device_manager::DeviceManagerState, events};

pub const STATUS_MENU_ID: &str = "airpods-status";
pub const LEFT_BATTERY_MENU_ID: &str = "airpods-left-battery";
pub const RIGHT_BATTERY_MENU_ID: &str = "airpods-right-battery";
pub const CASE_BATTERY_MENU_ID: &str = "airpods-case-battery";
pub const REFRESH_BATTERY_MENU_ID: &str = "refresh-battery";
pub const SWITCH_TO_PC_MENU_ID: &str = "switch-airpods-to-pc";
pub const RECONNECT_MENU_ID: &str = "reconnect-airpods";

pub fn create_status_item(app: &App) -> MenuItem<Wry> {
    MenuItem::with_id(app, STATUS_MENU_ID, "AirPods Status", false, None::<&str>).unwrap()
}

pub fn create_left_battery_item(app: &App) -> MenuItem<Wry> {
    MenuItem::with_id(app, LEFT_BATTERY_MENU_ID, "Left: --", false, None::<&str>).unwrap()
}

pub fn create_right_battery_item(app: &App) -> MenuItem<Wry> {
    MenuItem::with_id(app, RIGHT_BATTERY_MENU_ID, "Right: --", false, None::<&str>).unwrap()
}

pub fn create_case_battery_item(app: &App) -> MenuItem<Wry> {
    MenuItem::with_id(app, CASE_BATTERY_MENU_ID, "Case: --", false, None::<&str>).unwrap()
}

pub fn create_refresh_battery_item(app: &App) -> MenuItem<Wry> {
    MenuItem::with_id(app, REFRESH_BATTERY_MENU_ID, "Refresh Battery", true, None::<&str>)
        .unwrap()
}

pub fn create_switch_to_pc_item(app: &App) -> MenuItem<Wry> {
    MenuItem::with_id(
        app,
        SWITCH_TO_PC_MENU_ID,
        "Switch AirPods to this PC",
        true,
        None::<&str>,
    )
    .unwrap()
}

pub fn create_reconnect_item(app: &App) -> MenuItem<Wry> {
    MenuItem::with_id(app, RECONNECT_MENU_ID, "Reconnect AirPods", true, None::<&str>).unwrap()
}

pub fn on_menu_event(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        REFRESH_BATTERY_MENU_ID => {
            tracing::info!("Refresh Battery selected from tray");
            refresh_battery_state(app);
        }
        SWITCH_TO_PC_MENU_ID | RECONNECT_MENU_ID => {
            tracing::info!("AirPods reconnect fallback selected from tray");
            super::windows_settings::open_bluetooth_settings(app);
        }
        _ => {}
    }
}

fn refresh_battery_state(app: &AppHandle) {
    let device_manager = app.state::<RwLock<DeviceManagerState>>();
    let device_manager = device_manager.read().unwrap();
    let Some(properties) = device_manager.device_properties.clone() else {
        tracing::info!("No AirPods battery state available to refresh");
        return;
    };

    if let Err(error) = app.emit(events::DEVICE_PROPERTIES_UPDATED, properties) {
        tracing::error!("Failed to refresh AirPods battery state: {error}");
    }
}
