use tauri::{
    tray::{TrayIcon, TrayIconBuilder},
    App,
};

mod handlers;
mod menu;

pub fn init(app: &mut App) -> TrayIcon {
    let app_name = crate::utils::get_app_name(app);
    let menu = menu::create_tray_menu(app);
    let tray: TrayIcon = TrayIconBuilder::new()
        .tooltip(app_name)
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(|app, event| menu::on_menu_event(app, event))
        .on_tray_icon_event(|tray, event| handlers::on_tray_icon_event(tray, event))
        .build(app)
        .unwrap();

    handlers::init_tooltip_listener(&tray);

    tray
}
