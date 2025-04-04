use std::sync::RwLock;

use tauri::Manager;
use tauri_plugin_updater::UpdaterExt;

use crate::settings::SettingsState;

pub fn init(app: &mut tauri::App) {
    // Check if automatic updates are enabled
    let settings_state = app.state::<RwLock<SettingsState>>();
    let settings_state = settings_state.read().unwrap();

    if !settings_state.auto_update {
        return;
    }

    let app_handle = app.app_handle().clone();
    tauri::async_runtime::spawn(async move {
        update(app_handle).await.unwrap_or_else(|e| {
            tracing::error!("Failed to check for updates: {:?}", e);
        });
    });
}

async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    // Check for updates
    if let Some(update) = app.updater()?.check().await? {
        let mut downloaded = 0;

        // alternatively we could also call update.download() and update.install() separately
        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    tracing::info!("Downloaded {downloaded} from {content_length:?}");
                },
                || {
                    tracing::info!("Download finished");
                },
            )
            .await?;

        tracing::info!("Update installed");
        app.restart();
    }

    Ok(())
}
