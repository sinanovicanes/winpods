use bluetooth::{
    apple_cp::ProximityPairingMessage, AdvertisementReceivedData, AdvertisementWatcher,
};
use device::Device;
use utils::EventDispatcher;

use super::selected_device::SelectedDevice;

struct DeviceConnectedEvent(SelectedDevice);
struct DeviceUpdatedEvent(SelectedDevice);
struct DeviceDisconnectedEvent;

pub struct DeviceManagerState {
    pub device: Option<SelectedDevice>,
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
        let selected_device = SelectedDevice::new(device);
        self.device = Some(selected_device.clone());
        self.dispatcher
            .dispatch(DeviceConnectedEvent(selected_device));
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
        let device = self.device.as_mut().unwrap();
        device.on_advertisement_received(data, protocol)
    }

    pub fn dispatch_device_updated(&self) {
        if let Some(device) = &self.device {
            self.dispatcher.dispatch(DeviceUpdatedEvent(device.clone()));
        }
    }

    pub fn on_device_connected(&self, callback: impl Fn(&SelectedDevice) + Send + Sync + 'static) {
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

    pub fn on_device_updated(&self, callback: impl Fn(&SelectedDevice) + Send + Sync + 'static) {
        self.dispatcher
            .add_listener::<DeviceUpdatedEvent, _>(move |event| {
                callback(&event.0);
            });
    }
}
