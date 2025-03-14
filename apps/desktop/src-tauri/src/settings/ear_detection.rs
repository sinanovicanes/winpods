use std::sync::Mutex;

use tauri::{App, Listener, Manager};

use crate::{device_manager::Device, events};

use super::SettingsState;

#[derive(Debug, Clone, Default)]
struct EarDetectionState {
    pub stopped: bool,
}

pub fn init(app: &mut App) {
    let app_handle = app.app_handle().clone();
    app.listen(events::DEVICE_UPDATED, move |event| {
        let settings_state = app_handle.state::<Mutex<SettingsState>>();
        let settings_state = settings_state.lock().unwrap();

        // Check if ear detection is enabled
        if !settings_state.ear_detection {
            return;
        }

        let Ok(device) = serde_json::from_str::<Device>(event.payload()) else {
            tracing::error!("Failed to parse device from event payload");
            return;
        };

        let Some(properties) = device.properties else {
            tracing::warn!("Device properties not found");
            return;
        };

        let ear_detection_state = app_handle.state::<Mutex<EarDetectionState>>();
        let mut ear_detection_state = ear_detection_state.lock().unwrap();

        if properties.left_in_ear || properties.right_in_ear {
            if !ear_detection_state.stopped {
                return;
            }

            if media::play() {
                ear_detection_state.stopped = false;
            }

            return;
        }

        if ear_detection_state.stopped {
            return;
        }

        if media::stop() {
            ear_detection_state.stopped = true;
        }
    });

    app.manage(Mutex::new(EarDetectionState::default()));
}
