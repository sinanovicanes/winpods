use std::fmt::Debug;

use serde::{Serialize, ser::SerializeStruct};
use utils::EventDispatcher;
use windows::{
    Devices::{
        Bluetooth::{BluetoothConnectionStatus, BluetoothDevice},
        Enumeration::DeviceInformation,
    },
    Foundation::{IPropertyValue, TypedEventHandler},
    core::{HSTRING, IInspectable, Interface},
};

const PROPERTY_BLUETOOTH_VENDOR_ID: &str = "System.DeviceInterface.Bluetooth.VendorId";
const PROPERTY_BLUETOOTH_PRODUCT_ID: &str = "System.DeviceInterface.Bluetooth.ProductId";
const PROPERTY_AEP_CONTAINER_ID: &str = "System.Devices.Aep.ContainerId";

struct DeviceConnectionChangedEvent(DeviceConnectionState);
struct DeviceNameChangedEvent(String);

#[derive(Clone)]
pub struct Device {
    device: BluetoothDevice,
    dispatcher: EventDispatcher,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub enum DeviceConnectionState {
    Connected,
    Disconnected,
}

impl Device {
    pub fn from_bluetooth_address(address: u64) -> windows::core::Result<Self> {
        let device = BluetoothDevice::FromBluetoothAddressAsync(address)?.get()?;
        let dispatcher = EventDispatcher::new();
        let device = Self { device, dispatcher };

        device.init();

        Ok(device)
    }

    fn init(&self) {
        let dispatcher = self.dispatcher.clone();
        let _ = self.device.ConnectionStatusChanged(&TypedEventHandler::<
            BluetoothDevice,
            IInspectable,
        >::new(
            move |device, _inspactable| {
                let Some(device) = device.as_ref() else {
                    return Ok(());
                };

                let state = match device.ConnectionStatus() {
                    Ok(BluetoothConnectionStatus::Connected) => DeviceConnectionState::Connected,
                    _ => DeviceConnectionState::Disconnected,
                };

                dispatcher.dispatch(DeviceConnectionChangedEvent(state));
                Ok(())
            },
        ));

        let dispatcher = self.dispatcher.clone();
        let _ = self.device.ConnectionStatusChanged(&TypedEventHandler::<
            BluetoothDevice,
            IInspectable,
        >::new(
            move |device, _inspactable| {
                let Some(device) = device.as_ref() else {
                    return Ok(());
                };

                let device_name = device.Name()?;
                dispatcher.dispatch(DeviceNameChangedEvent(device_name.to_string()));
                Ok(())
            },
        ));
    }

    pub fn get_name(&self) -> windows::core::Result<String> {
        let name = self.device.Name()?;
        Ok(name.to_string())
    }

    pub fn get_address(&self) -> windows::core::Result<u64> {
        let address = self.device.BluetoothAddress()?;
        Ok(address)
    }

    pub fn get_info(&self) -> windows::core::Result<DeviceInformation> {
        let properties = vec![
            HSTRING::from(PROPERTY_BLUETOOTH_VENDOR_ID),
            HSTRING::from(PROPERTY_BLUETOOTH_PRODUCT_ID),
            HSTRING::from(PROPERTY_AEP_CONTAINER_ID),
        ];

        let properties = windows_collections::IIterable::from(properties);
        let info = DeviceInformation::CreateFromIdAsyncAdditionalProperties(
            &self.device.DeviceId()?,
            &properties,
        )?
        .get()?;

        Ok(info)
    }

    pub fn get_vendor_id(&self) -> windows::core::Result<u16> {
        let properties = self.get_info()?.Properties()?;
        let vendor_id = properties
            .Lookup(&HSTRING::from(PROPERTY_BLUETOOTH_VENDOR_ID))?
            .cast::<IPropertyValue>()?
            .GetUInt16()?;
        Ok(vendor_id)
    }

    pub fn get_product_id(&self) -> windows::core::Result<u16> {
        let properties = self.get_info()?.Properties()?;
        let product_id = properties
            .Lookup(&HSTRING::from(PROPERTY_BLUETOOTH_PRODUCT_ID))?
            .cast::<IPropertyValue>()?
            .GetUInt16()?;
        Ok(product_id)
    }

    pub fn get_aep_id(&self) -> windows::core::Result<u16> {
        let properties = self.get_info()?.Properties()?;
        let product_id = properties
            .Lookup(&HSTRING::from(PROPERTY_AEP_CONTAINER_ID))?
            .cast::<IPropertyValue>()?
            .GetUInt16()?;
        Ok(product_id)
    }

    pub fn get_connection_state(&self) -> DeviceConnectionState {
        match self.device.ConnectionStatus() {
            Ok(BluetoothConnectionStatus::Connected) => DeviceConnectionState::Connected,
            _ => DeviceConnectionState::Disconnected,
        }
    }

    pub fn is_connected(&self) -> bool {
        matches!(
            self.get_connection_state(),
            DeviceConnectionState::Connected
        )
    }

    pub fn on_connection_changed(
        &self,
        callback: impl Fn(DeviceConnectionState) + Send + Sync + 'static,
    ) {
        self.dispatcher
            .add_listener::<DeviceConnectionChangedEvent, _>(move |event| {
                callback(event.0);
            });
    }

    pub fn on_name_changed(&self, callback: impl Fn(String) + Send + Sync + 'static) {
        self.dispatcher
            .add_listener::<DeviceNameChangedEvent, _>(move |event| {
                callback(event.0.clone());
            });
    }
}

impl TryFrom<BluetoothDevice> for Device {
    type Error = windows::core::Error;

    fn try_from(value: BluetoothDevice) -> Result<Self, Self::Error> {
        let dispatcher = EventDispatcher::new();
        let device = Self {
            device: value,
            dispatcher,
        };

        device.init();

        Ok(device)
    }
}

impl TryFrom<DeviceInformation> for Device {
    type Error = windows::core::Error;

    fn try_from(value: DeviceInformation) -> Result<Self, Self::Error> {
        let bluetooth_device = BluetoothDevice::FromIdAsync(&value.Id()?)?.get()?;
        let dispatcher = EventDispatcher::new();
        let device = Self {
            device: bluetooth_device,
            dispatcher,
        };

        device.init();

        Ok(device)
    }
}

impl Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Device")
            .field("name", &self.get_name())
            .field("address", &self.get_address())
            .field("vendor_id", &self.get_vendor_id())
            .field("product_id", &self.get_product_id())
            .field("aep_id", &self.get_aep_id())
            .field("connection_state", &self.get_connection_state())
            .finish()
    }
}

impl Serialize for Device {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Device", 6)?;
        state.serialize_field("name", &self.get_name().unwrap_or("Unknown".to_string()))?;
        state.serialize_field("address", &self.get_address().unwrap_or(0))?;
        state.end()
    }
}
