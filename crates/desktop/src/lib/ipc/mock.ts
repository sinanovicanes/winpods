import { Events } from "./events";
import type { Backend, Unlisten } from "./backend";
import type {
  AdapterState,
  AvailableDevice,
  BluetoothAddress,
  Battery,
  DeviceInfo,
  DeviceProperties,
  DeviceSnapshot,
  Settings,
  SettingsPatch,
  UpdateStatus
} from "./types";

/**
 * A backend that fakes the Rust app.
 *
 * This is what makes `bun run dev` usable on a machine that cannot run winpods at all: the app
 * is Windows only, but the UI is not, and designing it should not require a Windows box.
 *
 * It keeps real state, emits the same events as the backend, and slowly drains the batteries so
 * live updates are visible. `devControls` below drives the dev-only panel that exercises the
 * states which are otherwise hard to reach -- bluetooth off, disconnected, critical battery.
 */

const DEVICES: AvailableDevice[] = [
  { address: 0xa1b2c3d4e5f6, name: "Anes's AirPods Pro", model: "AirPodsPro2" },
  { address: 0xb2c3d4e5f6a1, name: "Anes's AirPods Max", model: "AirPodsMaxUsbC" },
  { address: 0xc3d4e5f6a1b2, name: "Beats Fit Pro", model: "BeatsFitPro" }
];

const DRAIN_INTERVAL = 4000;

type Listener = (payload: unknown) => void;

class MockBackend implements Backend {
  readonly isLive = false;

