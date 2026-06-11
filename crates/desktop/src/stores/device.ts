import { Events } from "@/constants";
import { isTauriRuntime } from "@/tauri";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { acceptHMRUpdate, defineStore } from "pinia";
import { computed, ref, watch } from "vue";

export const useDevice = defineStore("device-connection", () => {
  const device = ref<Device | null>(null);
  const deviceProperties = ref<DeviceProperties | null>(null);
  const availableDevices = ref<Device[]>([]);
  const audioEndpoints = ref<AudioEndpoint[]>([]);
  const serviceActionStatus = ref("");
  const isServiceActionPending = ref(false);
  const serviceActionName = ref<"connect" | "disconnect" | null>(null);
  const audioRouteStatus = ref("");
  const isAudioRoutePending = ref(false);
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

  if (isTauriRuntime()) {
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
  }

  async function getAvailableDevices(): Promise<Device[]> {
    if (!isTauriRuntime()) {
      return [];
    }

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
    if (!isTauriRuntime()) {
      return;
    }

    try {
      const response = await invoke<{ device: Device; properties: DeviceProperties }>(
        "get_current_device"
      );

      device.value = response.device || null;
      deviceProperties.value = response.properties || null;
      await refreshAudioEndpoints();
    } catch (e) {
      console.error(`Failed to request device: ${e}`);
    }
  }

  async function refreshAudioEndpoints(): Promise<void> {
    if (!isTauriRuntime()) {
      audioEndpoints.value = [];
      return;
    }

    try {
      audioEndpoints.value = await invoke<AudioEndpoint[]>("get_audio_output_endpoints");
    } catch (e) {
      audioEndpoints.value = [];
      console.error(`Failed to get audio outputs: ${e}`);
    }
  }

  async function selectDevice(address: number): Promise<void> {
    if (!isTauriRuntime()) {
      return;
    }

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
    if (!isTauriRuntime()) {
      return;
    }

    try {
      await invoke("clear_device_selection");
      device.value = null;
    } catch (e) {
      console.error(`Failed to disconnect: ${e}`);
    }
  }

  async function openBluetoothSettings(): Promise<void> {
    if (!isTauriRuntime()) {
      return;
    }

    try {
      await invoke("open_bluetooth_settings");
    } catch (e) {
      console.error(`Failed to open Bluetooth settings: ${e}`);
    }
  }

  async function openSoundSettings(): Promise<void> {
    if (!isTauriRuntime()) {
      return;
    }

    try {
      await invoke("open_sound_settings");
    } catch (e) {
      console.error(`Failed to open Sound settings: ${e}`);
    }
  }

  async function routeAudioToSelectedDevice(): Promise<void> {
    if (!isTauriRuntime()) {
      return;
    }

    isAudioRoutePending.value = true;
    audioRouteStatus.value = "Looking for an active AirPods output in Windows...";

    try {
      const summary = await invoke<AudioRouteSummary>("route_audio_to_selected_device");
      audioEndpoints.value = summary.endpoints;
      audioRouteStatus.value = audioRouteSummaryText(summary);
      await refreshAudioEndpoints();
    } catch (e) {
      audioRouteStatus.value = `Audio route failed: ${e}`;
      await refreshAudioEndpoints();
      console.error(`Failed to route audio to selected device: ${e}`);
    } finally {
      isAudioRoutePending.value = false;
    }
  }

  async function connectSelectedDeviceServices(): Promise<void> {
    if (!isTauriRuntime()) {
      return;
    }

    isServiceActionPending.value = true;
    serviceActionName.value = "connect";
    serviceActionStatus.value = "Requesting Windows to connect AirPods services...";

    try {
      const summary = await invoke<ServiceToggleSummary>(
        "connect_selected_device_services"
      );
      serviceActionStatus.value = serviceSummaryText("Connect", summary);
      await refreshCurrentDeviceAfterServiceChange();
    } catch (e) {
      serviceActionStatus.value = `Connect failed: ${e}`;
      console.error(`Failed to connect selected device services: ${e}`);
    } finally {
      isServiceActionPending.value = false;
      serviceActionName.value = null;
    }
  }

  async function disconnectSelectedDeviceServices(): Promise<void> {
    if (!isTauriRuntime()) {
      return;
    }

    isServiceActionPending.value = true;
    serviceActionName.value = "disconnect";
    serviceActionStatus.value = "Requesting Windows to disconnect AirPods services...";

    try {
      const summary = await invoke<ServiceToggleSummary>(
        "disconnect_selected_device_services"
      );
      serviceActionStatus.value = serviceSummaryText("Disconnect", summary);
      await refreshCurrentDeviceAfterServiceChange();
    } catch (e) {
      serviceActionStatus.value = `Disconnect failed: ${e}`;
      console.error(`Failed to disconnect selected device services: ${e}`);
    } finally {
      isServiceActionPending.value = false;
      serviceActionName.value = null;
    }
  }

  function serviceSummaryText(action: string, summary: ServiceToggleSummary): string {
    if (summary.attempted === 0) {
      return `${action} could not run: Windows reported no installed Bluetooth services for this device.`;
    }

    if (summary.succeeded === 0) {
      return `${action} requested, but Windows did not accept any of the ${summary.attempted} Bluetooth service changes. Use Bluetooth settings for this device.`;
    }

    const failedText = summary.failed > 0 ? ` ${summary.failed} failed.` : "";
    return `${action} requested: Windows accepted ${summary.succeeded}/${summary.attempted} Bluetooth service changes.${failedText} Watching for the AirPods state to update...`;
  }

  function audioRouteSummaryText(summary: AudioRouteSummary): string {
    if (!summary.matchedEndpoint) {
      return "Windows did not expose an active AirPods audio output endpoint.";
    }

    if (summary.rolesSucceeded === 0) {
      return `Found ${summary.matchedEndpoint.name}, but Windows refused the default-output change.`;
    }

    const failedText = summary.rolesFailed > 0 ? ` ${summary.rolesFailed} roles failed.` : "";
    return `Audio routed to ${summary.matchedEndpoint.name}: ${summary.rolesSucceeded}/${summary.rolesAttempted} default roles updated.${failedText}`;
  }

  async function refreshCurrentDeviceAfterServiceChange(): Promise<void> {
    const refreshDelays = [0, 1500, 2000, 3000, 3500];

    for (const delay of refreshDelays) {
      if (delay > 0) {
        await new Promise(resolve => setTimeout(resolve, delay));
      }

      await refreshCurrentDevice();
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
    audioEndpoints,
    serviceActionStatus,
    isServiceActionPending,
    serviceActionName,
    audioRouteStatus,
    isAudioRoutePending,
    batteryLevel,
    isCharging,
    connect: selectDevice,
    disconnect: clearDeviceSelection,
    connectServices: connectSelectedDeviceServices,
    disconnectServices: disconnectSelectedDeviceServices,
    routeAudio: routeAudioToSelectedDevice,
    openBluetoothSettings,
    openSoundSettings,
    refreshAvailableDevices,
    refreshAudioEndpoints
  };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useDevice, import.meta.hot));
}
