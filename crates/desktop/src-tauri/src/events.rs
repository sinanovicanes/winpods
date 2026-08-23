//! The app's internal event bus.
//!
//! Everything interesting that happens in the backend becomes an [`AppEvent`] on one broadcast
//! channel. Features subscribe to that channel, and a single bridge task forwards each event to
//! the webviews.
//!
//! This replaces the previous arrangement, where the backend talked to *itself* through Tauri's
//! global event system: features re-parsed their own payloads with
//! `serde_json::from_str(event.payload())` and silently returned when that failed. Here the
//! payload is the typed value, so there is nothing to reparse and nothing to fail.

use tauri::{AppHandle, Emitter};
use winpods_bluetooth::{AdapterState, ConnectionState};
use winpods_core::{DeviceInfo, DeviceProperties, Settings};

use crate::updater::UpdateStatus;

/// Event names the UI subscribes to. Kept in sync with `src/lib/ipc/events.ts`.
pub mod name {
    pub const ADAPTER_STATE_CHANGED: &str = "adapter-state-changed";
    pub const DEVICE_SELECTED: &str = "device-selected";
    pub const DEVICE_SELECTION_CLEARED: &str = "device-selection-cleared";
    pub const DEVICE_CONNECTION_CHANGED: &str = "device-connection-changed";
    pub const DEVICE_NAME_CHANGED: &str = "device-name-changed";
    pub const DEVICE_PROPERTIES_UPDATED: &str = "device-properties-updated";
    pub const SETTINGS_CHANGED: &str = "settings-changed";
    pub const UPDATE_STATUS_CHANGED: &str = "update-status-changed";
}

/// Something that changed in the backend.
#[derive(Debug, Clone)]
pub enum AppEvent {
    AdapterStateChanged(AdapterState),
    DeviceSelected(DeviceInfo),
    DeviceSelectionCleared,
    DeviceConnectionChanged(ConnectionState),
    DeviceNameChanged(String),
    DevicePropertiesUpdated(DeviceProperties),
    SettingsChanged(Settings),
    UpdateStatusChanged(UpdateStatus),
}

impl AppEvent {
    /// Forwards this event to every webview.
    ///
    /// Both windows listen to all of them; each decides what it cares about.
    pub fn emit(&self, app: &AppHandle) {
        let result = match self {
            Self::AdapterStateChanged(state) => app.emit(name::ADAPTER_STATE_CHANGED, state),
            Self::DeviceSelected(info) => app.emit(name::DEVICE_SELECTED, info),
            Self::DeviceSelectionCleared => app.emit(name::DEVICE_SELECTION_CLEARED, ()),
            Self::DeviceConnectionChanged(state) => {
                app.emit(name::DEVICE_CONNECTION_CHANGED, state)
            }
            Self::DeviceNameChanged(nm) => app.emit(name::DEVICE_NAME_CHANGED, nm),
            Self::DevicePropertiesUpdated(props) => {
                app.emit(name::DEVICE_PROPERTIES_UPDATED, props)
            }
            Self::SettingsChanged(settings) => app.emit(name::SETTINGS_CHANGED, settings),
            Self::UpdateStatusChanged(status) => app.emit(name::UPDATE_STATUS_CHANGED, status),
        };

        if let Err(e) = result {
            tracing::error!("Failed to emit {self:?} to the webviews: {e}");
        }
    }
}
