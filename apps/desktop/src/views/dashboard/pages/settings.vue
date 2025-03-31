<script setup lang="ts">
import { Switch } from "@/components";
import { useSettings } from "@/stores/settings";

const settings = useSettings();
const batteryThresholdOptions = [
  { value: 0, label: "Disabled" },
  ...Array.from({ length: 9 }, (_, i) => ({
    value: (i + 1) * 10,
    label: `${(i + 1) * 10}%`
  }))
];
</script>

<template>
  <div class="flex flex-col space-y-6 gap-4">
    <h1 class="text-3xl font-semibold text-gray-900">Settings</h1>
    <div class="bg-white rounded-2xl shadow-sm border border-gray-100 mb-8">
      <section class="border-b border-gray-100">
        <div class="px-8 py-6">
          <h2 class="text-lg font-medium text-gray-900">General</h2>
        </div>

        <div class="px-8 pb-6 space-y-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium text-gray-700">Auto Start</p>
              <p class="text-xs text-gray-500 mt-1">
                Start the application automatically when you start your computer
              </p>
            </div>
            <Switch v-model="settings.autoStart" />
          </div>
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium text-gray-700">Auto Update</p>
              <p class="text-xs text-gray-500 mt-1">
                Keep the application up to date automatically
              </p>
            </div>
            <Switch v-model="settings.autoUpdate" />
          </div>
        </div>
      </section>
      <section>
        <div class="px-8 py-6">
          <h2 class="text-lg font-medium text-gray-900">Notifications</h2>
        </div>

        <div class="px-8 pb-6 space-y-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium text-gray-700">Low battery</p>
              <p class="text-xs text-gray-500 mt-1">
                Sends a notification when the battery level is equal to or lower than the
                threshold value
              </p>
            </div>
            <div class="relative">
              <select
                class="h-9 min-w-24 text-sm text-end font-medium text-gray-700 bg-gray-50 border border-gray-200 rounded-lg pl-3 pr-2 outline-none appearance-none cursor-pointer transition-all"
                v-model="settings.lowBatteryThreshold"
              >
                <option
                  v-for="option in batteryThresholdOptions"
                  :key="option.value"
                  :value="option.value"
                  class="py-1 text-end cursor-pointer"
                >
                  {{ option.label }}
                </option>
              </select>
            </div>
          </div>
        </div>
      </section>
    </div>
  </div>
</template>
