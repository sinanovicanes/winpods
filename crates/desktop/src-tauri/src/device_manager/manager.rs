use bluetooth::{AdapterWatcher, AdvertisementWatcher, Device, DeviceConnectionState};
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
    pub adapter_watcher: AdapterWatcher,
    dispatcher: EventDispatcher,
}

impl DeviceManagerState {
    pub fn new() -> Self {
        let adv_watcher =
            AdvertisementWatcher::new().expect("Failed to initialize AdvertisementWatcher");
        let adapter_watcher = AdapterWatcher::new();

        Self {
            device: None,
            device_properties: None,
            adv_watcher,
            adapter_watcher,
            dispatcher: EventDispatcher::new(),
        }
    }

    pub fn select_device(&mut self, device: Device) {
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

    pub fn clear_device_selection(&mut self) {
        self.device = None;
        self.device_properties = None;
        self.dispatcher.dispatch(DeviceDisconnectedEvent);
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
