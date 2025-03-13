use tauri::Manager;

mod bluetooth;
mod events;
mod models;
mod tray;
mod utils;
mod views;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window(views::WIDGET) {
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_pinia::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .setup(move |app| {
            tray::init(app);
            bluetooth::init(app);

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

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
