import { app } from "@tauri-apps/api";
import { relaunch } from "@tauri-apps/plugin-process";
import { check } from "@tauri-apps/plugin-updater";
import { acceptHMRUpdate, defineStore } from "pinia";
import { computed, ref } from "vue";

export const useUpdater = defineStore("updater", () => {
  const currentVersion = ref<string>("0.0.0");
  const latestVersion = ref<string>("0.0.0");
  const isUpdateAvailable = computed(() => currentVersion.value !== latestVersion.value);
  const isUpdating = ref(false);

  async function init() {
    const [current, update] = await Promise.all([
      app.getVersion(),
      check().catch(() => null)
    ]);

    currentVersion.value = current;
    latestVersion.value = (update && update.version) || current;

    createUpdateCheckerInterval();
  }

  function createUpdateCheckerInterval(ms = 60 * 60 * 1000) {
    const interval = setInterval(async () => {
      try {
        const update = await check();
        latestVersion.value = (update && update.version) || currentVersion.value;
      } catch (e) {
        console.error("[Updater] Error checking for updates:", e);
      }
    }, ms);

    // Cleanup function to clear the interval
    return () => clearInterval(interval);
  }

  async function update() {
    if (isUpdating.value) return;
    isUpdating.value = true;

    try {
      const update = await check();

      if (!update) {
        throw new Error("No update available");
      }

      await update.downloadAndInstall();
      await relaunch(); // Restart the app after update
    } catch (e) {
      console.error("[Updater] Error during update:", e);
    } finally {
      isUpdating.value = false;
    }
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
