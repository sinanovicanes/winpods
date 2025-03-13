use tauri::{tray::TrayIcon, Listener};

use crate::models::ConnectedDevice;

pub fn init_tooltip_listener(tray: &TrayIcon) {
    let app_handle = tray.app_handle();
    let app_name = crate::utils::get_app_name_by_handle(app_handle);
    let tray_handle = tray.clone();

    app_handle.listen("device-updated", move |event| {
        if let Ok(device) = serde_json::from_str::<ConnectedDevice>(event.payload()) {
            let tooltip = format!("{}\n{}", app_name, device.to_tooltip());
            let _ = tray_handle.set_tooltip(Some(tooltip));
        }
    });
}

impl ConnectedDevice {
    fn to_tooltip(&self) -> String {
        format!(
            "{}\nLeft: {}% {}\nRight: {}% {}",
            self.name,
            self.left_battery.level,
            if self.left_battery.charging {
                "⚡"
            } else {
                ""
            },
            self.right_battery.level,
            if self.right_battery.charging {
                "⚡"
            } else {
                ""
            }
        )
    }
}
