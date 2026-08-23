//! Enumerating the currently connected bluetooth devices.

use anyhow::{Context, Result};
use windows::{
    Devices::{
        Bluetooth::{BluetoothConnectionStatus, BluetoothDevice},
        Enumeration::DeviceInformation,
    },
    core::HSTRING,
};
use winpods_apple_cp::AppleDeviceModel;

use super::{Device, PROPERTY_PRODUCT_ID, PROPERTY_VENDOR_ID, display_name, friendly_name};
use crate::properties::{string_property, u16_property};

/// Windows exposes the bluetooth MAC as a hex string on the device interface. It is not needed for
/// the result -- the opened device reports its own address -- but it groups the interfaces of one
/// device together, so only one of them has to be opened.
const PROPERTY_ADDRESS: &str = "System.DeviceInterface.Bluetooth.DeviceAddress";

/// A connected bluetooth device as reported by device enumeration.
///
/// This is deliberately inert: no WinRT object, no event handlers. Listing devices for a picker
/// should not register handlers, which is what building a full [`Device`] per result used to do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveredDevice {
    pub address: u64,
    pub name: String,
    pub vendor_id: Option<u16>,
    pub product_id: Option<u16>,
}

impl DiscoveredDevice {
    /// The Apple model of this device, or [`AppleDeviceModel::Unknown`].
    pub fn model(&self) -> AppleDeviceModel {
        self.product_id
            .map(AppleDeviceModel::from_model_id)
            .unwrap_or_default()
    }

    /// Opens the full [`Device`] for this entry.
    pub async fn open(&self) -> Result<Device> {
        Device::from_address(self.address).await
    }
}

/// Lists every currently connected bluetooth device, once each.
///
/// The query asks for the vendor id, product id and address alongside the device itself, so those
/// cost no round trips of their own; the previous implementation ran one query for the device list
/// and then several more *per device* to read the same properties.
///
/// Reading happens in two phases on purpose. WinRT collection iterators are not `Send`, and Tauri
/// requires command futures to be `Send`, so the iterator must not be alive across an `await`.
/// Phase one drains the collection with no awaits at all; phase two opens the devices.
pub async fn connected_devices() -> Result<Vec<DiscoveredDevice>> {
    crate::com::ensure_mta();

    let filter = BluetoothDevice::GetDeviceSelectorFromConnectionStatus(
        BluetoothConnectionStatus::Connected,
    )
    .context("failed to build the connected device selector")?;

    // The `IIterable` is scoped so it is dropped before the await: like the collection
    // iterators, it is not `Send`, and holding one across an await poisons the whole future.
    let operation = {
        let requested = windows_collections::IIterable::<HSTRING>::from(vec![
            HSTRING::from(PROPERTY_VENDOR_ID),
            HSTRING::from(PROPERTY_PRODUCT_ID),
            HSTRING::from(PROPERTY_ADDRESS),
        ]);

        DeviceInformation::FindAllAsyncAqsFilterAndAdditionalProperties(&filter, &requested)?
    };

    let found = operation
        .await
        .context("failed to enumerate the connected bluetooth devices")?;

    // Phase one: fully synchronous, so the collection is dropped before the first await.
    let mut interfaces: Vec<Interface> = Vec::new();

    for info in found {
        match read_interface(&info) {
            Ok(interface) => collect_interface(&mut interfaces, interface),
            Err(e) => tracing::debug!("Skipped a connected bluetooth device: {e:#}"),
        }
    }

    // Phase two: open each device, which is the only way to reach its friendly name.
    let mut devices: Vec<DiscoveredDevice> = Vec::new();

    for interface in interfaces {
        match open_interface(&interface).await {
            Ok(device) => collect_device(&mut devices, device),
            Err(e) => tracing::debug!(
                "Skipped the bluetooth device behind `{}`: {e:#}",
                interface.id
            ),
        }
    }

    Ok(devices)
}

/// Finds the first connected device made by `vendor_id`.
pub async fn find_connected_device_by_vendor(vendor_id: u16) -> Result<Option<DiscoveredDevice>> {
    Ok(connected_devices()
        .await?
        .into_iter()
        .find(|device| device.vendor_id == Some(vendor_id)))
}

/// One enumeration result: a device *interface*, not a device.
///
/// A single pair of AirPods exposes several (audio, hands-free, ...), and the vendor and product
/// ids are not necessarily present on all of them, so the ids are merged across the interfaces
/// that share an address.
///
/// The device id is kept as a `String` rather than an `HSTRING` so nothing WinRT-owned has to
/// survive into the async phase.
struct Interface {
    id: String,
    /// `None` when the address property is absent, which happens on some Windows builds.
    address: Option<u64>,
    vendor_id: Option<u16>,
    product_id: Option<u16>,
}

/// Reads one enumeration result. Synchronous by design; see [`connected_devices`].
///
/// Note what is *not* read here: `DeviceInformation::Name`. An interface is named after the
/// service it exposes, so AirPods enumerate as `"Bluetooth"` or as their bare address rather than
/// as "Anes's AirPods Pro". The name has to come from the opened device instead.
fn read_interface(info: &DeviceInformation) -> Result<Interface> {
    let properties = info.Properties()?;

    Ok(Interface {
        id: info.Id()?.to_string(),
        address: string_property(&properties, PROPERTY_ADDRESS).and_then(parse_address),
        vendor_id: u16_property(&properties, PROPERTY_VENDOR_ID),
        product_id: u16_property(&properties, PROPERTY_PRODUCT_ID),
    })
}

