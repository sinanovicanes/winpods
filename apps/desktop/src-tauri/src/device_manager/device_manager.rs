use bluetooth::{
    apple_cp::ProximityPairingMessage, AdvertisementReceivedData, AdvertisementWatcher,
};
use utils::EventDispatcher;

use super::Device;

struct DeviceConnectedEvent(Device);
struct DeviceUpdatedEvent(Device);
struct DeviceDisconnectedEvent;

pub struct DeviceManagerState {
    pub device: Option<Device>,
    pub adv_watcher: AdvertisementWatcher,
    dispatcher: EventDispatcher,
}

impl DeviceManagerState {
    pub fn new() -> Self {
        let adv_watcher =
            AdvertisementWatcher::new().expect("Failed to initialize AdvertisementWatcher");

        Self {
            device: None,
            adv_watcher,
            dispatcher: EventDispatcher::new(),
        }
    }

    pub fn is_connected(&self) -> bool {
        self.device.is_some()
    }

    pub fn connect(&mut self, device: Device) {
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
    ) {
        let device = self.device.as_mut().unwrap();
        let is_device_updated = device.on_advertisement_received(data, protocol);

        if is_device_updated {
            self.dispatcher.dispatch(DeviceUpdatedEvent(device.clone()));
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

    pub fn on_device_updated(&self, callback: impl Fn(&Device) + Send + Sync + 'static) {
        self.dispatcher
            .add_listener::<DeviceUpdatedEvent, _>(move |event| {
                callback(&event.0);
            });
    }
}
