use bluetooth::{
    apple_cp::AppleCP, find_connected_device, AdvertisementReceivedData, AdvertisementWatcher,
};
use std::sync::Mutex;
use tauri::{App, Emitter, Manager};

use crate::events;

mod device;

pub use device::Device;

struct DeviceManagerState {
    pub device: Option<Device>,
    _adv_watcher: AdvertisementWatcher,
}

impl DeviceManagerState {
    pub fn is_connected(&self) -> bool {
        self.device.is_some()
    }

    pub fn on_advertisement_received(
        &mut self,
        data: &AdvertisementReceivedData,
        protocol: &AppleCP,
    ) -> bool {
        let device = self.device.as_mut().unwrap();

        device.on_advertisement_received(data, protocol)
    }
}

pub fn init(app: &mut App) {
    let adv_watcher =
        AdvertisementWatcher::new().expect("Failed to initialize AdvertisementWatcher");

    // Get app_handle for the callback
    let app_handle = app.app_handle().clone();
    adv_watcher.on_received(move |data| {
        let device_manager = app_handle.state::<Mutex<DeviceManagerState>>();
        let mut device_manager = device_manager.lock().unwrap();

        if !device_manager.is_connected() {
            tracing::info!("Got advertisement but no device connected");
            return;
        }

        let Some(apple_data) = data.manufacturer_data_map.get(&AppleCP::VENDOR_ID) else {
            tracing::info!("No Apple data found in received advertisement");
            return;
        };

        let Some(protocol) = AppleCP::from_bytes(apple_data) else {
            tracing::info!("Received Apple data is not valid");
            return;
        };

        // TODO: Validate the received data using old data

        let is_updated = device_manager.on_advertisement_received(&data, &protocol);

        if !is_updated {
            return;
        }

        app_handle
            .emit(events::DEVICE_UPDATED, device_manager.device.as_ref())
            .unwrap_or_else(|e| {
                tracing::error!("Failed to emit device connected event: {}", e);
            });
    });

    let device = find_connected_device().map(|info| {
        let name = info
            .Name()
            .map(|name| name.to_string())
            .unwrap_or("Unknown".to_string());

        let address = info.BluetoothAddress().unwrap_or(0);

        Device {
            name,
            address,
            properties: None,
        }
    });

    adv_watcher
        .start()
        .expect("Failed to start AdvertisementWatcher");

    // Store the watcher in the app state to keep it alive
    app.manage(Mutex::new(DeviceManagerState {
        device,
        _adv_watcher: adv_watcher,
    }));
}
