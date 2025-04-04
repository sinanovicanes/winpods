use tauri::{
    menu::{MenuEvent, MenuItem},
    App, AppHandle, Manager, Wry,
};

pub const MENU_ID: &str = "dashboard";

pub fn create_menu_item(app: &App) -> MenuItem<Wry> {
    MenuItem::with_id(app, MENU_ID, "Dashboard", true, None::<&str>).unwrap()
}

pub fn on_menu_event(app: &AppHandle, _event: MenuEvent) {
    let Some(view) = app.get_webview_window(crate::views::MAIN) else {
        tracing::error!("Main window not found");
        return;
    };

    if view.show().is_err() {
        tracing::error!("Failed to show main window")
    }
}
