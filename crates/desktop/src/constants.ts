export enum BluetoothAdapterState {
  On = "on",
  Off = "off"
}

export enum Events {
  DeviceConnected = "device-connected",
  DeviceDisconnected = "device-disconnected",
  DevicePropertiesUpdated = "device-properties-updated",
  DeviceNameUpdated = "device-name-updated",
  DeviceConnectionStateUpdated = "device-connection-state-updated",
  BluetoothAdapterStateUpdated = "bluetooth-adapter-state-updated",
  SettingsUpdateAutoStart = "settings:update:auto_start",
  SettingsUpdateAutoUpdate = "settings:update:auto_update",
  SettingsUpdateLowBatteryThreshold = "settings:update:low_battery_threshold",
  SettingsUpdateEarDetection = "settings:update:ear_detection",
  SettingsSetAutoStart = "settings:set:auto_start",
  SettingsSetAutoUpdate = "settings:set:auto_update",
  SettingsSetLowBatteryThreshold = "settings:set:low_battery_threshold",
  SettingsSetEarDetection = "settings:set:ear_detection"
}
