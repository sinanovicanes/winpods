use std::sync::Mutex;

use crate::settings::SettingsState;

#[tauri::command]
pub fn get_settings_state(settings_state: tauri::State<Mutex<SettingsState>>) -> SettingsState {
    tracing::info!("UI requested current settings state");
    let settings_state = settings_state.lock().unwrap();

    settings_state.clone()
}
