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

export enum Events {
  DeviceUpdated = "device-updated",
  SettingsUpdateAutoUpdate = "settings:update:auto_update",
  SettingsUpdateNotifications = "settings:update:notifications",
  SettingsUpdateLowBatteryNotification = "settings:update:low_battery_notification",
  SettingsUpdateEarDetection = "settings:update:ear_detection",
  SettingsSetAutoUpdate = "settings:set:auto_update",
  SettingsSetNotifications = "settings:set:notifications",
  SettingsSetLowBatteryNotification = "settings:set:low_battery_notification",
  SettingsSetEarDetection = "settings:set:ear_detection"
}
