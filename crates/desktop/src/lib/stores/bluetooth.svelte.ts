import { backend, Events } from "$lib/ipc";
import type { AdapterState } from "$lib/ipc";

/**
 * Whether bluetooth is usable.
 *
 * When it is not, both windows show a warning instead of their normal content -- readings would
 * be stale and there is nothing the user can do from inside the app.
 */
class BluetoothStore {
  state = $state<AdapterState>("on");
  ready = $state(false);

  readonly isOn = $derived(this.state === "on");

  #started = false;

  async start() {
    if (this.#started) return;
    this.#started = true;

    await backend.listen<AdapterState>(Events.AdapterStateChanged, state => {
      this.state = state;
    });

    // The adapter may have been unreachable while this window was loading, so its state change
    // was missed. Re-reading on focus keeps the window from being stuck on the warning.
    await backend.onWindowFocus(() => void this.refresh());

    await this.refresh();
  }

  async refresh() {
    try {
      this.state = await backend.getAdapterState();
    } catch (error) {
      console.error("Failed to read the bluetooth adapter state:", error);
      this.state = "off";
    } finally {
      this.ready = true;
    }
  }
}

export const bluetooth = new BluetoothStore();
