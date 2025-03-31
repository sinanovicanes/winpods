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

use crate::{
    Error, Result,
    apple_cp::{AppleDeviceExt, AppleDeviceModel},
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
    pub fn from_bluetooth_address(address: u64) -> Result<Self> {
        let device = BluetoothDevice::FromBluetoothAddressAsync(address)
            .map_err(|_| Error::DeviceNotFound)?
            .get()
            .map_err(|_| Error::DeviceNotFound)?;
        let dispatcher = EventDispatcher::new();
        let device = Self { device, dispatcher };

        device.init();

        Ok(device)
    }

    pub fn from_device_id(device_id: impl Into<HSTRING>) -> Result<Self> {
        let device = BluetoothDevice::FromIdAsync(&device_id.into())
            .map_err(|_| Error::DeviceNotFound)?
            .get()
            .map_err(|_| Error::DeviceNotFound)?;
        let dispatcher = EventDispatcher::new();
        let device = Self { device, dispatcher };

        device.init();

        Ok(device)
    }
}

impl Device {
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
        let _ = self
            .device
            .NameChanged(&TypedEventHandler::<BluetoothDevice, IInspectable>::new(
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

    pub fn on_connection_changed(
        &self,
        callback: impl Fn(DeviceConnectionState) + Send + Sync + 'static,
    ) {
        self.dispatcher
            .add_listener::<DeviceConnectionChangedEvent, _>(move |event| {
                callback(event.0);
            });
    }

    pub fn on_name_changed(&self, callback: impl Fn(&String) + Send + Sync + 'static) {
        self.dispatcher
            .add_listener::<DeviceNameChangedEvent, _>(move |event| {
                callback(&event.0);
            });
    }
}

impl Device {
    pub fn get_device_id(&self) -> Result<String> {
        let device_id = self
            .device
            .DeviceId()
            .map_err(|_| Error::PropertyNotFound)?;
        Ok(device_id.to_string())
    }

    pub fn get_name(&self) -> Result<String> {
        let name = self
            .device
            .DeviceInformation()
            .map_err(|_| Error::DeviceNotFound)?
            .Name()
            .map_err(|_| Error::PropertyNotFound)?;
        Ok(name.to_string())
    }

    pub fn get_address(&self) -> Result<u64> {
        let address = self
            .device
            .BluetoothAddress()
            .map_err(|_| Error::PropertyNotFound)?;
        Ok(address)
    }

    pub fn get_info(&self) -> Result<DeviceInformation> {
        let properties = vec![
            HSTRING::from(PROPERTY_BLUETOOTH_VENDOR_ID),
            HSTRING::from(PROPERTY_BLUETOOTH_PRODUCT_ID),
            HSTRING::from(PROPERTY_AEP_CONTAINER_ID),
        ];

        let properties = windows_collections::IIterable::from(properties);
        let info = DeviceInformation::CreateFromIdAsyncAdditionalProperties(
            &self.device.DeviceId().map_err(|_| Error::DeviceNotFound)?,
            &properties,
        )
        .map_err(|_| Error::DeviceNotFound)?
        .get()
        .map_err(|_| Error::DeviceNotFound)?;

        Ok(info)
    }

    pub fn get_vendor_id(&self) -> Result<u16> {
        let properties = self
            .get_info()?
            .Properties()
            .map_err(|_| Error::PropertyNotFound)?;
        let vendor_id = properties
            .Lookup(&HSTRING::from(PROPERTY_BLUETOOTH_VENDOR_ID))
            .map_err(|_| Error::PropertyNotFound)?
            .cast::<IPropertyValue>()
            .map_err(|_| Error::PropertyNotFound)?
            .GetUInt16()
            .map_err(|_| Error::PropertyNotFound)?;
        Ok(vendor_id)
    }

    pub fn get_product_id(&self) -> Result<u16> {
        let properties = self
            .get_info()?
            .Properties()
            .map_err(|_| Error::PropertyNotFound)?;
        let product_id = properties
            .Lookup(&HSTRING::from(PROPERTY_BLUETOOTH_PRODUCT_ID))
            .map_err(|_| Error::PropertyNotFound)?
            .cast::<IPropertyValue>()
            .map_err(|_| Error::PropertyNotFound)?
            .GetUInt16()
            .map_err(|_| Error::PropertyNotFound)?;
        Ok(product_id)
    }

    pub fn get_aep_id(&self) -> Result<u16> {
        let properties = self
            .get_info()?
            .Properties()
            .map_err(|_| Error::PropertyNotFound)?;
        let product_id = properties
            .Lookup(&HSTRING::from(PROPERTY_AEP_CONTAINER_ID))
            .map_err(|_| Error::PropertyNotFound)?
            .cast::<IPropertyValue>()
            .map_err(|_| Error::PropertyNotFound)?
            .GetUInt16()
            .map_err(|_| Error::PropertyNotFound)?;
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
}

impl TryFrom<BluetoothDevice> for Device {
    type Error = Error;

    fn try_from(value: BluetoothDevice) -> Result<Self> {
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
    type Error = Error;

    fn try_from(value: DeviceInformation) -> Result<Self> {
        let device_id = value.Id().map_err(|_| Error::DeviceNotFound)?;
        Self::from_device_id(device_id)
    }
}

impl Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Device")
            .field("device_id", &self.get_device_id())
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
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Device", 6)?;
        state.serialize_field("name", &self.get_name().unwrap_or("Unknown".to_string()))?;
        state.serialize_field("address", &self.get_address().unwrap_or(0))?;
        state.end()
    }
}

impl AppleDeviceExt for Device {
    fn get_device_model(&self) -> AppleDeviceModel {
        match self.get_product_id() {
            Ok(product_id) => AppleDeviceModel::from_model_id(product_id),
            Err(_) => AppleDeviceModel::Unknown,
        }
    }
}
