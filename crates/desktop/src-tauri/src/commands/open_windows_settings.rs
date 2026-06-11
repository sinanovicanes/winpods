use std::process::Command;

const BLUETOOTH_SETTINGS_URI: &str = "ms-settings:bluetooth";
const SOUND_SETTINGS_URI: &str = "ms-settings:sound";

#[tauri::command]
pub fn open_bluetooth_settings() {
    open_settings_uri(BLUETOOTH_SETTINGS_URI);
}

#[tauri::command]
pub fn open_sound_settings() {
    open_settings_uri(SOUND_SETTINGS_URI);
}

fn open_settings_uri(uri: &str) {
    if let Err(error) = Command::new("explorer.exe").arg(uri).spawn() {
        tracing::error!("Failed to open Windows settings URI {uri}: {error}");
    }
}
