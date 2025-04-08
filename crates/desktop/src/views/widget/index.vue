<script setup lang="ts">
import { computed } from "vue";
import { AirPodsImage, Battery, Error } from "@/components";
import { useDevice } from "@/stores/device";
import { AirPodsModel } from "@/constants";

const connectedDeviceStore = useDevice();
const device = computed(() => connectedDeviceStore.device);
const deviceProperties = computed(() => connectedDeviceStore.deviceProperties);
</script>

<template>
  <main
    class="bg-gradient-to-b from-gray-50 to-gray-200 h-screen w-screen flex flex-col items-center justify-center p-4"
  >
    <div v-if="device" class="bg-white rounded-xl shadow-lg p-6 max-w-xs w-full">
      <h1 class="text-center text-xl font-semibold text-gray-800 mb-2">
        {{ device.name || AirPodsModel[device.model] }}
      </h1>
      <div class="flex justify-center mb-6">
        <AirPodsImage class="w-4/5" :model="device.model" />
      </div>
      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <span class="text-gray-700 font-medium">Left</span>
          <div class="flex items-center">
            <Battery v-if="deviceProperties" :battery="deviceProperties.leftBattery" />
            <div v-else class="animate-pulse rounded-lg w-[30px] p-1 bg-gray-200"></div>
          </div>
        </div>
        <div class="flex items-center justify-between">
          <span class="text-gray-700 font-medium">Right</span>
          <div class="flex items-center">
            <Battery v-if="deviceProperties" :battery="deviceProperties.rightBattery" />
            <div v-else class="animate-pulse rounded-lg w-[30px] p-1 bg-gray-200"></div>
          </div>
        </div>
        <div class="flex items-center justify-between">
          <span class="text-gray-700 font-medium">Case</span>
          <div class="flex items-center">
            <Battery
              v-if="deviceProperties?.caseBattery"
              :battery="deviceProperties.caseBattery"
            />
            <div
              v-else-if="!deviceProperties"
              class="animate-pulse rounded-lg w-[30px] p-1 bg-gray-200"
            ></div>
          </div>
        </div>
      </div>
    </div>
    <Error
      v-else
      title="No device selected"
      message="Please select a device from the dashboard."
    />
  </main>
</template>
