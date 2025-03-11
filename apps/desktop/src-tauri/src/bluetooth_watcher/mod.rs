use bluetooth::AdvertisementWatcher;
use device::apple_cp::{self, AirPods};
use tauri::{App, Emitter, Manager};

use crate::{
    events,
    models::{Battery, ConnectedDevice},
};

struct BluetoothWatcher {
    _watcher: AdvertisementWatcher,
}

pub fn init(app: &mut App) {
    let mut watcher = AdvertisementWatcher::new();

    // Get app_handle for the callback
    let app_handle = app.app_handle().clone();

    watcher.on_received(move |data| {
        let Some(apple_data) = data.manufacturer_data_map.get(&apple_cp::VENDOR_ID) else {
            return;
        };

        let Some(airpods) = AirPods::from_bytes(apple_data) else {
            return;
        };

        let device_name =
            bluetooth::get_device_name_by_address(data.address).unwrap_or("Unknown".to_string());

        let left_battery = Battery::new(
            airpods.get_left_battery().unwrap_or(0) * 10,
            airpods.is_left_charging(),
        );

        let right_battery = Battery::new(
            airpods.get_right_battery().unwrap_or(0) * 10,
            airpods.is_right_charging(),
        );

        let connected_device = ConnectedDevice::new(
            device_name,
            airpods.get_model(),
            left_battery,
            right_battery,
        );

        app_handle
            .emit(events::DEVICE_UPDATED, connected_device)
            .unwrap_or_else(|e| {
                tracing::error!("Failed to emit device connected event: {}", e);
            });
    });

    watcher.start();

    // Store the watcher in the app state to keep it alive
    app.manage(BluetoothWatcher { _watcher: watcher });
}
