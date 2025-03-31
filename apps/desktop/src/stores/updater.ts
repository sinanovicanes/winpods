import { app } from "@tauri-apps/api";
import { check } from "@tauri-apps/plugin-updater";
import { acceptHMRUpdate, defineStore } from "pinia";
import { computed, ref } from "vue";

export const useUpdater = defineStore("updater", () => {
  const currentVersion = ref<string>("0.0.0");
  const latestVersion = ref<string>("0.0.0");
  const isUpdateAvailable = computed(() => currentVersion.value !== latestVersion.value);
  const isUpdating = ref(false);

  async function init() {
    const [current, update] = await Promise.all([app.getVersion(), check()]);

    currentVersion.value = current;
    latestVersion.value = (update && update.version) || current;

    setInterval(
      async () => {
        try {
          const update = await check();
          latestVersion.value = (update && update.version) || currentVersion.value;
        } catch (e) {
          console.error("[Updater] Error checking for updates:", e);
        }
      },
      5 * 60 * 1000
    ); // Check for updates every 5 minutes
  }

  async function update() {
    if (isUpdating.value) return;
    isUpdating.value = true;
    const update = await check();

    if (!update) {
      console.error("[Updater] No update information found.");
      isUpdating.value = false;
      return;
    }

    await update.downloadAndInstall();
    // TODO: Add import { relaunch } from '@tauri-apps/plugin-process';
  }

  init().catch(e => console.error("[Updater] Error during initialization:", e));

  return {
    currentVersion,
    latestVersion,
    isUpdateAvailable,
    isUpdating,
    update
  };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useUpdater, import.meta.hot));
}
