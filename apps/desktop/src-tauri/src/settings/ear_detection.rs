use std::sync::{Mutex, RwLock};

use media::GlobalMediaController;
use tauri::{App, Listener, Manager};

use crate::{device_manager::DeviceProperties, events};

use super::SettingsState;

#[derive(Debug, Clone, Default)]
struct EarDetectionState {
    pub paused: bool,
    media_controller: GlobalMediaController,
}

pub fn init(app: &mut App) {
    let app_handle = app.app_handle().clone();
    app.listen(events::DEVICE_PROPERTIES_UPDATED, move |event| {
        let settings_state = app_handle.state::<RwLock<SettingsState>>();
        let settings_state = settings_state.read().unwrap();

        // Check if ear detection is enabled
        if !settings_state.ear_detection {
            return;
        }

        let Ok(properties) = serde_json::from_str::<DeviceProperties>(event.payload()) else {
            return;
        };

        let ear_detection_state = app_handle.state::<Mutex<EarDetectionState>>();
        let mut ear_detection_state = ear_detection_state.lock().unwrap();
        let both_in_ear = properties.left_in_ear && properties.right_in_ear;

        if both_in_ear {
            if !ear_detection_state.paused {
                return;
            }

            tracing::info!("Both pods are in ear, resuming media");
            match ear_detection_state.media_controller.resume() {
                Ok(_) => ear_detection_state.paused = false,
                Err(_) => tracing::error!("Failed to play media"),
            }

            return;
        }

        if ear_detection_state.paused {
            return;
        }

        tracing::info!("One or both pods are out of ear, pausing media");
        match ear_detection_state.media_controller.pause() {
            Ok(_) => ear_detection_state.paused = true,
            Err(_) => tracing::error!("Failed to pause media"),
        }
    });

    app.manage(Mutex::new(EarDetectionState::default()));
}
