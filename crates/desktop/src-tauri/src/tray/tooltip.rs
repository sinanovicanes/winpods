//! Keeps the tray tooltip showing the current battery levels.

use std::sync::Arc;

use std::fmt::Write as _;
use tauri::tray::TrayIcon;
use winpods_core::{Battery, DeviceProperties};

use crate::{events::AppEvent, state::AppState};

/// Starts the task that rewrites the tooltip as readings change.
pub fn init(tray: &TrayIcon, state: &Arc<AppState>, app_name: String) {
    let tray = tray.clone();
    let state = Arc::clone(state);

    tauri::async_runtime::spawn(async move {
        let mut events = state.subscribe();

        while let Ok(event) = events.recv().await {
            // Only these change what the tooltip says.
            match event {
                AppEvent::DevicePropertiesUpdated(_)
                | AppEvent::DeviceSelected(_)
                | AppEvent::DeviceNameChanged(_)
                | AppEvent::DeviceSelectionCleared
                | AppEvent::DeviceConnectionChanged(_) => {}
                _ => continue,
            }

            let snapshot = state.devices.snapshot().await;
            let tooltip = match snapshot.device {
                Some(device) => render(&app_name, &device.name, snapshot.properties.as_ref()),
                None => app_name.clone(),
            };

            if let Err(e) = tray.set_tooltip(Some(&tooltip)) {
                tracing::error!("Failed to update the tray tooltip: {e}");
            }
        }
    });
}

/// Renders the tooltip.
///
/// Windows truncates tray tooltips at 127 characters, which is plenty for three lines but does
/// mean the device name goes last so a long name cannot push the batteries out of view.
fn render(app_name: &str, device_name: &str, properties: Option<&DeviceProperties>) -> String {
    let mut tooltip = String::with_capacity(96);
    let _ = writeln!(tooltip, "{app_name}");

    let Some(properties) = properties else {
        let _ = write!(tooltip, "{device_name}");
        return tooltip;
    };

    // Over-ear models are one unit with one battery, so a left/right split would invent a
    // distinction the device does not have.
    if properties.model.is_single_unit() {
        if let Some(level) = battery_line("Battery", properties.overall_battery(), false) {
            let _ = writeln!(tooltip, "{level}");
        }
    } else {
        if let Some(level) = battery_line("Left", properties.left_battery, properties.left_in_ear) {
            let _ = writeln!(tooltip, "{level}");
        }

        if let Some(level) =
            battery_line("Right", properties.right_battery, properties.right_in_ear)
        {
            let _ = writeln!(tooltip, "{level}");
        }

        if let Some(level) = battery_line("Case", properties.case_battery, false) {
            let _ = writeln!(tooltip, "{level}");
        }
    }

    let _ = write!(tooltip, "{device_name}");

    tooltip
}

fn battery_line(label: &str, battery: Option<Battery>, in_ear: bool) -> Option<String> {
    let battery = battery?;
    let mut line = format!("{label}: {}%", battery.level);

    if battery.charging {
        line.push_str(" ⚡");
    }

    if in_ear {
        line.push_str(" 👂");
    }

    Some(line)
}

#[cfg(test)]
mod tests {
    use super::*;
    use winpods_apple_cp::AppleDeviceModel;

    fn properties() -> DeviceProperties {
        DeviceProperties {
            rssi: -50,
            address: 1,
            model: AppleDeviceModel::AirPodsPro2,
            left_battery: Some(Battery::new(80, false)),
            right_battery: Some(Battery::new(70, true)),
            case_battery: Some(Battery::new(50, false)),
            left_in_ear: true,
            right_in_ear: false,
        }
    }

    #[test]
    fn renders_every_reported_battery() {
        let tooltip = render("winpods", "My AirPods", Some(&properties()));

        assert!(tooltip.starts_with("winpods\n"));
        assert!(tooltip.contains("Left: 80% 👂"));
        assert!(tooltip.contains("Right: 70% ⚡"));
        assert!(tooltip.contains("Case: 50%"));
        assert!(tooltip.ends_with("My AirPods"));
    }

    #[test]
    fn omits_batteries_that_reported_nothing() {
        let mut props = properties();
        props.case_battery = None;
        props.left_battery = None;

        let tooltip = render("winpods", "My AirPods", Some(&props));
        assert!(!tooltip.contains("Case"));
        assert!(!tooltip.contains("Left"));
        assert!(tooltip.contains("Right"));
    }

    #[test]
    fn falls_back_to_the_device_name_without_readings() {
        let tooltip = render("winpods", "My AirPods", None);
        assert_eq!(tooltip, "winpods\nMy AirPods");
    }

    #[test]
    fn single_unit_models_report_one_battery() {
        let props = DeviceProperties {
            model: AppleDeviceModel::AirPodsMaxUsbC,
            left_battery: Some(Battery::new(60, false)),
            right_battery: Some(Battery::new(60, false)),
            case_battery: None,
            ..properties()
        };

        let tooltip = render("winpods", "My AirPods Max", Some(&props));
        assert!(tooltip.contains("Battery: 60%"), "got {tooltip:?}");
        assert!(
            !tooltip.contains("Left"),
            "AirPods Max have no left/right split"
        );
        assert!(!tooltip.contains("Right"));
        assert!(!tooltip.contains("Case"));
    }

    #[test]
    fn stays_within_the_windows_tooltip_limit() {
        // Windows silently truncates at 127 characters.
        let tooltip = render("winpods", &"A".repeat(60), Some(&properties()));
        assert!(
            tooltip.chars().count() <= 127,
            "got {} chars",
            tooltip.chars().count()
        );
    }
}
