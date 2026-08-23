import { backend, Events } from "$lib/ipc";
import type { UpdateStatus } from "$lib/ipc";

const UNKNOWN: UpdateStatus = {
  current: "0.0.0",
  available: null,
  installing: false,
  progress: null,
  error: null
};

/**
 * Mirrors the backend's update state.
 *
 * Deliberately has no timer and no check of its own. Rust polls the endpoint and pushes the result
 * here, so there is exactly one update check in the app — previously the backend auto-installed
 * while this store independently offered a manual install of the same update.
 */
class UpdaterStore {
  status = $state<UpdateStatus>({ ...UNKNOWN });

  readonly currentVersion = $derived(this.status.current);
  readonly latestVersion = $derived(this.status.available);
  readonly updateAvailable = $derived(this.status.available !== null);
  readonly installing = $derived(this.status.installing);
  readonly progress = $derived(this.status.progress);

  #started = false;

  async start() {
    if (this.#started) return;
    this.#started = true;

    await backend.listen<UpdateStatus>(Events.UpdateStatusChanged, status => {
      this.status = status;
    });

    await this.refresh();
  }

  async refresh() {
    try {
      this.status = await backend.getUpdateStatus();
    } catch (error) {
      console.error("Failed to read the update status:", error);
    }
  }

  /** Asks the backend to install. It restarts the app on success, so this may never resolve. */
  async install() {
    if (this.status.installing) return;

    try {
      await backend.installUpdate();
    } catch (error) {
      console.error("Failed to install the update:", error);
      // The backend records the failure in its own status; pick it up.
      await this.refresh();
    }
  }
}

export const updater = new UpdaterStore();
