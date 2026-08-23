//! Notifies once when a bud runs low.

use std::sync::Arc;

use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

use crate::{events::AppEvent, state::AppState};

pub fn start(app: &AppHandle, state: &Arc<AppState>) {
    let app = app.clone();
    let state = Arc::clone(state);

    tauri::async_runtime::spawn(async move {
        // Latches so the user gets one notification per low-battery episode rather than one per
        // advertisement, which arrive every few seconds.
        let mut notified = false;
        let mut events = state.subscribe();

        while let Ok(event) = events.recv().await {
            match event {
                AppEvent::DevicePropertiesUpdated(properties) => {
                    let settings = state.settings.get().await;

                    if !settings.low_battery_notification_enabled() {
                        continue;
                    }

                    if !properties.has_low_battery(settings.low_battery_threshold) {
                        // Back above the threshold (or charging), so the next dip notifies again.
                        notified = false;
                        continue;
                    }

                    if notified {
                        continue;
                    }

                    notify(&app, settings.low_battery_threshold);
                    notified = true;
                }

                // Without this the latch would stay set, and reconnecting an already low device
                // would never notify.
                AppEvent::DeviceConnectionChanged(connection) if !connection.is_connected() => {
                    notified = false;
                }
                AppEvent::DeviceSelectionCleared => notified = false,

                _ => {}
            }
        }
    });
}

fn notify(app: &AppHandle, threshold: u8) {
    let result = app
        .notification()
        .builder()
        .title("winpods - Low battery")
        .body(format!("Your device is at or below {threshold}%."))
        .show();

    match result {
        Ok(()) => tracing::info!("Sent the low battery notification"),
        Err(e) => tracing::error!("Failed to send the low battery notification: {e}"),
    }
}
