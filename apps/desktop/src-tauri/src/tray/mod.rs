use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButtonState, TrayIcon, TrayIconBuilder},
    App, Manager,
};

pub fn init(app: &mut App) -> TrayIcon {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).unwrap();
    let menu = Menu::with_items(app, &[&quit_i]).unwrap();
    let app_name = app
        .config()
        .product_name
        .clone()
        .unwrap_or("App".to_string());

    // Clone app handle for use in the event closures
    let app_handle = app.handle().clone();

    let tray: TrayIcon = TrayIconBuilder::new()
        .tooltip(app_name)
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                tracing::info!("Quitting app");
                app.exit(0);
            }
            _ => {
                tracing::warn!("Unknown menu item: {:?}", event.id);
            }
        })
        .on_tray_icon_event(move |_tray_handler, event| {
            match event {
                tauri::tray::TrayIconEvent::Click {
                    position,
                    button,
                    button_state,
                    ..
                } => {
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
                            let _ = window.set_position(tauri::Position::Physical(
                                tauri::PhysicalPosition {
                                    x: (position.x - (size.width as f64 / 2.0)) as i32,
                                    y: (position.y - size.height as f64) as i32,
                                },
                            ));
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
        })
        .build(app)
        .unwrap();

    tray
}
