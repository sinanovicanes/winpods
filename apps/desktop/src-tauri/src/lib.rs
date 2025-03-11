use bluetooth::AdvirtesementWatcher;
use device::apple_cp::AirPods;
use models::{Battery, ConnectedDevice};
use std::sync::{Arc, Mutex};
use tauri::{Emitter, Manager};

mod models;
mod tray;

#[derive(Default)]
struct AppState {
    device: Option<ConnectedDevice>,
}

// Store the watcher to keep it alive
struct BluetoothWatcher {
    _watcher: AdvirtesementWatcher,
}

// Define event names as constants for consistency
const EVENT_DEVICE_CONNECTED: &str = "device-connected";
const EVENT_DEVICE_UPDATED: &str = "device-updated";
const EVENT_DEVICE_DISCONNECTED: &str = "device-disconnected";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = Arc::new(Mutex::new(AppState::default()));

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_pinia::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .setup(move |app| {
            tray::init(app);

            // let window = app.get_webview_window("main").unwrap();
            // let window_clone = window.clone();

            // Hide the window when it loses focus
            // window.on_window_event(move |event| match event {
            //     tauri::WindowEvent::Focused(focused) => {
            //         if !focused {
            //             tracing::info!("Hiding window");
            //             let _ = window_clone.hide();
            //         }
            //     }
            //     _ => {}
            // });

            // Set up watcher
            let mut watcher = AdvirtesementWatcher::new();

            // Clone state for the callback
            let callback_state = state.clone();

            // Get app_handle for the callback
            let app_handle = app.app_handle().clone();

            watcher.on_received(move |args| {
                let mut state_guard = callback_state.lock().unwrap();
                let apple_data = args.manufacturer_data_map.get(&76);

                if let Some(apple_data) = apple_data {
                    let airpods = AirPods::from_bytes(apple_data);

                    if let Some(airpods) = airpods {
                        let left_battery = Battery::new(
                            airpods.get_left_battery().unwrap_or(0) * 10,
                            airpods.is_left_charging(),
                        );

                        let right_battery = Battery::new(
                            airpods.get_right_battery().unwrap_or(0) * 10,
                            airpods.is_right_charging(),
                        );

                        let connected_device = ConnectedDevice::new(
                            args.address.to_string(),
                            airpods.get_model(),
                            right_battery,
                            left_battery,
                        );

                        // Determine if this is a new connection or an update
                        let is_new_connection = state_guard.device.is_none();

                        // Update the state
                        state_guard.device = Some(connected_device.clone());

                        // Drop the mutex guard before emitting events to avoid deadlocks
                        drop(state_guard);

                        // Emit the appropriate event based on connection status
                        if is_new_connection {
                            app_handle
                                .emit(EVENT_DEVICE_CONNECTED, connected_device)
                                .unwrap_or_else(|e| {
                                    tracing::error!("Failed to emit device connected event: {}", e);
                                });
                        } else {
                            app_handle
                                .emit(EVENT_DEVICE_UPDATED, connected_device)
                                .unwrap_or_else(|e| {
                                    tracing::error!("Failed to emit device updated event: {}", e);
                                });
                        }
                    }
                } else {
                    // Check if we had a device before and now we don't
                    if state_guard.device.is_some() {
                        state_guard.device = None;

                        // Drop the mutex guard before emitting events
                        drop(state_guard);

                        // Emit disconnection event
                        app_handle
                            .emit(EVENT_DEVICE_DISCONNECTED, ())
                            .unwrap_or_else(|e| {
                                tracing::error!("Failed to emit device disconnected event: {}", e);
                            });
                    }
                }
            });

            watcher.start();

            // Store the watcher in the app state to keep it alive
            app.manage(BluetoothWatcher { _watcher: watcher });

            app.manage(state.clone());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