  #listeners = new Map<string, Set<Listener>>();
  #adapter: AdapterState = "on";
  #device: DeviceInfo | null = null;
  #properties: DeviceProperties | null = null;
  #settings: Settings = {
    autoStart: true,
    autoUpdate: true,
    earDetection: true,
    lowBatteryThreshold: 20
  };
  #alwaysOnTop = false;
  #update: UpdateStatus = {
    current: "0.2.0",
    available: "0.2.1",
    installing: false,
    progress: null,
    error: null
  };

  constructor() {
    // Start with a device already selected, which is the interesting state to design against.
    this.selectDevice(DEVICES[0]!.address);
    setInterval(() => this.#drain(), DRAIN_INTERVAL);
  }

  // ---------------------------------------------------------------- events

  #emit(event: string, payload: unknown) {
    for (const handler of this.#listeners.get(event) ?? []) {
      handler(payload);
    }
  }

  async listen<T>(event: string, handler: (payload: T) => void): Promise<Unlisten> {
    const wrapped = handler as Listener;
    const handlers = this.#listeners.get(event) ?? new Set<Listener>();
    handlers.add(wrapped);
    this.#listeners.set(event, handlers);

    return () => handlers.delete(wrapped);
  }

  async onWindowFocus(): Promise<Unlisten> {
    // There is no window to focus in a browser tab.
    return () => {};
  }

  // ---------------------------------------------------------------- commands

  async getAdapterState() {
    return this.#adapter;
  }

  async listDevices() {
    return this.#adapter === "on" ? DEVICES : [];
  }

  async getCurrentDevice(): Promise<DeviceSnapshot> {
    return { device: this.#device, properties: this.#properties };
  }

  async selectDevice(address: BluetoothAddress): Promise<DeviceInfo> {
    const found = DEVICES.find(device => device.address === address);

    if (!found) {
      throw new Error(`No device at address ${address}`);
    }

    this.#device = { ...found, connectionState: "connected" };
    this.#properties = freshProperties(found);

    this.#emit(Events.DeviceSelected, this.#device);
    this.#emit(Events.DevicePropertiesUpdated, this.#properties);

    return this.#device;
  }

  async clearDeviceSelection() {
    this.#device = null;
    this.#properties = null;
    this.#emit(Events.DeviceSelectionCleared, null);
  }

  async getSettings() {
    return this.#settings;
  }

  async updateSettings(patch: SettingsPatch): Promise<Settings> {
    const next = { ...this.#settings, ...stripUndefined(patch) };
    // Mirrors the backend's clamp so the UI's reconciliation path is exercised too.
    next.lowBatteryThreshold = Math.min(next.lowBatteryThreshold, 90);

    this.#settings = next;
    this.#emit(Events.SettingsChanged, next);

    return next;
  }

  // ---------------------------------------------------------------- window

  async hideWindow() {
    console.info("[mock] hideWindow()");
  }

  async isAlwaysOnTop() {
    return this.#alwaysOnTop;
  }

  async setAlwaysOnTop(value: boolean) {
    this.#alwaysOnTop = value;
  }

  // ---------------------------------------------------------------- updater

  async getUpdateStatus(): Promise<UpdateStatus> {
    return this.#update;
  }

  async installUpdate() {
    if (this.#update.installing) return;

    // Mirrors the backend: progress ticks up, then the app would restart.
    this.#patchUpdate({ installing: true, progress: 0 });

    for (let percent = 20; percent <= 100; percent += 20) {
      await new Promise(resolve => setTimeout(resolve, 400));
      this.#patchUpdate({ progress: percent });
    }

    this.#patchUpdate({ installing: false, progress: null, available: null });
    console.info(
      "[mock] installUpdate() finished (a real install would restart the app)"
    );
  }

  #patchUpdate(patch: Partial<UpdateStatus>) {
    this.#update = { ...this.#update, ...patch };
    this.#emit(Events.UpdateStatusChanged, this.#update);
  }

  // ---------------------------------------------------------------- simulation

  /** Drains the batteries a little so the UI visibly updates over time. */
  #drain() {
    if (!this.#properties || this.#device?.connectionState !== "connected") {
      return;
    }

    const step = (battery: Battery | null): Battery | null => {
      if (!battery) return null;
      if (battery.charging) {
        return { ...battery, level: Math.min(100, battery.level + 10) };
      }
      return { ...battery, level: Math.max(0, battery.level - 10) };
    };

    this.#patchProperties({
      leftBattery: step(this.#properties.leftBattery),
      rightBattery: step(this.#properties.rightBattery)
    });
  }

  #patchProperties(patch: Partial<DeviceProperties>) {
    if (!this.#properties) return;

    this.#properties = { ...this.#properties, ...patch };
    this.#emit(Events.DevicePropertiesUpdated, this.#properties);
  }

  // ---------------------------------------------------------------- dev controls

  /** Drives the dev-only panel. Never reachable in a Tauri build. */
  readonly dev = {
    setAdapter: (state: AdapterState) => {
      this.#adapter = state;
      this.#emit(Events.AdapterStateChanged, state);
    },

    setConnected: (connected: boolean) => {
      if (!this.#device) return;

      this.#device = {
        ...this.#device,
        connectionState: connected ? "connected" : "disconnected"
      };

      if (!connected) {
        this.#properties = null;
      } else {
        this.#properties = freshProperties(this.#device);
        this.#emit(Events.DevicePropertiesUpdated, this.#properties);
      }

      this.#emit(Events.DeviceConnectionChanged, this.#device.connectionState);
    },

    setBattery: (level: number) => {
      const props = this.#properties;
      if (!props) return;

      this.#patchProperties({
        leftBattery: props.leftBattery && { level, charging: props.leftBattery.charging },
        rightBattery: props.rightBattery && {
          level,
          charging: props.rightBattery.charging
        }
      });
    },

    setCharging: (charging: boolean) => {
      const props = this.#properties;
      if (!props) return;

      this.#patchProperties({
        leftBattery: props.leftBattery && { ...props.leftBattery, charging },
        rightBattery: props.rightBattery && { ...props.rightBattery, charging },
        caseBattery: props.caseBattery && { ...props.caseBattery, charging }
      });
    },

    setInEar: (inEar: boolean) => {
      this.#patchProperties({ leftInEar: inEar, rightInEar: inEar });
    },

    setCaseReported: (reported: boolean) => {
      this.#patchProperties({
        caseBattery: reported ? { level: 50, charging: false } : null
      });
    },

    setBudReported: (side: "left" | "right", reported: boolean) => {
      const key = side === "left" ? "leftBattery" : "rightBattery";
      this.#patchProperties({ [key]: reported ? { level: 70, charging: false } : null });
    },

    setDeviceName: (name: string) => {
      if (!this.#device) return;
      this.#device = { ...this.#device, name };
      this.#emit(Events.DeviceNameChanged, name);
    },

    devices: DEVICES
  };
}

function freshProperties(device: AvailableDevice | DeviceInfo): DeviceProperties {
  const overEar = device.model === "AirPodsMax" || device.model === "AirPodsMaxUsbC";

  return {
    rssi: -52,
    address: device.address,
    model: device.model,
    leftBattery: { level: 90, charging: false },
    // Over-ear models are one unit: they report a single battery and no case, so the second
    // reading is absent rather than duplicated.
    rightBattery: overEar ? null : { level: 80, charging: false },
    caseBattery: overEar ? null : { level: 50, charging: false },
    leftInEar: true,
    rightInEar: !overEar
  };
}

/** Drops `undefined` values so they do not overwrite real settings when spread. */
function stripUndefined(patch: SettingsPatch): SettingsPatch {
  return Object.fromEntries(
    Object.entries(patch).filter(([, value]) => value !== undefined)
  ) as SettingsPatch;
}

export const mockBackend = new MockBackend();
