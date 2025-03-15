<script setup lang="ts">
import { computed } from "vue";
import { AirPodsImage, Battery } from "@/components";
import { useDeviceConnection } from "@/stores/device-connection";
import { AirPodsModel } from "@/constants";

const connectedDeviceStore = useDeviceConnection();
const device = computed(() => connectedDeviceStore.device);
</script>

<template>
  <main
    class="bg-gradient-to-b from-gray-50 to-gray-200 h-screen w-screen flex flex-col items-center justify-center p-4"
  >
    <div
      v-if="device && device.properties"
      class="bg-white rounded-xl shadow-lg p-6 max-w-xs w-full"
    >
      <h1 class="text-center text-xl font-semibold text-gray-800 mb-2">
        {{ device.name || AirPodsModel[device.properties.model] }}
      </h1>
      <div class="flex justify-center mb-6">
        <AirPodsImage class="w-4/5" :model="device.properties.model" />
      </div>

      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <span class="text-gray-700 font-medium">Left</span>
          <div class="flex items-center">
            <Battery :battery="device.properties.leftBattery" />
          </div>
        </div>

        <div class="flex items-center justify-between">
          <span class="text-gray-700 font-medium">Right</span>
          <div class="flex items-center">
            <Battery :battery="device.properties.rightBattery" />
          </div>
        </div>

        <div class="flex items-center justify-between">
          <span class="text-gray-700 font-medium">Case</span>
          <div class="flex items-center">
            <Battery
              v-if="device.properties.caseBattery"
              :battery="device.properties.caseBattery"
            />
          </div>
        </div>
      </div>
    </div>

    <div
      v-else
      class="bg-white rounded-xl shadow-lg p-6 max-w-xs w-full flex flex-col items-center"
    >
      <div class="text-red-500 mb-2">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="h-8 w-8"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
          />
        </svg>
      </div>
      <p class="text-gray-700 font-medium">Disconnected</p>
      <p class="text-gray-500 text-sm mt-1">Waiting for AirPods connection...</p>
    </div>
  </main>
</template>
