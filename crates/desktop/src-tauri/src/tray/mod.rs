//! The system tray icon: menu, click handling and the hover tooltip.

mod menu;
mod tooltip;

use std::sync::Arc;

use anyhow::{Context, Result};
use tauri::{
    App,
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
};
use tauri_plugin_positioner::{Position, WindowExt};

use crate::{state::AppState, views};

/// Builds the tray icon and starts the task that keeps its tooltip current.
pub fn init(app: &App, state: &Arc<AppState>) -> Result<TrayIcon> {
    let name = app_name(app);

    let tray = TrayIconBuilder::new()
        .tooltip(&name)
        .icon(
            app.default_window_icon()
                .context("the app has no default window icon")?
                .clone(),
        )
        .menu(&menu::build(app)?)
        .on_menu_event(menu::on_event)
        .on_tray_icon_event(on_tray_event)
        // Without this a left click both opens the menu and toggles the widget.
        .show_menu_on_left_click(false)
        .build(app)
        .context("failed to build the tray icon")?;

    tooltip::start(&tray, state, name);

    Ok(tray)
}

fn app_name(app: &App) -> String {
    app.config()
        .product_name
        .clone()
        .unwrap_or_else(|| "winpods".to_string())
}

/// Toggles the widget on a left click.
fn on_tray_event(tray: &TrayIcon, event: TrayIconEvent) {
    // Records the tray position so `Position::TrayCenter` below knows where to put the widget.
    tauri_plugin_positioner::on_tray_event(tray.app_handle(), &event);

    let TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Down,
        ..
    } = event
    else {
        return;
    };

    let app = tray.app_handle();

    let Some(window) = views::get(app, views::WIDGET) else {
        return;
    };

    if window.is_visible().unwrap_or(false) {
        tracing::debug!("Hiding the widget");

        if let Err(e) = window.hide() {
            tracing::error!("Failed to hide the widget: {e}");
        }

        return;
    }

    // Reposition before showing, otherwise the widget flashes at its previous location.
    if let Err(e) = window.move_window(Position::TrayCenter) {
        tracing::error!("Failed to move the widget to the tray: {e}");
        return;
    }

    tracing::debug!("Showing the widget");
    let _ = window.show();
    let _ = window.set_focus();
}
