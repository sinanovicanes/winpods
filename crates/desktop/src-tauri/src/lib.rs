//! winpods: AirPods integration for Windows.
//!
//! # Shape of the app
//!
//! The backend owns all state and pushes it to the UI. A single broadcast channel of
//! [`events::AppEvent`] carries everything that changes; features subscribe to it, and one bridge
//! task forwards each event to the webviews. The UI reads the current state with commands and
//! requests changes with commands, so every write has a result the UI can act on.
//!
//! ```text
//!   AdapterWatcher ─┐                        ┌─ features::ear_detection
//!                   ├─ AppEvent broadcast ───┼─ features::low_battery
//!   AdvertisementWatcher                     ├─ features::autostart
//!         │                                  ├─ tray::tooltip
//!         └─ DeviceService                   └─ features::bridge ── emit ─→ webviews
//! ```
//!
//! # Async
//!
//! Every WinRT call is awaited rather than blocked on, so nothing occupies a thread for the
//! duration of a bluetooth round trip. That has one hard requirement: WinRT needs a COM apartment
//! on the calling thread and tokio worker threads have none, so
//! [`winpods_bluetooth::com::ensure_mta`] must run before any other bluetooth call.

use std::sync::Arc;

use tauri::{Manager, WindowEvent};

mod bluetooth;
mod commands;
mod device;
mod error;
mod events;
mod features;
pub mod logging;
mod settings;
mod state;
mod tray;
mod updater;
mod views;

use state::AppState;

pub fn run() {
    // Register the COM apartment before anything touches WinRT. Doing it here, on the main
    // thread and before any task exists, keeps it off the hot paths.
    winpods_bluetooth::com::ensure_mta();

    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // A second launch should surface the running app rather than start another tray icon.
            if let Some(window) = views::get(app, views::WIDGET) {
                let _ = window.set_focus();
            }
        }))
        .invoke_handler(tauri::generate_handler![
            commands::get_adapter_state,
            commands::list_devices,
            commands::get_current_device,
            commands::select_device,
            commands::clear_device_selection,
            commands::get_settings,
            commands::update_settings,
            commands::get_update_status,
            commands::install_update,
        ])
        .setup(|app| {
            let settings_path = app
                .path()
                .app_config_dir()
                .map(|dir| dir.join("settings.json"))?;
            let state = AppState::new(settings_path, app.package_info().version.to_string())?;

            app.manage(Arc::clone(&state));
            features::init(app.handle(), &state);
            tray::init(app, &state)?;
            bluetooth::init(&state);

            Ok(())
        })
        .on_window_event(|window, event| {
            // Prevents window from closing
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();

                if let Err(e) = window.hide() {
                    tracing::error!("Failed to hide {}: {e}", window.label());
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("failed to run the winpods application");
}
