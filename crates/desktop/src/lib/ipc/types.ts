/**
 * Types mirroring the Rust models in `crates/core`.
 *
 * These are hand-written rather than generated, so any change to a Rust model has to be
 * reflected here. `svelte-check` catches a field that stops being used, not one that changes
 * shape, so treat the Rust definitions as the source of truth.
 */

/** Mirrors `winpods_apple_cp::AppleDeviceModel`, which serialises as the bare variant name. */
export type AppleDeviceModel =
  | "AirPods1"
  | "AirPods2"
  | "AirPods3"
  | "AirPods4"
  | "AirPods4Anc"
  | "AirPodsPro"
  | "AirPodsPro2"
  | "AirPodsPro2UsbC"
  | "AirPodsPro3"
  | "AirPodsMax"
  | "AirPodsMaxUsbC"
  | "PowerbeatsPro"
  | "PowerbeatsPro2"
  | "BeatsFitPro"
  | "BeatsStudioBuds"
  | "BeatsStudioBudsPlus"
  | "BeatsSoloBuds"
  | "Unknown";

/** Mirrors `winpods_bluetooth::AdapterState`. */
export type AdapterState = "on" | "off";

/** Mirrors `winpods_core::ConnectionState`. */
export type ConnectionState = "connected" | "disconnected";

export interface Battery {
  /** Charge level as a percentage, 0-100. */
  level: number;
  charging: boolean;
}

/**
 * Bluetooth addresses are 48-bit, so they survive the trip through JSON as a `number`
 * without losing precision despite being a `u64` in Rust.
 */
export type BluetoothAddress = number;

export interface DeviceInfo {
  address: BluetoothAddress;
  name: string;
  connectionState: ConnectionState;
  model: AppleDeviceModel;
}

/**
 * Every battery is nullable: a bud in a closed case reports no level at all, which is
 * different from reporting 0%.
 */
export interface DeviceProperties {
  rssi: number;
  address: BluetoothAddress;
  model: AppleDeviceModel;
  leftBattery: Battery | null;
  rightBattery: Battery | null;
  caseBattery: Battery | null;
  leftInEar: boolean;
  rightInEar: boolean;
}

export interface DeviceSnapshot {
  device: DeviceInfo | null;
  properties: DeviceProperties | null;
}

export interface AvailableDevice {
  address: BluetoothAddress;
  name: string;
  model: AppleDeviceModel;
}

export interface Settings {
  autoStart: boolean;
  autoUpdate: boolean;
  earDetection: boolean;
  /** Notify at or below this percentage. 0 turns the notification off. */
  lowBatteryThreshold: number;
}

/** A partial settings change. Only the named fields are applied. */
export type SettingsPatch = Partial<Settings>;

/**
 * Update state, owned entirely by Rust.
 *
 * The UI never checks for or installs updates itself — two owners meant the automatic path and
 * the manual button could install the same update at once and race to restart.
 */
export interface UpdateStatus {
  /** The running version. */
  current: string;
  /** The newer version, when one is available. */
  available: string | null;
  /** Whether an install is running right now. */
  installing: boolean;
  /** Download progress as a percentage, when the total size is known. */
  progress: number | null;
  /** Why the last check or install failed. Cleared by the next successful check. */
  error: string | null;
}
