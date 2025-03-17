<script setup lang="ts">
import { AirPodsImage, Battery, Switch, WPButton } from "@/components";
import { AirPodsModel } from "@/constants";
import { useDeviceConnection } from "@/stores/device-connection";
import { useSettings } from "@/stores/settings";
import { computed } from "vue";

const deviceStore = useDeviceConnection();
const settings = useSettings();
const device = computed(() => deviceStore.device);
const availableDevices = computed(() => deviceStore.availableDevices);
</script>

<template>
  <div class="flex flex-col space-y-6 gap-4">
    <h1 class="text-3xl font-semibold text-gray-900">Dashboard</h1>
    <section
      v-if="device && device.properties"
      class="bg-white rounded-2xl shadow-md p-8 border border-gray-100 flex flex-col gap-2"
    >
      <main class="flex justify-between w-full">
        <div class="flex flex-col space-y-4 gap-4">
          <header class="flex justify-between items-center w-full">
            <div>
              <h2 class="text-xl font-medium text-gray-900">
                {{ device.name || "Connected Device" }}
              </h2>
              <p class="text-sm text-gray-500 mt-1">
                {{ AirPodsModel[device.properties.model] }}
              </p>
            </div>
          </header>
          <div class="space-y-5 w-[100px]">
            <div class="flex items-center justify-between">
              <span class="text-gray-700 font-medium">Left</span>
              <Battery :battery="device.properties.leftBattery" />
            </div>

            <div class="flex items-center justify-between">
              <span class="text-gray-700 font-medium">Right</span>
              <Battery :battery="device.properties.rightBattery" />
            </div>

            <div class="flex w-full items-center justify-between">
              <span class="text-gray-700 font-medium">Case</span>
              <Battery
                v-if="device.properties.caseBattery"
                :battery="device.properties.caseBattery"
              />
            </div>
          </div>
        </div>

        <div class="w-[200px] flex justify-center items-center">
          <AirPodsImage :model="device.properties.model" />
        </div>
      </main>
      <footer class="flex flex-col gap-4 justify-start mt-2">
        <div class="flex items-center justify-between w-full">
          <div>
            <p class="text-sm font-medium text-gray-700">Automatic Ear Detection</p>
            <p class="text-xs text-gray-500 mt-1">
              When enabled, audio automatically pauses when AirPods are removed from your
              ears
            </p>
          </div>
          <Switch v-model="settings.earDetection" />
        </div>
        <WPButton variant="danger" @click="deviceStore.disconnect()">Disconnect</WPButton>
      </footer>
    </section>
    <section v-else-if="!!device" class="flex flex-col gap-2">
      <p class="text-gray-500">Connecting...</p>
      <WPButton variant="danger" @click.stop="deviceStore.disconnect()">Cancel</WPButton>
    </section>
    <section v-else class="flex flex-col gap-2">
      <header class="flex justify-between items-end w-full">
        <p class="text-gray-500">Select device to connect</p>
        <WPButton variant="primary" @click.stop="deviceStore.refreshAvailableDevices()"
          >Refresh</WPButton
        >
      </header>
      <select
        @change="deviceStore.connect(Number(($event.target as HTMLSelectElement).value))"
        class="w-full h-10 px-4 py-2 bg-white rounded-lg text-sm font-medium text-gray-800 appearance-none outline-none border border-gray-300 bg-clip-padding shadow-sm cursor-pointer transition-all duration-200 hover:border-gray-400 focus:border-blue-500 focus:ring focus:ring-blue-200 focus:ring-opacity-50"
      >
        <option value="" disabled selected>Select device</option>
        <option
          v-for="device in availableDevices"
          :key="device.address"
          :value="device.address"
          class="py-2 px-4 text-gray-800 font-medium hover:bg-gray-50 focus:bg-blue-50"
        >
          {{ device.name }}
        </option>
      </select>
    </section>
  </div>
</template>
