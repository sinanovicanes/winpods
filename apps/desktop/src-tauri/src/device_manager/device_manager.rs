use bluetooth::{
    apple_cp::{AppleDeviceModel, ProximityPairingMessage},
    AdvertisementReceivedData, AdvertisementWatcher, Device, DeviceConnectionState,
};
use utils::EventDispatcher;

use crate::tray::Tooltip;

use super::DeviceProperties;

struct DeviceConnectedEvent(Device);
struct DevicePropertiesUpdatedEvent(DeviceProperties);
struct DeviceDisconnectedEvent;
struct DeviceNameUpdatedEvent(String);
struct DeviceConnectionUpdatedEvent(DeviceConnectionState);

pub struct DeviceManagerState {
    pub device: Option<Device>,
    pub device_properties: Option<DeviceProperties>,
    pub adv_watcher: AdvertisementWatcher,
    dispatcher: EventDispatcher,
}

impl DeviceManagerState {
    pub fn new() -> Self {
        let adv_watcher =
            AdvertisementWatcher::new().expect("Failed to initialize AdvertisementWatcher");

        Self {
            device: None,
            device_properties: None,
            adv_watcher,
            dispatcher: EventDispatcher::new(),
        }
    }

    pub fn is_connected(&self) -> bool {
        self.device.is_some()
    }

    pub fn connect(&mut self, device: Device) {
        let dispatcher = self.dispatcher.clone();
        device.on_name_changed(move |name| {
            dispatcher.dispatch(DeviceNameUpdatedEvent(name.clone()));
        });

        let dispatcher = self.dispatcher.clone();
        device.on_connection_changed(move |state| {
            dispatcher.dispatch(DeviceConnectionUpdatedEvent(state));
        });

        self.device = Some(device.clone());
        self.dispatcher.dispatch(DeviceConnectedEvent(device));
    }

    pub fn disconnect(&mut self) {
        self.device = None;
        self.dispatcher.dispatch(DeviceDisconnectedEvent);
    }

    pub fn on_advertisement_received(
        &mut self,
        data: &AdvertisementReceivedData,
        protocol: &ProximityPairingMessage,
    ) -> bool {
        let Some(device) = &self.device else {
            return false;
        };

        let new_properties = DeviceProperties::from_advertisement(data, protocol);

        if device.get_device_model() != new_properties.model {
            return false;
        }

        if let Some(properties) = &self.device_properties {
            if !properties.is_within_update_limits(&new_properties) {
                return false;
            }
        }

        self.device_properties = Some(new_properties);
        true
    }

    pub fn dispatch_device_updated(&self) {
        if let Some(properties) = &self.device_properties {
            self.dispatcher
                .dispatch(DevicePropertiesUpdatedEvent(properties.clone()));
        }
    }

    pub fn on_device_connected(&self, callback: impl Fn(&Device) + Send + Sync + 'static) {
        self.dispatcher
            .add_listener::<DeviceConnectedEvent, _>(move |event| {
                callback(&event.0);
            });
    }

    pub fn on_device_disconnected(&self, callback: impl Fn() + Send + Sync + 'static) {
        self.dispatcher
            .add_listener::<DeviceDisconnectedEvent, _>(move |_event| {
                callback();
            });
    }

    pub fn on_device_connection_changed(
        &self,
        callback: impl Fn(&DeviceConnectionState) + Send + Sync + 'static,
    ) {
        self.dispatcher
            .add_listener::<DeviceConnectionUpdatedEvent, _>(move |event| {
                callback(&event.0);
            });
    }

    pub fn on_device_name_changed(&self, callback: impl Fn(&String) + Send + Sync + 'static) {
        self.dispatcher
            .add_listener::<DeviceNameUpdatedEvent, _>(move |event| {
                callback(&event.0);
            });
    }

    pub fn on_device_properties_updated(
        &self,
        callback: impl Fn(&DeviceProperties) + Send + Sync + 'static,
    ) {
        self.dispatcher
            .add_listener::<DevicePropertiesUpdatedEvent, _>(move |event| {
                callback(&event.0);
            });
    }
}

trait AppleDevice {
    fn get_device_model(&self) -> AppleDeviceModel;
}

impl AppleDevice for Device {
    fn get_device_model(&self) -> AppleDeviceModel {
        match self.get_product_id() {
            Ok(product_id) => AppleDeviceModel::from_model_id(product_id),
            Err(_) => AppleDeviceModel::Unknown,
        }
    }
}

impl Tooltip for DeviceManagerState {
    fn to_tooltip(&self) -> String {
        let Some(device) = &self.device else {
            return String::new();
        };

        let mut tooltip = format!("{}\n", device.get_name().unwrap_or("Connected".to_string()));

        if let Some(properties) = &self.device_properties {
            tooltip.push_str(&properties.to_tooltip());
        }

        tooltip
    }
}
