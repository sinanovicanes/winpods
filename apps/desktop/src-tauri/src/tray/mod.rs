use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIcon, TrayIconBuilder},
    App,
};

pub fn init(app: &mut App) -> TrayIcon {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).unwrap();
    let menu = Menu::with_items(app, &[&quit_i]).unwrap();
    let app_name = app
        .config()
        .product_name
        .clone()
        .unwrap_or("App".to_string());

    let tray: TrayIcon = TrayIconBuilder::new()
        .tooltip(app_name)
        .icon(app.default_window_icon().unwrap().clone())
        // .show_menu_on_left_click(true)
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
        .build(app)
        .unwrap();

    tray
}
