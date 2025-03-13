<template>
  <div>
    <h1 class="text-2xl font-medium text-gray-900 mb-8">Settings</h1>

    <div class="bg-white rounded-2xl shadow-sm border border-gray-100 mb-8">
      <!-- General Settings Section -->
      <div class="border-b border-gray-100">
        <div class="px-8 py-6">
          <h2 class="text-lg font-medium text-gray-900">General</h2>
        </div>

        <div class="px-8 pb-6 space-y-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-700">Appearance</p>
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
              <p class="text-sm text-gray-700">Auto Update</p>
              <p class="text-xs text-gray-500 mt-1">
                Keep the application up to date automatically
              </p>
            </div>
            <div class="relative inline-block w-12 align-middle select-none">
              <input
                type="checkbox"
                id="auto-update"
                v-model="autoUpdate"
                class="sr-only"
              />
              <label
                for="auto-update"
                :class="[
                  'block h-6 w-12 rounded-full transition-colors duration-200 ease-in-out cursor-pointer',
                  autoUpdate ? 'bg-blue-500' : 'bg-gray-200'
                ]"
              >
                <span
                  :class="[
                    'block h-5 w-5 mt-0.5 ml-0.5 rounded-full bg-white shadow transform transition-transform duration-200 ease-in-out',
                    autoUpdate ? 'translate-x-6' : 'translate-x-0'
                  ]"
                ></span>
              </label>
            </div>
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
              <p class="text-sm text-gray-700">Enable Notifications</p>
              <p class="text-xs text-gray-500 mt-1">Receive alerts about system events</p>
            </div>
            <div class="relative inline-block w-12 align-middle select-none">
              <input
                type="checkbox"
                id="notifications"
                v-model="notifications"
                class="sr-only"
              />
              <label
                for="notifications"
                :class="[
                  'block h-6 w-12 rounded-full transition-colors duration-200 ease-in-out cursor-pointer',
                  notifications ? 'bg-blue-500' : 'bg-gray-200'
                ]"
              >
                <span
                  :class="[
                    'block h-5 w-5 mt-0.5 ml-0.5 rounded-full bg-white shadow transform transition-transform duration-200 ease-in-out',
                    notifications ? 'translate-x-6' : 'translate-x-0'
                  ]"
                ></span>
              </label>
            </div>
          </div>

          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-700">Sound Alerts</p>
              <p class="text-xs text-gray-500 mt-1">Play sound with notifications</p>
            </div>
            <div class="relative inline-block w-12 align-middle select-none">
              <input type="checkbox" id="sound" v-model="soundAlerts" class="sr-only" />
              <label
                for="sound"
                :class="[
                  'block h-6 w-12 rounded-full transition-colors duration-200 ease-in-out cursor-pointer',
                  soundAlerts ? 'bg-blue-500' : 'bg-gray-200'
                ]"
              >
                <span
                  :class="[
                    'block h-5 w-5 mt-0.5 ml-0.5 rounded-full bg-white shadow transform transition-transform duration-200 ease-in-out',
                    soundAlerts ? 'translate-x-6' : 'translate-x-0'
                  ]"
                ></span>
              </label>
            </div>
          </div>
        </div>
      </div>

      <!-- Privacy Section -->
      <div>
        <div class="px-8 py-6">
          <h2 class="text-lg font-medium text-gray-900">Privacy</h2>
        </div>

        <div class="px-8 pb-6 space-y-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-700">Analytics</p>
              <p class="text-xs text-gray-500 mt-1">Share anonymous usage data</p>
            </div>
            <div class="relative inline-block w-12 align-middle select-none">
              <input type="checkbox" id="analytics" v-model="analytics" class="sr-only" />
              <label
                for="analytics"
                :class="[
                  'block h-6 w-12 rounded-full transition-colors duration-200 ease-in-out cursor-pointer',
                  analytics ? 'bg-blue-500' : 'bg-gray-200'
                ]"
              >
                <span
                  :class="[
                    'block h-5 w-5 mt-0.5 ml-0.5 rounded-full bg-white shadow transform transition-transform duration-200 ease-in-out',
                    analytics ? 'translate-x-6' : 'translate-x-0'
                  ]"
                ></span>
              </label>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="flex justify-end mt-8 space-x-4">
      <button
        class="px-6 py-3 rounded-full border border-gray-300 text-sm font-medium text-gray-700 hover:bg-gray-50 transition-colors duration-200"
      >
        Cancel
      </button>
      <button
        class="bg-blue-500 hover:bg-blue-600 text-white px-6 py-3 rounded-full text-sm font-medium transition-colors duration-200 shadow-sm"
      >
        Save Changes
      </button>
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue";

const theme = ref("light");
const autoUpdate = ref(true);
const notifications = ref(true);
const soundAlerts = ref(false);
const analytics = ref(true);
</script>
