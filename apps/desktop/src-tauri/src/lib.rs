use bluetooth::AdvirtesementWatcher;
use device::apple_cp::AirPods;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tauri::Manager;

mod tray;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_device(state: tauri::State<'_, Arc<Mutex<AppState>>>) -> Option<Device> {
    let state = state.lock().unwrap();
    state.device.clone()
}

#[derive(Debug, Clone, Serialize)]
struct Device {
    model: String,
    left_battery: u8,
    right_battery: u8,
}

#[derive(Default)]
struct AppState {
    device: Option<Device>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = Arc::new(Mutex::new(AppState::default()));
    let mut watcher = AdvirtesementWatcher::new();

    // Clone state for the callback
    let callback_state = state.clone();
    watcher.on_received(move |args| {
        let mut state = callback_state.lock().unwrap();
        let apple_data = args.manufacturer_data_map.get(&76);

        if let Some(apple_data) = apple_data {
            let airpods = AirPods::from_bytes(apple_data);

            if let Some(airpods) = airpods {
                state.device = Some(Device {
                    model: airpods.get_model_as_string(),
                    left_battery: airpods.get_left_battery().unwrap_or(0),
                    right_battery: airpods.get_right_battery().unwrap_or(0),
                });
                tracing::info!("Updated AirPods state: {:?}", state.device);
            }
        }
    });

    watcher.start();

    tauri::Builder::default()
        .setup(|app| {
            tray::init(app);

            app.manage(state);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_device])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
