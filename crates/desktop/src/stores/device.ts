import { Events } from "@/constants";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { acceptHMRUpdate, defineStore } from "pinia";
import { computed, ref, watch } from "vue";

export const useDevice = defineStore("device-connection", () => {
  const device = ref<Device | null>(null);
  const deviceProperties = ref<DeviceProperties | null>(null);
  const availableDevices = ref<Device[]>([]);
  const batteryLevel = computed<number>(() => {
    const properties = deviceProperties.value;

    if (!properties) {
      return 0;
    }

    // If either battery is not available, return the other battery's level
    if (!properties.leftBattery.level) {
      return properties.rightBattery.level;
    } else if (!properties.rightBattery.level) {
      return properties.leftBattery.level;
    }

    return Math.min(properties.leftBattery.level, properties.rightBattery.level);
  });

  const isCharging = computed<boolean>(() => {
    const properties = deviceProperties.value;

    if (!properties) {
      return false;
    }

    // If either battery is not available, return the other battery's charging state
    if (!properties.leftBattery.level) {
      return properties.rightBattery.charging;
    } else if (!properties.rightBattery.level) {
      return properties.leftBattery.charging;
    }

    return properties.leftBattery.charging && properties.rightBattery.charging;
  });

  watch(device, async newDevice => {
    // Clear available devices when a device is connected
    if (!!newDevice) {
      availableDevices.value = [];
    } else {
      // Refresh available devices when a device is disconnected
      availableDevices.value = await getAvailableDevices();
    }
  });

  listen<Device>(Events.DeviceSelected, event => (device.value = event.payload));
  listen<Device>(Events.DeviceSelectionCleared, _ => (device.value = null));
  listen<Pick<Device, "name">>(Events.DeviceNameUpdated, event => {
    if (!device.value) return;
    device.value = { ...device.value, name: event.payload.name };
  });
  listen<DeviceConnectionState>(Events.DeviceConnectionStateUpdated, event => {
    console.log("Device connection state updated", event.payload);
    if (device.value) {
      device.value.connectionState = event.payload;
    }

    if (event.payload === "disconnected") {
      deviceProperties.value = null;
    }
  });
  listen<DeviceProperties>(
    Events.DevicePropertiesUpdated,
    event => (deviceProperties.value = event.payload)
  );

  async function getAvailableDevices(): Promise<Device[]> {
    try {
      return await invoke<Device[]>("get_bluetooth_device_list");
    } catch (e) {
      console.error(`Failed to get available devices: ${e}`);
      return [];
    }
  }

  async function refreshAvailableDevices(): Promise<void> {
    availableDevices.value = await getAvailableDevices();
  }

  async function refreshCurrentDevice(): Promise<void> {
    try {
      const response = await invoke<{ device: Device; properties: DeviceProperties }>(
        "get_current_device"
      );

      device.value = response.device || null;
      deviceProperties.value = response.properties || null;
    } catch (e) {
      console.error(`Failed to request device: ${e}`);
    }
  }

  async function selectDevice(address: number): Promise<void> {
    try {
      const device = availableDevices.value.find(d => d.address === address);
      if (!device) {
        throw new Error(`Device not found with address: ${address}`);
      }
      await invoke("select_device", device);
    } catch (e) {
      console.error(`Failed to connect to device: ${e}`);
    }
  }

  async function clearDeviceSelection(): Promise<void> {
    try {
      await invoke("clear_device_selection");
      device.value = null;
    } catch (e) {
      console.error(`Failed to disconnect: ${e}`);
    }
  }

  async function init() {
    await refreshCurrentDevice();

    if (device.value) {
      return;
    }

    const devices = await getAvailableDevices();
    availableDevices.value = devices;
  }

  init();

  return {
    device,
    deviceProperties,
    availableDevices,
    batteryLevel,
    isCharging,
    connect: selectDevice,
    disconnect: clearDeviceSelection,
    refreshAvailableDevices
  };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useDevice, import.meta.hot));
}
