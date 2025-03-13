use crate::Device;
use std::collections::HashMap;
use utils::EventDispatcher;
use windows::{Devices::Bluetooth::BluetoothDevice, core::HSTRING};

// Event types
#[derive(Clone)]
struct DeviceAddedEvent(Device);

#[derive(Clone)]
struct DeviceUpdatedEvent(Device);

#[derive(Clone)]
struct DeviceRemovedEvent(Device);

pub struct DeviceManager {
    pub devices: HashMap<String, Device>,
    event_dispatcher: EventDispatcher,
}

impl DeviceManager {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
            event_dispatcher: EventDispatcher::new(),
        }
    }

    pub fn add_device(&mut self, device: Device) {
        self.devices.insert(device.id.clone(), device.clone());
        self.event_dispatcher.dispatch(DeviceAddedEvent(device));
    }

    pub fn remove_device(&mut self, device_id: String) {
        if let Some(device) = self.devices.remove(&device_id) {
            self.event_dispatcher.dispatch(DeviceRemovedEvent(device));
        }
    }

    pub fn update_device(&mut self, device: Device) {
        if let Some(stored_device) = self.devices.get_mut(&device.id) {
            *stored_device = device.clone();
            self.event_dispatcher.dispatch(DeviceUpdatedEvent(device));
        }
    }

    pub fn get_device_by_bluetooth_address(&self, address: u64) -> Option<&Device> {
        for device in self.devices.values() {
            if let Ok(bluetooth_device) = BluetoothDevice::FromIdAsync(&HSTRING::from(&device.id))
                .ok()?
                .get()
            {
                if let Ok(device_address) = bluetooth_device.BluetoothAddress() {
                    if device_address == address {
                        return Some(device);
                    }
                }
            }
        }

        None
    }

    pub fn get_device(&self, device_id: &str) -> Option<&Device> {
        self.devices.get(device_id)
    }

    pub fn get_device_name(&self, device_id: &str) -> Option<String> {
        self.devices
            .get(device_id)
            .map(|device| device.name.clone())
    }

    pub fn on_new_device<F>(&self, callback: F)
    where
        F: Fn(&Device) + Send + Sync + 'static,
    {
        self.event_dispatcher
            .add_listener::<DeviceAddedEvent, _>(move |event| {
                callback(&event.0);
            });
    }

    pub fn on_device_updated<F>(&self, callback: F)
    where
        F: Fn(&Device) + Send + Sync + 'static,
    {
        self.event_dispatcher
            .add_listener::<DeviceUpdatedEvent, _>(move |event| {
                callback(&event.0);
            });
    }

    pub fn on_device_removed<F>(&self, callback: F)
    where
        F: Fn(&Device) + Send + Sync + 'static,
    {
        self.event_dispatcher
            .add_listener::<DeviceRemovedEvent, _>(move |event| {
                callback(&event.0);
            });
    }
}

impl Default for DeviceManager {
    fn default() -> Self {
        Self::new()
    }
}
