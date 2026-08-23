/**
 * Backend event names.
 *
 * Kept in sync with `events::name` in `crates/desktop/src-tauri/src/events.rs`.
 */
export const Events = {
  AdapterStateChanged: "adapter-state-changed",
  DeviceSelected: "device-selected",
  DeviceSelectionCleared: "device-selection-cleared",
  DeviceConnectionChanged: "device-connection-changed",
  DeviceNameChanged: "device-name-changed",
  DevicePropertiesUpdated: "device-properties-updated",
  SettingsChanged: "settings-changed",
  UpdateStatusChanged: "update-status-changed"
} as const;
