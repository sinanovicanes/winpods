import { backend, Events } from "$lib/ipc";
import type { Settings, SettingsPatch } from "$lib/ipc";

const DEFAULTS: Settings = {
  autoStart: true,
  autoUpdate: true,
  earDetection: true,
  lowBatteryThreshold: 20
};

/**
 * The user's settings.
 *
 * Every change goes through `update`, which sends a patch and adopts whatever the backend
 * returns. That matters: the backend clamps out-of-range values, so the response is the truth
 * and not necessarily what was sent.
 */
class SettingsStore {
  current = $state<Settings>({ ...DEFAULTS });
  loading = $state(true);
  saving = $state(false);

  #started = false;

  async start() {
    if (this.#started) return;
    this.#started = true;

    await backend.listen<Settings>(Events.SettingsChanged, settings => {
      this.current = settings;
    });

    try {
      this.current = await backend.getSettings();
    } catch (error) {
      // Keep the defaults; otherwise the settings page would not be editable at all.
      console.error("Failed to read the settings:", error);
    } finally {
      this.loading = false;
    }
  }

  /** Applies a partial change and adopts the stored result. */
  async update(patch: SettingsPatch) {
    // Apply optimistically so a toggle feels instant, then reconcile with the backend.
    const previous = this.current;
    this.current = { ...this.current, ...patch };
    this.saving = true;

    try {
      this.current = await backend.updateSettings(patch);
    } catch (error) {
      console.error("Failed to update the settings:", error);
      this.current = previous;
    } finally {
      this.saving = false;
    }
  }
}

export const settings = new SettingsStore();
