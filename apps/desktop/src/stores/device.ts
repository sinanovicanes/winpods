import { Events } from "@/constants";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { acceptHMRUpdate, defineStore } from "pinia";
import { ref, watch } from "vue";

export const useDevice = defineStore("device-connection", () => {
  const device = ref<Device | null>(null);
  const deviceProperties = ref<DeviceProperties | null>(null);
  const availableDevices = ref<Device[]>([]);

  watch(device, async newDevice => {
    // Clear available devices when a device is connected
    if (!!newDevice) {
      availableDevices.value = [];
    } else {
      // Refresh available devices when a device is disconnected
      availableDevices.value = await getAvailableDevices();
    }
  });

  listen<Device>(Events.DeviceConnected, event => (device.value = event.payload));
  listen<Device>(Events.DeviceDisconnected, _ => (device.value = null));
  listen<Pick<Device, "name">>(Events.DeviceNameUpdated, event => {
    if (!device.value) return;
    device.value = { ...device.value, name: event.payload.name };
  });
  listen<Pick<Device, "connectionState">>(Events.DeviceConnectionUpdated, event => {
    if (!device.value) return;
    device.value = { ...device.value, connectionState: event.payload.connectionState };
  });
  listen<DeviceProperties>(
    Events.DevicePropertiesUpdated,
    event => (deviceProperties.value = event.payload)
  );

  async function disconnect(): Promise<void> {
    try {
      await invoke("disconnect");
      device.value = null;
    } catch (e) {
      console.error(`Failed to disconnect: ${e}`);
    }
  }

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

  async function connect(address: number): Promise<void> {
    try {
      const device = availableDevices.value.find(d => d.address === address);
      if (!device) {
        throw new Error(`Device not found with address: ${address}`);
      }
      await invoke("connect", device);
    } catch (e) {
      console.error(`Failed to connect to device: ${e}`);
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
    connect,
    disconnect,
    refreshAvailableDevices
  };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useDevice, import.meta.hot));
}
