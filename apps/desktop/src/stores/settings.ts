import { Events } from "@/constants";
import { invoke } from "@tauri-apps/api/core";
import { emit, listen } from "@tauri-apps/api/event";
import { disable, enable, isEnabled } from "@tauri-apps/plugin-autostart";
import { acceptHMRUpdate, defineStore } from "pinia";
import { ref, watch } from "vue";

interface SettingsState {
  autoUpdate: boolean;
  notification: boolean;
  lowBatteryNotification: boolean;
  earDetection: boolean;
}

export const useSettings = defineStore("settings", () => {
  const autoStart = ref(true);
  const autoUpdate = ref(true);
  const notifications = ref(true);
  const lowBatteryNotification = ref(true);
  const earDetection = ref(true);

  async function init() {
    const settingsState = await invoke<SettingsState>("get_settings_state");

    autoUpdate.value = settingsState.autoUpdate;
    notifications.value = settingsState.notification;
    lowBatteryNotification.value = settingsState.lowBatteryNotification;
    earDetection.value = settingsState.earDetection;

    // Initialize autoStart value
    isEnabled().then(value => {
      autoStart.value = value;

      watch(autoStart, async value => {
        if (value) {
          await enable();
          console.log("AutoStart enabled");
        } else {
          await disable();
          console.log("AutoStart disabled");
        }
      });

      console.log("Auto start initialized:", value);
    });

    // Initialize listeners after the initial values are set
    listen<boolean>(Events.SettingsUpdateAutoUpdate, event => {
      autoUpdate.value = event.payload;
    });

    listen<boolean>(Events.SettingsUpdateNotifications, event => {
      notifications.value = event.payload;
    });

    listen<boolean>(Events.SettingsUpdateLowBatteryNotification, event => {
      lowBatteryNotification.value = event.payload;
    });

    listen<boolean>(Events.SettingsUpdateEarDetection, event => {
      earDetection.value = event.payload;
    });

    const createSynchronizer = (event: string) => (newValue: any) =>
      emit(event, newValue);
    watch(autoUpdate, createSynchronizer(Events.SettingsSetAutoUpdate));
    watch(notifications, createSynchronizer(Events.SettingsSetNotifications));
    watch(
      lowBatteryNotification,
      createSynchronizer(Events.SettingsSetLowBatteryNotification)
    );
    watch(earDetection, createSynchronizer(Events.SettingsSetEarDetection));
  }

  init();

  return {
    autoStart,
    autoUpdate,
    notifications,
    lowBatteryNotification,
    earDetection
  };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useSettings, import.meta.hot));
}
