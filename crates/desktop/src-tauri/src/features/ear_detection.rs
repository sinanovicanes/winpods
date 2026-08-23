//! Pauses media when a bud leaves an ear and resumes it when both are back in.

use std::sync::Arc;

use winpods_core::DeviceProperties;
use winpods_media::MediaController;

use crate::{events::AppEvent, state::AppState};

pub fn init(state: &Arc<AppState>) {
    let state = Arc::clone(state);

    tauri::async_runtime::spawn(async move {
        // Owned by this task alone, so no lock is needed. The previous version kept it in a
        // `Mutex` in Tauri's state purely because the logic lived in a synchronous event handler.
        let mut controller = MediaController::new();
        let mut events = state.subscribe();

        while let Ok(event) = events.recv().await {
            match event {
                AppEvent::DevicePropertiesUpdated(properties) => {
                    if !state.settings.get().await.ear_detection {
                        continue;
                    }

                    apply(&mut controller, &properties).await;
                }

                // The device is gone, so the sessions are no longer ours to resume. Forget them
                // rather than resuming: the user may have started playing something else since.
                AppEvent::DeviceConnectionChanged(connection) if !connection.is_connected() => {
                    forget(&mut controller, "device disconnected");
                }
                AppEvent::DeviceSelectionCleared => {
                    forget(&mut controller, "device selection cleared");
                }

                _ => {}
            }
        }
    });
}

async fn apply(controller: &mut MediaController, properties: &DeviceProperties) {
    let both_in_ear = properties.left_in_ear && properties.right_in_ear;

    if both_in_ear {
        if !controller.has_paused_sessions() {
            return;
        }

        tracing::info!("Both buds are in ear, resuming media");

        if let Err(e) = controller.resume().await {
            tracing::error!("Failed to resume media: {e:#}");
        }

        return;
    }

    if controller.has_paused_sessions() {
        return;
    }

    tracing::info!("A bud left the ear, pausing media");

    if let Err(e) = controller.pause().await {
        tracing::error!("Failed to pause media: {e:#}");
    }
}

fn forget(controller: &mut MediaController, reason: &str) {
    if !controller.has_paused_sessions() {
        return;
    }

    tracing::info!("Forgetting paused media sessions ({reason})");
    controller.reset();
}
