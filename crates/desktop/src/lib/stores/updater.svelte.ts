import { backend } from "$lib/ipc";

const CHECK_INTERVAL = 60 * 60 * 1000;

/** Tracks the installed version and whether a newer one is available. */
class UpdaterStore {
  currentVersion = $state("0.0.0");
  latestVersion = $state<string | null>(null);
  installing = $state(false);

  readonly updateAvailable = $derived(
    this.latestVersion !== null && this.latestVersion !== this.currentVersion
  );

  #started = false;

  async start() {
    if (this.#started) return;
    this.#started = true;

    try {
      this.currentVersion = await backend.getVersion();
    } catch (error) {
      console.error("Failed to read the app version:", error);
    }

    await this.check();
    setInterval(() => void this.check(), CHECK_INTERVAL);
  }

  async check() {
    try {
      const update = await backend.checkForUpdate();
      this.latestVersion = update?.version ?? null;
    } catch (error) {
      // Expected when offline; nothing the user needs to see.
      console.warn("Update check failed:", error);
    }
  }

  async install() {
    if (this.installing) return;
    this.installing = true;

    try {
      await backend.installUpdate();
    } catch (error) {
      console.error("Failed to install the update:", error);
    } finally {
      this.installing = false;
    }
  }
}

export const updater = new UpdaterStore();
