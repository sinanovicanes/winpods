use std::sync::Mutex;

use tauri::{App, AppHandle, Listener, Manager};
use tauri_plugin_notification::NotificationExt;

use crate::{device_manager::Device, events};

use super::SettingsState;

#[derive(Debug, Clone, Default)]
struct LowBatteryNotificationState {
    pub sended: bool,
}

// TODO: Move to settings
const BATTERY_TRESHOLD: u8 = 20;

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
    app.listen(events::DEVICE_UPDATED, move |event| {
        let settings_state = app_handle.state::<Mutex<SettingsState>>();
        let settings_state = settings_state.lock().unwrap();

        // Check if low battery notification is enabled
        if !settings_state.low_battery_notification {
            return;
        }

        let low_battery_notification_state =
            app_handle.state::<Mutex<LowBatteryNotificationState>>();
        let mut low_battery_notification_state = low_battery_notification_state.lock().unwrap();

        let Ok(device) = serde_json::from_str::<Device>(event.payload()) else {
            tracing::error!("Failed to parse device from event payload");
            return;
        };

        let Some(properties) = device.properties else {
            tracing::warn!("Device properties not found");
            return;
        };

        if !properties.left_battery.charging && properties.left_battery.level <= BATTERY_TRESHOLD
            || !properties.right_battery.charging
                && properties.right_battery.level <= BATTERY_TRESHOLD
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
