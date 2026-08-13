import { BluetoothAdapterState, Events } from "@/constants";
import { invokeWithRetry, onWindowShown } from "@/utils";
import { listen } from "@tauri-apps/api/event";
import { acceptHMRUpdate, defineStore } from "pinia";
import { ref } from "vue";

export const useBluetooth = defineStore("bluetooth", () => {
  const isActive = ref(false);

  async function getBluetoothAdapterState(): Promise<boolean> {
    try {
      return await invokeWithRetry<boolean>("is_bluetooth_adapter_active");
    } catch (e) {
      console.error(`Failed to get the bluetooth adapter state: ${e}`);
      return false;
    }
  }

  listen<BluetoothAdapterState>(Events.BluetoothAdapterStateUpdated, event => {
    isActive.value = event.payload === BluetoothAdapterState.On;
  });

  async function refresh(): Promise<void> {
    isActive.value = await getBluetoothAdapterState();
  }

  // The adapter may have been unreachable while this window was loading, in which case its state
  // update was missed. Refreshing on show keeps the window from being stuck on the warning.
  onWindowShown(refresh);

  refresh();

  return {
    isActive,
    refresh
  };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useBluetooth, import.meta.hot));
}
