use tauri::{
    menu::{MenuEvent, MenuItem},
    App, AppHandle, Wry,
};

pub const MENU_ID: &str = "settings";

pub fn create_menu_item(app: &App) -> MenuItem<Wry> {
    MenuItem::with_id(app, MENU_ID, "Settings", true, None::<&str>).unwrap()
}

pub fn on_menu_event(_app: &AppHandle, _event: MenuEvent) {
    tracing::warn!("Settings menu item not implemented");
}
