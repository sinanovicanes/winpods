use std::sync::Mutex;

use tauri::{App, Manager};

use super::SettingsState;

pub fn init(app: &mut App) {
    let _settings_state = app.state::<Mutex<SettingsState>>();

    tracing::warn!("Ear detection not implemented yet");
}
