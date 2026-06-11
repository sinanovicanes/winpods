use std::sync::RwLock;

use tauri::{
    App, AppHandle, Emitter, Manager, Wry,
    menu::{Menu, MenuEvent, MenuItem, MenuItemKind},
};

use bluetooth::DeviceConnectionState;

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
    MenuItem::with_id(
        app,
        REFRESH_BATTERY_MENU_ID,
        "Refresh Battery",
        true,
        None::<&str>,
    )
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
    MenuItem::with_id(
        app,
        RECONNECT_MENU_ID,
        "Reconnect AirPods",
        true,
        None::<&str>,
    )
    .unwrap()
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

pub fn update_menu_items(menu: &Menu<Wry>, device_manager: &DeviceManagerState) {
    let status = match &device_manager.device {
        Some(device) => {
            let name = device.get_name().unwrap_or_else(|_| "AirPods".to_string());
            let state = match device.get_connection_state() {
                DeviceConnectionState::Connected => "Connected",
                DeviceConnectionState::Disconnected => "Selected",
            };
            format!("{name}: {state}")
        }
        None => "No AirPods selected".to_string(),
    };

    set_menu_text(menu, STATUS_MENU_ID, status);

    let Some(properties) = &device_manager.device_properties else {
        set_menu_text(menu, LEFT_BATTERY_MENU_ID, "Left: --");
        set_menu_text(menu, RIGHT_BATTERY_MENU_ID, "Right: --");
        set_menu_text(menu, CASE_BATTERY_MENU_ID, "Case: --");
        return;
    };

    set_menu_text(
        menu,
        LEFT_BATTERY_MENU_ID,
        format!("Left: {}", battery_text(properties.left_battery.level)),
    );
    set_menu_text(
        menu,
        RIGHT_BATTERY_MENU_ID,
        format!("Right: {}", battery_text(properties.right_battery.level)),
    );
    set_menu_text(
        menu,
        CASE_BATTERY_MENU_ID,
        format!(
            "Case: {}",
            properties
                .case_battery
                .as_ref()
                .map(|battery| battery_text(battery.level))
                .unwrap_or_else(|| "--".to_string())
        ),
    );
}

fn battery_text(level: u8) -> String {
    if level == 0 {
        "--".to_string()
    } else {
        format!("{level}%")
    }
}

fn set_menu_text(menu: &Menu<Wry>, id: &str, text: impl AsRef<str>) {
    let Some(MenuItemKind::MenuItem(item)) = menu.get(id) else {
        return;
    };

    if let Err(error) = item.set_text(text) {
        tracing::warn!("Failed to update tray menu item {id}: {error}");
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
