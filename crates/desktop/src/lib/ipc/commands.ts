import { invoke, type InvokeArgs } from "@tauri-apps/api/core";

import type {
  AdapterState,
  AvailableDevice,
  BluetoothAddress,
  DeviceInfo,
  DeviceSnapshot,
  Settings,
  SettingsPatch
} from "./types";

const RETRY_ATTEMPTS = 5;
const RETRY_BASE_DELAY = 200;

const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

/**
 * Invokes a command, retrying a few times with exponential backoff before giving up.
 *
 * The windows declared in `tauri.conf.json` are created before the backend finishes its setup,
 * so the first calls from a freshly loaded webview can land too early. Failing silently would
 * leave the UI showing its defaults until the app is restarted.
 */
async function call<T>(
  command: string,
  args?: InvokeArgs,
  attempts = RETRY_ATTEMPTS
): Promise<T> {
  for (let attempt = 0; ; attempt++) {
    try {
      return await invoke<T>(command, args);
    } catch (error) {
      if (attempt >= attempts) {
        throw error;
      }

      console.warn(`[${command}] failed, retrying (${attempt + 1}/${attempts}):`, error);
      await sleep(RETRY_BASE_DELAY * 2 ** attempt);
    }
  }
}

export const getAdapterState = () => call<AdapterState>("get_adapter_state");

export const listDevices = () => call<AvailableDevice[]>("list_devices");

export const getCurrentDevice = () => call<DeviceSnapshot>("get_current_device");

export const selectDevice = (address: BluetoothAddress) =>
  call<DeviceInfo>("select_device", { address });

export const clearDeviceSelection = () => call<void>("clear_device_selection");

export const getSettings = () => call<Settings>("get_settings");

/** Applies a partial settings change and resolves with what the backend actually stored. */
export const updateSettings = (patch: SettingsPatch) =>
  call<Settings>("update_settings", { patch });
