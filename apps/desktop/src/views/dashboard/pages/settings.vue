<template>
  <div class="flex flex-col space-y-6 gap-4">
    <h1 class="text-3xl font-semibold text-gray-900">Settings</h1>

    <div class="bg-white rounded-2xl shadow-sm border border-gray-100 mb-8">
      <!-- General Settings Section -->
      <div class="border-b border-gray-100">
        <div class="px-8 py-6">
          <h2 class="text-lg font-medium text-gray-900">General</h2>
        </div>

        <div class="px-8 pb-6 space-y-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium text-gray-700">Appearance</p>
              <p class="text-xs text-gray-500 mt-1">Choose your preferred theme</p>
            </div>
            <select
              id="app-theme"
              v-model="theme"
              class="rounded-lg border border-gray-300 bg-white py-2 px-4 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            >
              <option value="light">Light</option>
              <option value="dark">Dark</option>
              <option value="system">System Default</option>
            </select>
          </div>

          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium text-gray-700">Auto Update</p>
              <p class="text-xs text-gray-500 mt-1">
                Keep the application up to date automatically
              </p>
            </div>
            <Switch v-model="autoUpdate" />
          </div>
        </div>
      </div>

      <!-- Notification Settings Section -->
      <div class="border-b border-gray-100">
        <div class="px-8 py-6">
          <h2 class="text-lg font-medium text-gray-900">Notifications</h2>
        </div>

        <div class="px-8 pb-6 space-y-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium text-gray-700">Enable Notifications</p>
              <p class="text-xs text-gray-500 mt-1">Receive alerts about system events</p>
            </div>
            <Switch v-model="notifications" />
          </div>

          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium text-gray-700">Sound Alerts</p>
              <p class="text-xs text-gray-500 mt-1">Play sound with notifications</p>
            </div>
            <Switch v-model="soundAlerts" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { Switch } from "@/components";
import { ref } from "vue";

const theme = ref("light");
const autoUpdate = ref(true);
const notifications = ref(true);
const soundAlerts = ref(false);
const analytics = ref(true);
</script>
