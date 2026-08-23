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

use super::{Device, PROPERTY_PRODUCT_ID, PROPERTY_VENDOR_ID};
use crate::properties::{string_property, u16_property};

/// Windows exposes the bluetooth MAC as a hex string on the device interface, which lets us read
/// the address without instantiating a `BluetoothDevice` per result.
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

/// Lists every currently connected bluetooth device.
///
/// This is a single WinRT query that asks for the vendor id, product id and address alongside the
/// device itself. The previous implementation instead ran one query for the device list and then
/// several more *per device* to read those same properties.
///
/// Reading happens in two phases on purpose. WinRT collection iterators are not `Send`, and Tauri
/// requires command futures to be `Send`, so the iterator must not be alive across an `await`.
/// Phase one drains the collection with no awaits at all; phase two does the async fallback for
/// the devices whose address Windows did not report.
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

    let mut devices = Vec::new();
    let mut pending = Vec::new();

    // Phase one: fully synchronous, so the collection is dropped before the first await.
    for info in found {
        match read_entry(&info) {
            Ok(Entry::Complete(device)) => devices.push(device),
            Ok(Entry::AddressMissing(partial)) => pending.push(partial),
            Err(e) => tracing::debug!("Skipped a connected bluetooth device: {e:#}"),
        }
    }

    // Phase two: open only the devices whose address property was absent.
    for partial in pending {
        match address_via_id(&partial.id).await {
            Ok(address) => devices.push(DiscoveredDevice {
                address,
                name: partial.name,
                vendor_id: partial.vendor_id,
                product_id: partial.product_id,
            }),
            Err(e) => tracing::debug!(
                "Skipped `{}`, its bluetooth address could not be read: {e:#}",
                partial.name
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

/// A device whose address property Windows did not provide.
///
/// The device id is kept as a `String` rather than an `HSTRING` so nothing WinRT-owned has to
/// survive into the async phase.
struct Partial {
    id: String,
    name: String,
    vendor_id: Option<u16>,
    product_id: Option<u16>,
}

enum Entry {
    Complete(DiscoveredDevice),
    AddressMissing(Partial),
}

/// Reads one enumeration result. Synchronous by design; see [`connected_devices`].
fn read_entry(info: &DeviceInformation) -> Result<Entry> {
    let properties = info.Properties()?;
    let name = info.Name()?.to_string();
    let vendor_id = u16_property(&properties, PROPERTY_VENDOR_ID);
    let product_id = u16_property(&properties, PROPERTY_PRODUCT_ID);

    match string_property(&properties, PROPERTY_ADDRESS).and_then(parse_address) {
        Some(address) => Ok(Entry::Complete(DiscoveredDevice {
            address,
            name,
            vendor_id,
            product_id,
        })),
        // The address property is not present on every Windows build, so fall back to opening
        // the device. That costs a WinRT round trip, hence only as a fallback.
        None => Ok(Entry::AddressMissing(Partial {
            id: info.Id()?.to_string(),
            name,
            vendor_id,
            product_id,
        })),
    }
}

async fn address_via_id(id: &str) -> Result<u64> {
    let device = BluetoothDevice::FromIdAsync(&HSTRING::from(id))?
        .await
        .context("could not open the device to read its address")?;

    Ok(device.BluetoothAddress()?)
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
    use super::parse_address;

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
}
