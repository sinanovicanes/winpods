use tauri::{
    tray::{MouseButtonState, TrayIcon, TrayIconEvent},
    Manager,
};

pub fn on_tray_icon_event(tray: &TrayIcon, event: TrayIconEvent) {
    match event {
        tauri::tray::TrayIconEvent::Click {
            position,
            button,
            button_state,
            ..
        } => {
            let app_handle = tray.app_handle();
            if !matches!(button, tauri::tray::MouseButton::Left) {
                return;
            }

            if !matches!(button_state, MouseButtonState::Down) {
                return;
            }

            if let Some(window) = app_handle.get_webview_window("main") {
                // Get the window's current size
                if let Ok(size) = window.inner_size() {
                    // Position window above the tray icon
                    let _ =
                        window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                            x: (position.x - (size.width as f64 / 2.0)) as i32,
                            y: (position.y - size.height as f64) as i32,
                        }));
                }

                // Toggle window visibility
                if let Ok(true) = window.is_visible() {
                    let _ = window.hide();
                    tracing::info!("Hiding window");
                } else {
                    tracing::info!("Showing window");
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        }
        _ => {}
    }
}
