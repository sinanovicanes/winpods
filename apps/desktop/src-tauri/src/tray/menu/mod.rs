use tauri::{
    menu::{Menu, MenuEvent},
    App, AppHandle, Wry,
};

mod dashboard;
mod quit;

pub fn create_tray_menu(app: &App) -> Menu<Wry> {
    Menu::with_items(
        app,
        &[
            &dashboard::create_menu_item(app),
            &quit::create_menu_item(app),
        ],
    )
    .expect("Failed to create tray menu")
}

pub fn on_menu_event(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        quit::MENU_ID => quit::on_menu_event(app, event),
        dashboard::MENU_ID => dashboard::on_menu_event(app, event),
        _ => {
            tracing::warn!("Unknown menu item: {:?}", event.id);
        }
    }
}
