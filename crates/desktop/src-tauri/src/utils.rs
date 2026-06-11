use tauri::App;

pub fn get_app_name(app: &App) -> String {
    app.config()
        .product_name
        .clone()
        .unwrap_or("App".to_string())
}
