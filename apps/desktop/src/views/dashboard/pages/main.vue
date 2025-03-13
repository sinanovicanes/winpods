<script setup lang="ts">
import { AirPodsImage, Battery } from "@/components";
import { AirPodsModel } from "@/constants";
import { useConnectedDevice } from "@/stores/connected-device";
import { computed, ref } from "vue";

const deviceStore = useConnectedDevice();
const device = computed(() => deviceStore.device);
const autoDetection = ref(true);
</script>

<template>
  <div class="flex flex-col space-y-6 gap-4">
    <h1 class="text-3xl font-semibold text-gray-900">Dashboard</h1>

    <section
      v-if="device"
      class="bg-white rounded-2xl shadow-md p-8 border border-gray-100 flex flex-col gap-2"
    >
      <main class="flex justify-between w-full">
        <div class="flex flex-col space-y-4 gap-4">
          <header class="flex justify-between items-center w-full">
            <div>
              <h2 class="text-xl font-medium text-gray-900">
                {{ device.name || "Connected Device" }}
              </h2>
              <p class="text-sm text-gray-500 mt-1">{{ AirPodsModel[device.model] }}</p>
            </div>
          </header>
          <div class="space-y-5 w-[100px]">
            <div class="flex items-center justify-between">
              <span class="text-gray-700 font-medium">Left</span>
              <Battery :battery="device.leftBattery" />
            </div>

            <div class="flex items-center justify-between">
              <span class="text-gray-700 font-medium">Right</span>
              <Battery :battery="device.rightBattery" />
            </div>

            <div class="flex w-full items-center justify-between">
              <span class="text-gray-700 font-medium">Case</span>
              <Battery v-if="device.caseBattery" :battery="device.caseBattery" />
            </div>
          </div>
        </div>

        <div class="w-[200px] flex justify-center items-center">
          <AirPodsImage model="AirPodsPro2UsbC" />
        </div>
      </main>
      <footer class="flex gap-4 justify-start mt-2">
        <div class="flex items-center justify-between w-full">
          <div>
            <p class="text-sm font-medium text-gray-700">Automatic Ear Detection</p>
            <p class="text-xs text-gray-500 mt-1">
              When enabled, audio automatically pauses when AirPods are removed from your
              ears
            </p>
          </div>
          <div class="relative inline-block w-12 align-middle select-none">
            <input type="checkbox" id="sound" v-model="autoDetection" class="sr-only" />
            <label
              for="sound"
              :class="[
                'block h-6 w-12 rounded-full transition-colors duration-200 ease-in-out cursor-pointer',
                autoDetection ? 'bg-blue-500' : 'bg-gray-200'
              ]"
            >
              <span
                :class="[
                  'block h-5 w-5 mt-0.5 ml-0.5 rounded-full bg-white shadow transform transition-transform duration-200 ease-in-out',
                  autoDetection ? 'translate-x-6' : 'translate-x-0'
                ]"
              ></span>
            </label>
          </div>
        </div>
      </footer>
    </section>
  </div>
</template>
