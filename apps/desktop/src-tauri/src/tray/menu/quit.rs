use tauri::{
    menu::{MenuEvent, MenuItem},
    App, AppHandle, Wry,
};

pub const MENU_ID: &str = "quit";

pub fn create_menu_item(app: &App) -> MenuItem<Wry> {
    MenuItem::with_id(app, MENU_ID, "Quit", true, None::<&str>).unwrap()
}

pub fn on_menu_event(app: &AppHandle, _event: MenuEvent) {
    tracing::info!("Quitting app");
    app.exit(0);
}
