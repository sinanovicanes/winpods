use std::process::Command;

use tauri::{
    App, AppHandle, Wry,
    menu::{MenuEvent, MenuItem},
};

pub const BLUETOOTH_SETTINGS_MENU_ID: &str = "open-bluetooth-settings";
pub const SOUND_SETTINGS_MENU_ID: &str = "open-sound-settings";

const BLUETOOTH_SETTINGS_URI: &str = "ms-settings:bluetooth";
const SOUND_SETTINGS_URI: &str = "ms-settings:sound";

pub fn create_bluetooth_settings_item(app: &App) -> MenuItem<Wry> {
    MenuItem::with_id(
        app,
        BLUETOOTH_SETTINGS_MENU_ID,
        "Open Bluetooth Settings",
        true,
        None::<&str>,
    )
    .unwrap()
}

pub fn create_sound_settings_item(app: &App) -> MenuItem<Wry> {
    MenuItem::with_id(
        app,
        SOUND_SETTINGS_MENU_ID,
        "Open Sound Settings",
        true,
        None::<&str>,
    )
    .unwrap()
}

pub fn on_menu_event(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        BLUETOOTH_SETTINGS_MENU_ID => open_bluetooth_settings(app),
        SOUND_SETTINGS_MENU_ID => open_sound_settings(app),
        _ => {}
    }
}

pub fn open_bluetooth_settings(app: &AppHandle) {
    open_settings_uri(app, BLUETOOTH_SETTINGS_URI);
}

pub fn open_sound_settings(app: &AppHandle) {
    open_settings_uri(app, SOUND_SETTINGS_URI);
}

fn open_settings_uri(_app: &AppHandle, uri: &str) {
    if let Err(error) = Command::new("explorer.exe").arg(uri).spawn() {
        tracing::error!("Failed to open Windows settings URI {uri}: {error}");
    }
}
