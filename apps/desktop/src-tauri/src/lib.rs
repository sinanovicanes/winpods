use tauri::{Manager, WindowEvent};

mod commands;
mod device_manager;
mod events;
mod models;
mod settings;
mod tray;
mod utils;
mod views;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window(views::WIDGET) {
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_pinia::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_bluetooth_device_list,
            commands::connect,
            commands::disconnect
        ])
        .setup(move |app| {
            settings::init(app);
            device_manager::init(app);
            tray::init(app);

            Ok(())
        })
        .on_window_event(|window, event| match event {
            // Prevents window from closing
            WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
