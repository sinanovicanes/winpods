import { defineStore } from "pinia";

export const useDeviceSettingsStore = (deviceId: string) =>
  defineStore(`device:${deviceId}:settings`, () => {});
