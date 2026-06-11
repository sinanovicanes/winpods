import { BluetoothAdapterState, Events } from "@/constants";
import { isTauriRuntime } from "@/tauri";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { acceptHMRUpdate, defineStore } from "pinia";
import { ref } from "vue";

export const useBluetooth = defineStore("bluetooth", () => {
  const isActive = ref(!isTauriRuntime());

  async function getBluetoothAdapterState(): Promise<boolean> {
    if (!isTauriRuntime()) {
      return true;
    }

    try {
      return await invoke<boolean>("is_bluetooth_adapter_active");
    } catch {
      return false;
    }
  }

  if (isTauriRuntime()) {
    listen<BluetoothAdapterState>(Events.BluetoothAdapterStateUpdated, event => {
      isActive.value = event.payload === BluetoothAdapterState.On;
    });
  }

  async function init() {
    isActive.value = await getBluetoothAdapterState();
  }

  init();

  return {
    isActive
  };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useBluetooth, import.meta.hot));
}
