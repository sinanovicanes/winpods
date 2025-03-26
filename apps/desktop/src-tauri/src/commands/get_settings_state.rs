use std::sync::RwLock;

use crate::settings::SettingsState;

#[tauri::command]
pub fn get_settings_state(settings_state: tauri::State<RwLock<SettingsState>>) -> SettingsState {
    tracing::info!("UI requested current settings state");
    let settings_state = settings_state.read().unwrap();

    settings_state.clone()
}
