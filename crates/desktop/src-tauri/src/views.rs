//! The app's two webview windows.
//!
//! Both are declared in `tauri.conf.json` and created before the setup hook runs. Each loads its
//! own prerendered SvelteKit route, so the UI no longer has to branch on the window label at
//! runtime the way it did when both windows shared one entry point.

use tauri::{AppHandle, Manager, WebviewWindow};

/// The dashboard window: device overview and settings.
pub const DASHBOARD: &str = "main";

/// The frameless widget shown from the tray icon.
pub const WIDGET: &str = "widget";

/// Looks up a window by label, logging when it is missing.
///
/// A missing window is always a configuration bug rather than a runtime condition, so callers get
/// an `Option` and a log line instead of an error to propagate.
pub fn get(app: &AppHandle, label: &str) -> Option<WebviewWindow> {
    let window = app.get_webview_window(label);

    if window.is_none() {
        tracing::error!("Webview window `{label}` is not declared in tauri.conf.json");
    }

    window
}
