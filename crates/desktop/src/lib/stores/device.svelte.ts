import { backend, Events } from "$lib/ipc";
import type {
  AvailableDevice,
  BluetoothAddress,
  ConnectionState,
  DeviceInfo,
  DeviceProperties
} from "$lib/ipc";

/**
 * The selected device and its live readings.
 *
 * Mirrors the backend rather than deriving anything of its own: the backend already decides which
 * advertisements are plausible, so the UI's job is only to render what it is told.
 */
class DeviceStore {
  device = $state<DeviceInfo | null>(null);
  properties = $state<DeviceProperties | null>(null);
  available = $state<AvailableDevice[]>([]);

  /** True until the first snapshot arrives, so the UI can avoid flashing "no device". */
  loading = $state(true);
  refreshing = $state(false);

  readonly isConnected = $derived(this.device?.connectionState === "connected");

  /** The lower of the two buds, falling back to whichever one reported. */
  readonly overallLevel = $derived.by(() => {
    const left = this.properties?.leftBattery ?? null;
    const right = this.properties?.rightBattery ?? null;

    if (left && right) return Math.min(left.level, right.level);
    return (left ?? right)?.level ?? null;
  });

  /** Both buds have to charge for the device to read as charging. */
  readonly isCharging = $derived.by(() => {
    const left = this.properties?.leftBattery ?? null;
    const right = this.properties?.rightBattery ?? null;

    if (left && right) return left.charging && right.charging;
    return (left ?? right)?.charging ?? false;
  });

  #started = false;

  async start() {
    if (this.#started) return;
    this.#started = true;

    await Promise.all([
      backend.listen<DeviceInfo>(Events.DeviceSelected, device => {
        this.device = device;
        this.available = [];
      }),

      backend.listen(Events.DeviceSelectionCleared, () => {
        this.device = null;
        this.properties = null;
        void this.refreshAvailable();
      }),

      backend.listen<string>(Events.DeviceNameChanged, name => {
        if (this.device) {
          this.device = { ...this.device, name };
        }
      }),

      backend.listen<ConnectionState>(Events.DeviceConnectionChanged, connectionState => {
        if (this.device) {
          this.device = { ...this.device, connectionState };
        }

        // A disconnected device reports nothing, so its last readings are already stale.
        if (connectionState === "disconnected") {
          this.properties = null;
        }
      }),

      backend.listen<DeviceProperties>(Events.DevicePropertiesUpdated, properties => {
        this.properties = properties;
      })
    ]);

    // A device may have been selected before this window finished loading, in which case the
    // event was missed. Asking again on focus keeps it from being stuck on "No device selected".
    await backend.onWindowFocus(() => void this.refresh());

    await this.refresh();
  }

  async refresh() {
    try {
      const snapshot = await backend.getCurrentDevice();
      this.device = snapshot.device;
      this.properties = snapshot.properties;

      if (!snapshot.device) {
        await this.refreshAvailable();
      }
    } catch (error) {
      console.error("Failed to read the current device:", error);
    } finally {
      this.loading = false;
    }
  }

  async refreshAvailable() {
    this.refreshing = true;

    try {
      this.available = await backend.listDevices();
    } catch (error) {
      console.error("Failed to list the available devices:", error);
      this.available = [];
    } finally {
      this.refreshing = false;
    }
  }

  async select(address: BluetoothAddress) {
    try {
      this.device = await backend.selectDevice(address);
      this.available = [];
    } catch (error) {
      console.error("Failed to select the device:", error);
    }
  }

  async disconnect() {
    try {
      await backend.clearDeviceSelection();
      this.device = null;
      this.properties = null;
      await this.refreshAvailable();
    } catch (error) {
      console.error("Failed to clear the device selection:", error);
    }
  }
}

export const devices = new DeviceStore();
