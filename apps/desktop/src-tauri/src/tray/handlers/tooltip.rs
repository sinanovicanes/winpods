use crate::{device_manager::Device, events};
use tauri::{tray::TrayIcon, Listener};

pub fn init_tooltip_listener(tray: &TrayIcon) {
    let app_handle = tray.app_handle();

    let app_name = crate::utils::get_app_name_by_handle(app_handle);
    let tray_handle = tray.clone();
    app_handle.listen(events::DEVICE_UPDATED, move |event| {
        if let Ok(device) = serde_json::from_str::<Device>(event.payload()) {
            let tooltip = format!("{}\n{}", app_name, device.to_tooltip());
            let _ = tray_handle.set_tooltip(Some(tooltip));
        }
    });

    let tray_handle = tray.clone();
    let app_name = crate::utils::get_app_name_by_handle(app_handle);
    app_handle.listen(events::DEVICE_DISCONNECTED, move |_event| {
        let _ = tray_handle.set_tooltip(Some(&app_name));
    });
}

impl Device {
    fn to_tooltip(&self) -> String {
        if let Some(properties) = &self.properties {
            format!(
                "{}\nLeft: {}%\nRight: {}%",
                self.name, properties.left_battery.level, properties.right_battery.level
            )
        } else {
            "No device connected".to_string()
        }
    }
}
