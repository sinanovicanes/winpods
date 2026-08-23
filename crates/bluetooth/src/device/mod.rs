//! Paired bluetooth devices.

mod discovery;

pub use discovery::{DiscoveredDevice, connected_devices};
pub use winpods_core::ConnectionState;

use std::sync::Arc;

use anyhow::{Context, Result};
use tokio::sync::{OnceCell, broadcast};
use windows::{
    Devices::{
        Bluetooth::{BluetoothConnectionStatus, BluetoothDevice},
        Enumeration::DeviceInformation,
    },
    Foundation::TypedEventHandler,
    core::{HSTRING, IInspectable},
};
use winpods_apple_cp::AppleDeviceModel;

use crate::properties::u16_property;

/// Windows device properties carrying the bluetooth identity of a paired device.
pub(crate) const PROPERTY_VENDOR_ID: &str = "System.DeviceInterface.Bluetooth.VendorId";
pub(crate) const PROPERTY_PRODUCT_ID: &str = "System.DeviceInterface.Bluetooth.ProductId";

/// Enough for the connection and name changes of a single device.
const EVENT_CAPACITY: usize = 16;

pub(crate) fn friendly_name(device: &BluetoothDevice) -> Result<String> {
    let name = device
        .DeviceInformation()
        .context("device has no device information")?
        .Name()
        .context("device information has no name")?
        .to_string();

    Ok(name)
}

/// The name to show for a device, falling back to its address.
///
/// A blank entry in the picker would be unselectable, so an unnamed device is listed by address --
/// as a last resort only, since a device listed that way is the bug this used to have.
pub fn display_name(name: Option<String>, address: u64) -> String {
    match name {
        Some(name) if !name.trim().is_empty() => name,
        _ => format_address(address),
    }
}

/// Formats a bluetooth address the way Windows presents it, `"f0:04:e1:c3:d4:e5"`.
pub fn format_address(address: u64) -> String {
    // Addresses are 48 bit, so the two leading bytes of the `u64` are always padding.
    address.to_be_bytes()[2..]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<Vec<_>>()
        .join(":")
}

/// Maps the WinRT connection status onto the shared [`ConnectionState`].
fn connection_state(status: BluetoothConnectionStatus) -> ConnectionState {
    if status == BluetoothConnectionStatus::Connected {
        ConnectionState::Connected
    } else {
        ConnectionState::Disconnected
    }
}

/// Something that changed about a [`Device`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceEvent {
    ConnectionChanged(ConnectionState),
    NameChanged(String),
}

/// The vendor and product ids of a device.
///
/// Both are optional: Windows does not expose them for every paired device, and a device whose
/// product id is missing simply renders as an unknown model rather than failing outright.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DeviceIdentity {
    pub vendor_id: Option<u16>,
    pub product_id: Option<u16>,
}

impl DeviceIdentity {
    /// The Apple model this identity describes, or [`AppleDeviceModel::Unknown`].
    pub fn model(&self) -> AppleDeviceModel {
        self.product_id
            .map(AppleDeviceModel::from_model_id)
            .unwrap_or_default()
    }
}

/// A paired bluetooth device whose connection and name changes are observable.
///
/// Cloning is cheap and shares one underlying WinRT device and one event channel. The WinRT
/// handlers are unregistered once the last clone is dropped -- the previous implementation
/// registered them per instance and never removed any, so every device listing leaked handlers.
#[derive(Clone)]
pub struct Device(Arc<Inner>);

struct Inner {
    device: BluetoothDevice,
    events: broadcast::Sender<DeviceEvent>,
    /// Vendor and product ids never change for a device, so they are fetched at most once.
    identity: OnceCell<DeviceIdentity>,
    connection_token: i64,
    name_token: i64,
}

impl Drop for Inner {
    fn drop(&mut self) {
        let _ = self
            .device
            .RemoveConnectionStatusChanged(self.connection_token);
        let _ = self.device.RemoveNameChanged(self.name_token);
    }
}

impl Device {
    /// Opens the paired device with this bluetooth address.
    pub async fn from_address(address: u64) -> Result<Self> {
        crate::com::ensure_mta();

        let device = BluetoothDevice::FromBluetoothAddressAsync(address)?
            .await
            .with_context(|| format!("no paired bluetooth device at address {address:#014x}"))?;

        Self::wrap(device)
    }

    /// Opens the paired device with this Windows device id.
    pub async fn from_id(id: &HSTRING) -> Result<Self> {
        crate::com::ensure_mta();

        let device = BluetoothDevice::FromIdAsync(id)?
            .await
            .with_context(|| format!("no paired bluetooth device with id `{id}`"))?;

        Self::wrap(device)
    }

