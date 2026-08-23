//! Checks for updates on startup when the user opted into automatic updates.
//!
//! The UI has its own updater flow for the manual "update available" button; this covers the
//! unattended path.

use std::sync::Arc;

use anyhow::Result;
use tauri::AppHandle;
use tauri_plugin_updater::UpdaterExt;

use crate::state::AppState;

pub fn start(app: &AppHandle, state: &Arc<AppState>) {
    let app = app.clone();
    let state = Arc::clone(state);

    tauri::async_runtime::spawn(async move {
        if !state.settings.get().await.auto_update {
            tracing::info!("Automatic updates are disabled, skipping the update check");
            return;
        }

        if let Err(e) = check_and_install(&app).await {
            // A failed update check is expected offline and must never be fatal.
            tracing::warn!("Update check failed: {e:#}");
        }
    });
}

async fn check_and_install(app: &AppHandle) -> Result<()> {
    let Some(update) = app.updater()?.check().await? else {
        tracing::info!("winpods is up to date");
        return Ok(());
    };

    tracing::info!("Installing update {}", update.version);

    let mut downloaded = 0usize;
    update
        .download_and_install(
            |chunk, total| {
                downloaded += chunk;
                tracing::debug!("Downloaded {downloaded} of {total:?} bytes");
            },
            || tracing::info!("Download finished, installing"),
        )
        .await?;

    tracing::info!("Update installed, restarting");
    app.restart();
}
