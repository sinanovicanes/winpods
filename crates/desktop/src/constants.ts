export enum AirPodsModel {
  AirPods1 = "AirPods 1",
  AirPods2 = "AirPods 2",
  AirPods3 = "AirPods 3",
  AirPodsPro = "AirPods Pro",
  AirPodsPro2 = "AirPods Pro 2",
  AirPodsPro2UsbC = "AirPods Pro 2 (USB-C)",
  AirPodsMax = "AirPods Max",
  BeatsFitPro = "Beats Fit Pro",
  Unknown = "Unknown"
}

export enum BluetoothAdapterState {
  On = "on",
  Off = "off"
}

export enum Events {
  DeviceConnected = "device-connected",
  DeviceDisconnected = "device-disconnected",
  DevicePropertiesUpdated = "device-properties-updated",
  DeviceNameUpdated = "device-name-updated",
  DeviceConnectionUpdated = "device-connection-updated",
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
