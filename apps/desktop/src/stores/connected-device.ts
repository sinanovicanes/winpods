import { listen } from "@tauri-apps/api/event";
import { acceptHMRUpdate, defineStore } from "pinia";
import { ref } from "vue";

export const useConnectedDevice = defineStore("connected-device", () => {
  const device = ref<ConnectedDevice | null>(null);

  listen<ConnectedDevice>("device-updated", event => {
    device.value = event.payload;
  });

  return {
    device
  };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useConnectedDevice, import.meta.hot));
}
