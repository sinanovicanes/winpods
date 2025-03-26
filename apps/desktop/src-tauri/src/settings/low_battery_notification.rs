use std::sync::{Mutex, RwLock};

use tauri::{App, AppHandle, Listener, Manager};
use tauri_plugin_notification::NotificationExt;

use crate::{device_manager::DeviceProperties, events};

use super::SettingsState;

#[derive(Debug, Clone, Default)]
struct LowBatteryNotificationState {
    pub sended: bool,
}

fn send_low_battery_notification(app_handle: &AppHandle) {
    let result = app_handle
        .notification()
        .builder()
        .title("Windpods - Low battery")
        .body("Your device battery is low")
        .show();

    match result {
        Ok(_) => tracing::info!("Low battery notification sended"),
        Err(e) => tracing::error!("Failed to send low battery notification: {:?}", e),
    }
}

pub fn init(app: &mut App) {
    let app_handle = app.app_handle().clone();
    app.listen(events::DEVICE_PROPERTIES_UPDATED, move |event| {
        let settings_state = app_handle.state::<RwLock<SettingsState>>();
        let settings_state = settings_state.read().unwrap();

        // Check if low battery notification is enabled
        if settings_state.low_battery_threshold > 0 {
            return;
        }

        let Ok(properties) = serde_json::from_str::<DeviceProperties>(event.payload()) else {
            tracing::warn!("Failed to parse device properties");
            return;
        };

        let low_battery_notification_state =
            app_handle.state::<Mutex<LowBatteryNotificationState>>();
        let mut low_battery_notification_state = low_battery_notification_state.lock().unwrap();

        let battery_threshold = settings_state.low_battery_threshold;

        if !properties.left_battery.charging && properties.left_battery.level <= battery_threshold
            || !properties.right_battery.charging
                && properties.right_battery.level <= battery_threshold
        {
            if !low_battery_notification_state.sended {
                tracing::info!("Sending low battery notification");
                send_low_battery_notification(&app_handle);
                low_battery_notification_state.sended = true;
            }
        } else {
            low_battery_notification_state.sended = false;
        }
    });

    app.manage(Mutex::new(LowBatteryNotificationState::default()));
}
