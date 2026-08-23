//! Keeps the "launch at sign-in" registration in step with the setting.

use std::sync::Arc;

use tauri::AppHandle;
use tauri_plugin_autostart::ManagerExt;

use crate::{events::AppEvent, state::AppState};

pub fn init(app: &AppHandle, state: &Arc<AppState>) {
    let app = app.clone();
    let state = Arc::clone(state);

    tauri::async_runtime::spawn(async move {
        // Windows registers autostart outside the app, so it can drift: an uninstalled build, a
        // user editing the registry, a profile copied to another machine. Applying the stored
        // setting once at startup brings the two back together.
        apply(&app, state.settings.get().await.auto_start);

        let mut events = state.subscribe();

        while let Ok(event) = events.recv().await {
            if let AppEvent::SettingsChanged(settings) = event {
                apply(&app, settings.auto_start);
            }
        }
    });
}

fn apply(app: &AppHandle, enabled: bool) {
    let manager = app.autolaunch();
    let result = match manager.is_enabled() {
        // Already in the requested state; the plugin would happily rewrite the registry entry,
        // but there is no reason to.
        Ok(current) if current == enabled => return,
        _ if enabled => manager.enable(),
        _ => manager.disable(),
    };

    match result {
        Ok(()) => tracing::info!(
            "Auto start {}",
            if enabled { "enabled" } else { "disabled" }
        ),
        Err(e) => tracing::error!("Failed to change the auto start registration: {e}"),
    }
}
