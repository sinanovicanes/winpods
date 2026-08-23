import { getVersion } from "@tauri-apps/api/app";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { relaunch } from "@tauri-apps/plugin-process";
import { check } from "@tauri-apps/plugin-updater";

import * as commands from "./commands";
import type { Backend, Unlisten } from "./backend";

/** The real backend: Tauri commands, events and window controls. */
export const tauriBackend: Backend = {
  isLive: true,

  getAdapterState: commands.getAdapterState,
  listDevices: commands.listDevices,
  getCurrentDevice: commands.getCurrentDevice,
  selectDevice: commands.selectDevice,
  clearDeviceSelection: commands.clearDeviceSelection,
  getSettings: commands.getSettings,
  updateSettings: commands.updateSettings,

  async listen<T>(event: string, handler: (payload: T) => void): Promise<Unlisten> {
    return await listen<T>(event, e => handler(e.payload));
  },

  async onWindowFocus(callback: () => void): Promise<Unlisten> {
    return await getCurrentWindow().onFocusChanged(({ payload: focused }) => {
      if (focused) {
        callback();
      }
    });
  },

  hideWindow: () => getCurrentWindow().hide(),
  isAlwaysOnTop: () => getCurrentWindow().isAlwaysOnTop(),
  setAlwaysOnTop: (value: boolean) => getCurrentWindow().setAlwaysOnTop(value),

  getVersion,

  async checkForUpdate() {
    const update = await check();
    return update ? { version: update.version } : null;
  },

  async installUpdate() {
    const update = await check();

    if (!update) {
      throw new Error("No update available");
    }

    await update.downloadAndInstall();
    await relaunch();
  }
};
