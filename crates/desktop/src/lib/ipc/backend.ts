import type {
  AdapterState,
  AvailableDevice,
  BluetoothAddress,
  DeviceInfo,
  DeviceSnapshot,
  Settings,
  SettingsPatch,
  UpdateStatus
} from "./types";

export type Unlisten = () => void;

/**
 * Everything the UI needs from the outside world.
 *
 * Having this as an interface is what lets the UI run without the Rust backend: the real
 * implementation talks to Tauri, and a mock one serves fixtures so the app can be developed
 * and designed on any OS.
 */
export interface Backend {
  /** Whether this backend talks to the real Rust app. Drives the dev-only controls. */
  readonly isLive: boolean;

  getAdapterState(): Promise<AdapterState>;
  listDevices(): Promise<AvailableDevice[]>;
  getCurrentDevice(): Promise<DeviceSnapshot>;
  selectDevice(address: BluetoothAddress): Promise<DeviceInfo>;
  clearDeviceSelection(): Promise<void>;
  getSettings(): Promise<Settings>;
  updateSettings(patch: SettingsPatch): Promise<Settings>;

  /** Subscribes to a backend event. */
  listen<T>(event: string, handler: (payload: T) => void): Promise<Unlisten>;

  /** Runs `callback` whenever this window is brought back to the front. */
  onWindowFocus(callback: () => void): Promise<Unlisten>;

  /** Hides the current window. Used by the widget's close button. */
  hideWindow(): Promise<void>;

  /** Reads and sets the widget's always-on-top pin. */
  isAlwaysOnTop(): Promise<boolean>;
  setAlwaysOnTop(value: boolean): Promise<void>;

  /**
   * Update state. Rust owns checking and installing; the UI only reads and requests.
   */
  getUpdateStatus(): Promise<UpdateStatus>;
  installUpdate(): Promise<void>;
}
