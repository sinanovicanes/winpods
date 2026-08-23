use anyhow::{Context, Result};
use tauri::{
    App, AppHandle, Wry,
    menu::{Menu, MenuEvent, MenuItem},
};

use crate::views;

const DASHBOARD: &str = "dashboard";
const QUIT: &str = "quit";

pub fn build(app: &App) -> Result<Menu<Wry>> {
    let dashboard = MenuItem::with_id(app, DASHBOARD, "Dashboard", true, None::<&str>)
        .context("failed to create the dashboard menu item")?;
    let quit = MenuItem::with_id(app, QUIT, "Quit", true, None::<&str>)
        .context("failed to create the quit menu item")?;

    Menu::with_items(app, &[&dashboard, &quit]).context("failed to create the tray menu")
}

pub fn on_event(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        DASHBOARD => show_dashboard(app),
        QUIT => {
            tracing::info!("Quitting");
            app.exit(0);
        }
        other => tracing::warn!("Unknown tray menu item: {other}"),
    }
}

fn show_dashboard(app: &AppHandle) {
    let Some(window) = views::get(app, views::DASHBOARD) else {
        return;
    };

    if let Err(e) = window.show() {
        tracing::error!("Failed to show the dashboard: {e}");
        return;
    }

    // The window is only hidden, never closed, so it can still be minimized from a previous use.
    let _ = window.unminimize();

    if let Err(e) = window.set_focus() {
        tracing::error!("Failed to focus the dashboard: {e}");
    }
}
