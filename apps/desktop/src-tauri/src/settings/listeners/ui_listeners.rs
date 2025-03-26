use std::sync::RwLock;

use tauri::{App, Listener, Manager};

use crate::settings::SettingsState;

/// Creates UI event listeners to update the settings state
pub fn init(app: &mut App) {
    let app_handle = app.app_handle().clone();
    app.listen("settings:set:auto_update", move |event| {
        let Ok(new_state) = event.payload().parse::<bool>() else {
            tracing::error!("Failed to parse payload for settings:set:auto_update");
            return;
        };

        let settings_state = app_handle.state::<RwLock<SettingsState>>();
        let Ok(mut settings_state) = settings_state.write() else {
            tracing::error!("Failed to lock settings state");
            return;
        };

        settings_state.set_auto_update(new_state);
    });

    let app_handle = app.app_handle().clone();
    app.listen("settings:set:notification", move |event| {
        let Ok(new_state) = event.payload().parse::<bool>() else {
            tracing::error!("Failed to parse payload for settings:set:notification");
            return;
        };

        let settings_state = app_handle.state::<RwLock<SettingsState>>();
        let Ok(mut settings_state) = settings_state.write() else {
            tracing::error!("Failed to lock settings state");
            return;
        };

        settings_state.set_notification(new_state);
    });

    let app_handle = app.app_handle().clone();
    app.listen("settings:set:low_battery_notification", move |event| {
        let Ok(new_state) = event.payload().parse::<bool>() else {
            tracing::error!("Failed to parse payload for settings:set:low_battery_notification");
            return;
        };

        let settings_state = app_handle.state::<RwLock<SettingsState>>();
        let Ok(mut settings_state) = settings_state.write() else {
            tracing::error!("Failed to lock settings state");
            return;
        };

        settings_state.set_low_battery_notification(new_state);
    });

    let app_handle = app.app_handle().clone();
    app.listen("settings:set:ear_detection", move |event| {
        let Ok(new_state) = event.payload().parse::<bool>() else {
            tracing::error!("Failed to parse payload for settings:set:ear_detection");
            return;
        };

        let settings_state = app_handle.state::<RwLock<SettingsState>>();
        let Ok(mut settings_state) = settings_state.write() else {
            tracing::error!("Failed to lock settings state");
            return;
        };

        settings_state.set_ear_detection(new_state);
    });
}
