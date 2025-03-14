import { emit, listen } from "@tauri-apps/api/event";
import { acceptHMRUpdate, defineStore } from "pinia";
import { ref, watch } from "vue";

export const useSettings = defineStore("settings", () => {
  const autoUpdate = ref(true);
  const notifications = ref(true);
  const lowBatteryNotification = ref(true);
  const earDetection = ref(true);

  listen<boolean>("settings:update:auto_update", event => {
    autoUpdate.value = event.payload;
  });

  listen<boolean>("settings:update:notifications", event => {
    notifications.value = event.payload;
  });

  listen<boolean>("settings:update:low_battery_notification", event => {
    lowBatteryNotification.value = event.payload;
  });

  listen<boolean>("settings:update:ear_detection", event => {
    earDetection.value = event.payload;
  });

  const createSynchronizer = (event: string) => (newValue: any) => emit(event, newValue);
  watch(autoUpdate, createSynchronizer("settings:set:auto_update"));
  watch(notifications, createSynchronizer("settings:set:notifications"));
  watch(
    lowBatteryNotification,
    createSynchronizer("settings:set:low_battery_notification")
  );
  watch(earDetection, createSynchronizer("settings:set:ear_detection"));

  return {
    autoUpdate,
    notifications,
    lowBatteryNotification,
    earDetection
  };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useSettings, import.meta.hot));
}
