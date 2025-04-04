use tauri::{App, AppHandle};

pub fn get_app_name(app: &App) -> String {
    app.config()
        .product_name
        .clone()
        .unwrap_or("App".to_string())
}

pub fn get_app_name_by_handle(app_handle: &AppHandle) -> String {
    app_handle
        .config()
        .product_name
        .clone()
        .unwrap_or("App".to_string())
}
