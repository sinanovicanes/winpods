//! Keeps the update status current, and installs automatically when the user opted in.
//!
//! One task does both, so there is exactly one update check in the app. The UI reads the status
//! this produces; it never checks on its own.

use std::sync::Arc;
use std::time::Duration;

use tauri::AppHandle;

use crate::state::AppState;

/// How often to re-check. Updates are not urgent and the endpoint should not be polled hard.
const CHECK_INTERVAL: Duration = Duration::from_secs(60 * 60);

pub fn init(app: &AppHandle, state: &Arc<AppState>) {
    let app = app.clone();
    let state = Arc::clone(state);

    tauri::async_runtime::spawn(async move {
        loop {
            // Always check, even with automatic updates off: the dashboard needs to know an
            // update exists before it can offer the button.
            let status = match state.updates.check(&app).await {
                Ok(status) => status,
                Err(e) => {
                    tracing::warn!("Could not check for updates: {e:#}");
                    tokio::time::sleep(CHECK_INTERVAL).await;
                    continue;
                }
            };

            if status.is_installable() && state.settings.get().await.auto_update {
                // `install` restarts the app on success, so nothing after this runs in that case.
                if let Err(e) = state.updates.install(&app).await {
                    tracing::warn!("Automatic update install failed: {e:#}");
                }
            }

            tokio::time::sleep(CHECK_INTERVAL).await;
        }
    });
}