/// Adds an interface to the set to open, keeping one per known address.
fn collect_interface(interfaces: &mut Vec<Interface>, interface: Interface) {
    let known = interface.address.and_then(|address| {
        interfaces
            .iter_mut()
            .find(|other| other.address == Some(address))
    });

    match known {
        Some(kept) => {
            kept.vendor_id = kept.vendor_id.or(interface.vendor_id);
            kept.product_id = kept.product_id.or(interface.product_id);
        }
        None => interfaces.push(interface),
    }
}

/// Opens the device behind an interface to read its name and address.
async fn open_interface(interface: &Interface) -> Result<DiscoveredDevice> {
    let device = BluetoothDevice::FromIdAsync(&HSTRING::from(interface.id.as_str()))?
        .await
        .context("could not open the device behind the interface")?;

    let address = device
        .BluetoothAddress()
        .context("device has no bluetooth address")?;

    Ok(DiscoveredDevice {
        address,
        name: display_name(friendly_name(&device).ok(), address),
        vendor_id: interface.vendor_id,
        product_id: interface.product_id,
    })
}

/// Adds a device to the results, merging it into an entry with the same address.
///
/// The addresses of interfaces whose address property was absent are only known once they are
/// opened, so the deduplication has to happen again here.
fn collect_device(devices: &mut Vec<DiscoveredDevice>, device: DiscoveredDevice) {
    match devices
        .iter_mut()
        .find(|other| other.address == device.address)
    {
        Some(kept) => {
            kept.vendor_id = kept.vendor_id.or(device.vendor_id);
            kept.product_id = kept.product_id.or(device.product_id);
        }
        None => devices.push(device),
    }
}

/// Parses the bluetooth address property, which Windows formats as bare hex (`"a1b2c3d4e5f6"`).
fn parse_address(raw: String) -> Option<u64> {
    let cleaned: String = raw.chars().filter(|c| c.is_ascii_hexdigit()).collect();

    if cleaned.is_empty() {
        return None;
    }

    u64::from_str_radix(&cleaned, 16).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn interface(id: &str, address: Option<u64>, ids: Option<(u16, u16)>) -> Interface {
        Interface {
            id: id.to_string(),
            address,
            vendor_id: ids.map(|(vendor, _)| vendor),
            product_id: ids.map(|(_, product)| product),
        }
    }

    fn discovered(address: u64, name: &str, ids: Option<(u16, u16)>) -> DiscoveredDevice {
        DiscoveredDevice {
            address,
            name: name.to_string(),
            vendor_id: ids.map(|(vendor, _)| vendor),
            product_id: ids.map(|(_, product)| product),
        }
    }

    #[test]
    fn parses_bare_hex_addresses() {
        assert_eq!(parse_address("a1b2c3d4e5f6".into()), Some(0xa1b2c3d4e5f6));
        assert_eq!(parse_address("A1B2C3D4E5F6".into()), Some(0xa1b2c3d4e5f6));
    }

    #[test]
    fn tolerates_separators() {
        assert_eq!(
            parse_address("a1:b2:c3:d4:e5:f6".into()),
            Some(0xa1b2c3d4e5f6)
        );
        assert_eq!(
            parse_address("a1-b2-c3-d4-e5-f6".into()),
            Some(0xa1b2c3d4e5f6)
        );
    }

    #[test]
    fn rejects_unparseable_addresses() {
        assert_eq!(parse_address(String::new()), None);
        assert_eq!(parse_address("not-an-address".into()), None);
    }

    #[test]
    fn opens_one_interface_per_address() {
        let mut interfaces = Vec::new();

        collect_interface(&mut interfaces, interface("audio", Some(0xf004e1), None));
        collect_interface(&mut interfaces, interface("hfp", Some(0xf004e1), None));

        assert_eq!(interfaces.len(), 1);
        assert_eq!(interfaces[0].id, "audio");
    }

    #[test]
    fn merges_ids_across_the_interfaces_of_one_device() {
        let mut interfaces = Vec::new();

        // Windows does not report the ids on every interface of a device, so the interface that
        // gets opened is not necessarily the one that carried them.
        collect_interface(&mut interfaces, interface("audio", Some(0xf004e1), None));
        collect_interface(
            &mut interfaces,
            interface("hfp", Some(0xf004e1), Some((76, 8207))),
        );

        assert_eq!(interfaces.len(), 1);
        assert_eq!(interfaces[0].vendor_id, Some(76));
        assert_eq!(interfaces[0].product_id, Some(8207));
    }

    #[test]
    fn keeps_interfaces_without_an_address_apart() {
        let mut interfaces = Vec::new();

        collect_interface(&mut interfaces, interface("audio", None, None));
        collect_interface(&mut interfaces, interface("hfp", None, None));

        // Their addresses are unknown until they are opened, so both have to be.
        assert_eq!(interfaces.len(), 2);
    }

    #[test]
    fn deduplicates_devices_by_resolved_address() {
        let mut devices = Vec::new();

        collect_device(&mut devices, discovered(0xf004e1, "AirPods Pro", None));
        collect_device(
            &mut devices,
            discovered(0xf004e1, "AirPods Pro", Some((76, 8207))),
        );

        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].vendor_id, Some(76));
    }
}
