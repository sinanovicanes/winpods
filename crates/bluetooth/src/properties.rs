//! Helpers for reading `DeviceInformation` property bags.
//!
//! WinRT hands these out as `IMap<HSTRING, IInspectable>` where every value has to be cast to an
//! `IPropertyValue` and then read with a type specific getter. A missing property and a property
//! of an unexpected type are both "we could not read it", so these helpers collapse both into
//! `None` and log the difference.

use windows::{
    Foundation::IPropertyValue,
    core::{HSTRING, IInspectable, Interface},
};
use windows_collections::IMapView;

type Properties = IMapView<HSTRING, IInspectable>;

/// Reads a `u16` device property, returning `None` when it is absent or not a `u16`.
pub(crate) fn u16_property(properties: &Properties, name: &str) -> Option<u16> {
    property_value(properties, name)?
        .GetUInt16()
        .inspect_err(|e| tracing::debug!("Device property `{name}` is not a u16: {e}"))
        .ok()
}

/// Reads a string device property, returning `None` when it is absent or not a string.
pub(crate) fn string_property(properties: &Properties, name: &str) -> Option<String> {
    property_value(properties, name)?
        .GetString()
        .inspect_err(|e| tracing::debug!("Device property `{name}` is not a string: {e}"))
        .ok()
        .map(|value| value.to_string())
}

fn property_value(properties: &Properties, name: &str) -> Option<IPropertyValue> {
    properties
        .Lookup(&HSTRING::from(name))
        .inspect_err(|_| tracing::debug!("Device property `{name}` is not available"))
        .ok()?
        .cast::<IPropertyValue>()
        .inspect_err(|e| tracing::debug!("Device property `{name}` is not a property value: {e}"))
        .ok()
}
