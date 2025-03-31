use std::sync::RwLock;

use tauri::{App, AppHandle, Listener, Manager};
use tauri_plugin_autostart::ManagerExt;

use super::SettingsState;

fn update_auto_start(handle: &AppHandle, state: bool) {
    match state {
        true => handle.autolaunch().enable().unwrap_or_else(|e| {
            tracing::error!("Failed to set auto start: {:?}", e);
        }),
        false => handle.autolaunch().disable().unwrap_or_else(|e| {
            tracing::error!("Failed to set auto start: {:?}", e);
        }),
    }
}

fn synchronize_auto_start_state(app: &App) {
    let settings_state = app.state::<RwLock<SettingsState>>();
    let settings_state = settings_state.read().unwrap();
    let auto_launch = settings_state.auto_start;
    update_auto_start(app.app_handle(), auto_launch);
}

pub(super) fn init(app: &mut App) {
    // Make sure the auto start setting is in sync with the current state
    synchronize_auto_start_state(app);

    // Listen for further changes to the auto start setting
    let app_handle = app.app_handle().clone();
    app.listen("settings:update:auto_launch", move |event| {
        let Ok(state) = serde_json::from_str::<bool>(event.payload()) else {
            tracing::error!("Failed to parse auto start state: {:?}", event.payload());
            return;
        };

        update_auto_start(&app_handle, state);
    });
}