    /// Registers the WinRT handlers and returns the wrapped device.
    fn wrap(device: BluetoothDevice) -> Result<Self> {
        let (events, _) = broadcast::channel(EVENT_CAPACITY);

        let handler_events = events.clone();
        let connection_token = device
            .ConnectionStatusChanged(&TypedEventHandler::<BluetoothDevice, IInspectable>::new(
                move |device, _| {
                    let Some(device) = device.as_ref() else {
                        return Ok(());
                    };

                    let state = device
                        .ConnectionStatus()
                        .map(connection_state)
                        .unwrap_or(ConnectionState::Disconnected);

                    let _ = handler_events.send(DeviceEvent::ConnectionChanged(state));
                    Ok(())
                },
            ))
            .context("failed to listen for device connection changes")?;

        let handler_events = events.clone();
        let name_token = device
            .NameChanged(&TypedEventHandler::<BluetoothDevice, IInspectable>::new(
                move |device, _| {
                    let Some(device) = device.as_ref() else {
                        return Ok(());
                    };

                    if let Ok(name) = friendly_name(device) {
                        let _ = handler_events.send(DeviceEvent::NameChanged(name));
                    }

                    Ok(())
                },
            ))
            .context("failed to listen for device name changes")?;

        Ok(Self(Arc::new(Inner {
            device,
            events,
            identity: OnceCell::new(),
            connection_token,
            name_token,
        })))
    }

    /// Subscribes to this device's changes.
    pub fn subscribe(&self) -> broadcast::Receiver<DeviceEvent> {
        self.0.events.subscribe()
    }

    pub fn address(&self) -> Result<u64> {
        Ok(self.0.device.BluetoothAddress()?)
    }

    pub fn id(&self) -> Result<String> {
        Ok(self.0.device.DeviceId()?.to_string())
    }

    pub fn name(&self) -> Result<String> {
        friendly_name(&self.0.device)
    }

    pub fn connection_state(&self) -> ConnectionState {
        self.0
            .device
            .ConnectionStatus()
            .map(connection_state)
            .unwrap_or(ConnectionState::Disconnected)
    }

    pub fn is_connected(&self) -> bool {
        self.connection_state().is_connected()
    }

    /// The device's vendor and product ids.
    ///
    /// Fetched once and cached: reading them costs a WinRT round trip, and the previous
    /// implementation paid one *per property per call*, so a single device listing issued several
    /// queries per device.
    pub async fn identity(&self) -> DeviceIdentity {
        *self
            .0
            .identity
            .get_or_init(|| async {
                match self.load_identity().await {
                    Ok(identity) => identity,
                    Err(e) => {
                        tracing::warn!("Could not read the identity of a bluetooth device: {e:#}");
                        DeviceIdentity::default()
                    }
                }
            })
            .await
    }

    /// The Apple model of this device, or [`AppleDeviceModel::Unknown`].
    pub async fn model(&self) -> AppleDeviceModel {
        self.identity().await.model()
    }

    async fn load_identity(&self) -> Result<DeviceIdentity> {
        let id = self.0.device.DeviceId()?;

        // Scoped so the `IIterable` is dropped before the await; it is not `Send`.
        let operation = {
            let requested = windows_collections::IIterable::<HSTRING>::from(vec![
                HSTRING::from(PROPERTY_VENDOR_ID),
                HSTRING::from(PROPERTY_PRODUCT_ID),
            ]);

            DeviceInformation::CreateFromIdAsyncAdditionalProperties(&id, &requested)?
        };

        let info = operation
            .await
            .context("failed to read device properties")?;
        let properties = info.Properties()?;

        Ok(DeviceIdentity {
            vendor_id: u16_property(&properties, PROPERTY_VENDOR_ID),
            product_id: u16_property(&properties, PROPERTY_PRODUCT_ID),
        })
    }
}

impl std::fmt::Debug for Device {
    /// Only reports what can be read without a WinRT round trip; [`Device::identity`] is async and
    /// therefore unavailable here.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Device")
            .field("address", &self.address().ok())
            .field("name", &self.name().ok())
            .field("connection_state", &self.connection_state())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::{display_name, format_address};

    #[test]
    fn prefers_the_friendly_name() {
        assert_eq!(
            display_name(Some("Anes's AirPods Pro".into()), 0xf004e1c3d4e5),
            "Anes's AirPods Pro"
        );
    }

    #[test]
    fn falls_back_to_the_address_when_unnamed() {
        assert_eq!(display_name(None, 0xf004e1c3d4e5), "f0:04:e1:c3:d4:e5");
        assert_eq!(
            display_name(Some("   ".into()), 0xf004e1c3d4e5),
            "f0:04:e1:c3:d4:e5"
        );
    }

    #[test]
    fn formats_addresses_with_leading_zeroes() {
        assert_eq!(format_address(0x0004e1c3d4e5), "00:04:e1:c3:d4:e5");
    }
}
