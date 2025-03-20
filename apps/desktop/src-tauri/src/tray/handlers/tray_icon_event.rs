use tauri::{
    tray::{MouseButtonState, TrayIcon, TrayIconEvent},
    Manager,
};
use tauri_plugin_positioner::{Position, WindowExt};

pub fn on_tray_icon_event(tray: &TrayIcon, event: TrayIconEvent) {
    tauri_plugin_positioner::on_tray_event(tray.app_handle(), &event);

    if let tauri::tray::TrayIconEvent::Click {
        button,
        button_state,
        ..
    } = event
    {
        if !matches!(button, tauri::tray::MouseButton::Left) {
            return;
        }

        if !matches!(button_state, MouseButtonState::Down) {
            return;
        }

        let app_handle = tray.app_handle();
        let Some(window) = app_handle.get_webview_window(crate::views::WIDGET) else {
            tracing::error!("Widget window not found");
            return;
        };

        // Toggle window visibility
        if let Ok(true) = window.is_visible() {
            tracing::info!("Hiding widget");

            if window.hide().is_err() {
                tracing::error!("Failed to hide window");
            }
            return;
        }

        // Update window position
        if window.move_window(Position::TrayCenter).is_err() {
            tracing::error!("Failed to move window to tray center");
            return;
        }

        tracing::info!("Showing widget");
        let _ = window.show();
        let _ = window.set_focus();
    }
}
