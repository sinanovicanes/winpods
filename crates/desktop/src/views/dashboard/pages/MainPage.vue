<script setup lang="ts">
import Battery from "@/components/Battery.vue";
import Switch from "@/components/Switch.vue";
import Button from "@/components/Button.vue";
import { getModelDetails } from "@/models";
import { useDevice } from "@/stores/device";
import { useSettings } from "@/stores/settings";
import { computed } from "vue";

const deviceStore = useDevice();
const settings = useSettings();
const device = computed(() => deviceStore.device);
const deviceProperties = computed(() => deviceStore.deviceProperties);
const availableDevices = computed(() => deviceStore.availableDevices);
const modelDetails = computed(() => getModelDetails(device.value?.model ?? "Unknown"));
const connectionLabel = computed(() =>
  device.value?.connectionState === "connected" ? "Connected" : "Selected"
);
const hasBatteryData = computed(
  () =>
    !!deviceProperties.value &&
    (deviceProperties.value.leftBattery.level > 0 ||
      deviceProperties.value.rightBattery.level > 0 ||
      (deviceProperties.value.caseBattery?.level ?? 0) > 0)
);
const batteryUpdatedAt = computed(() => {
  if (!deviceProperties.value?.updatedAtUnixMs) {
    return "";
  }

  return new Date(deviceProperties.value.updatedAtUnixMs).toLocaleTimeString([], {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit"
  });
});
const noiseModes = ["Transparency", "Noise Cancellation", "Off"];
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
              <p class="text-sm text-gray-500 mt-1 flex items-center gap-2">
                <span>{{ modelDetails.name }}</span>
                <span
                  class="rounded-full px-2 py-0.5 text-xs"
                  :class="
                    device.connectionState === 'connected'
                      ? 'bg-green-50 text-green-700'
                      : 'bg-yellow-50 text-yellow-700'
                  "
                >
                  {{ connectionLabel }}
                </span>
              </p>
            </div>
          </header>
          <p v-if="!hasBatteryData" class="text-xs text-gray-500">
            Waiting for AirPods battery broadcast.
          </p>
          <p v-else class="text-xs text-gray-500">
            Battery from AirPods broadcast at {{ batteryUpdatedAt }}.
          </p>
          <div class="space-y-5 w-[100px]">
            <div class="flex items-center justify-between">
              <span class="text-gray-700 font-medium">Left</span>
              <Battery
                v-if="deviceProperties"
                :level="deviceProperties.leftBattery.level"
                :charging="deviceProperties.leftBattery.charging"
              />
              <div v-else class="animate-pulse rounded-lg w-1/2 p-1 bg-gray-200"></div>
            </div>
            <div class="flex items-center justify-between">
              <span class="text-gray-700 font-medium">Right</span>
              <Battery
                v-if="deviceProperties"
                :level="deviceProperties.rightBattery.level"
                :charging="deviceProperties.rightBattery.charging"
              />
              <div v-else class="animate-pulse rounded-lg w-1/2 p-1 bg-gray-200"></div>
            </div>
            <div class="flex w-full items-center justify-between">
              <span class="text-gray-700 font-medium">Case</span>
              <Battery
                v-if="deviceProperties?.caseBattery"
                :level="deviceProperties.caseBattery.level"
                :charging="deviceProperties.caseBattery.charging"
              />
              <div
                v-else-if="!deviceProperties"
                class="animate-pulse rounded-lg w-1/2 p-1 bg-gray-200"
              ></div>
            </div>
          </div>
        </div>
        <div class="w-[200px] flex justify-center items-center">
          <img :src="modelDetails.image" :alt="device.model" />
        </div>
      </main>
      <footer class="flex flex-col gap-4 justify-start mt-2">
        <div class="flex flex-col gap-3 rounded-lg border border-gray-100 bg-gray-50 p-4">
          <div class="flex items-start justify-between gap-4">
            <div>
              <p class="text-sm font-medium text-gray-700">Noise Control</p>
              <p class="text-xs text-gray-500 mt-1">
                AirPods mode switching is not available from Windows yet.
              </p>
            </div>
            <span class="rounded-full bg-yellow-50 px-2 py-0.5 text-xs text-yellow-700">
              Planned
            </span>
          </div>
          <div class="grid grid-cols-3 gap-2">
            <button
              v-for="mode in noiseModes"
              :key="mode"
              disabled
              class="rounded-lg border border-gray-200 bg-white px-3 py-2 text-xs font-medium text-gray-400"
            >
              {{ mode }}
            </button>
          </div>
        </div>
        <div class="flex flex-col gap-3 rounded-lg border border-gray-100 bg-gray-50 p-4">
          <div>
            <p class="text-sm font-medium text-gray-700">Device Switching</p>
            <p class="text-xs text-gray-500 mt-1">
              Experimental service toggles may connect or disconnect AirPods on Windows.
            </p>
          </div>
          <div class="flex flex-wrap gap-3">
            <Button
              variant="primary"
              size="sm"
              :loading="deviceStore.serviceActionName === 'connect'"
              :disabled="deviceStore.isServiceActionPending"
              @click="deviceStore.connectServices()"
            >
              Connect
            </Button>
            <Button
              variant="secondary"
              size="sm"
              :loading="deviceStore.serviceActionName === 'disconnect'"
              :disabled="deviceStore.isServiceActionPending"
              @click="deviceStore.disconnectServices()"
            >
              Disconnect
            </Button>
            <Button
              variant="secondary"
              size="sm"
              :disabled="deviceStore.isServiceActionPending"
              @click="deviceStore.disconnect()"
            >
              Clear selection
            </Button>
            <Button variant="primary" size="sm" @click="deviceStore.openBluetoothSettings()">
              Bluetooth settings
            </Button>
            <Button variant="secondary" size="sm" @click="deviceStore.openSoundSettings()">
              Sound settings
            </Button>
          </div>
          <p v-if="deviceStore.serviceActionStatus" class="text-xs text-gray-600">
            {{ deviceStore.serviceActionStatus }}
          </p>
        </div>
        <div class="flex items-center justify-between w-full">
          <div>
            <p class="text-sm font-medium text-gray-700">Automatic Ear Detection</p>
            <p class="text-xs text-gray-500 mt-1">
              Paused until AirPods in-ear broadcasts are reliable on Windows.
            </p>
          </div>
          <div class="flex items-center gap-2">
            <span class="rounded-full bg-gray-100 px-2 py-0.5 text-xs text-gray-500">
              Off
            </span>
            <Switch v-model="settings.earDetection" disabled />
          </div>
        </div>
      </footer>
    </section>
    <section v-else class="flex flex-col gap-2">
      <header class="flex justify-between items-end w-full">
        <p class="text-gray-500">Select device to connect</p>
        <Button variant="primary" @click.stop="deviceStore.refreshAvailableDevices()">
          Refresh
        </Button>
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
