<script setup lang="ts">
import { Battery, Error } from "@/components";
import { useDevice } from "@/stores/device";
import { faThumbTack, faXmark } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { computed } from "vue";

const connectedDeviceStore = useDevice();
const device = computed(() => connectedDeviceStore.device);
const deviceProperties = computed(() => connectedDeviceStore.deviceProperties);
</script>

<template>
  <div class="w-[300px] h-[100px] bg-black/40 backdrop-opacity-40 text-white">
    <template v-if="device">
      <header class="absolute top-2 right-2 flex">
        <button class="cursor-pointer hover:bg-gray-700 rounded-full py-0.5 px-1">
          <FontAwesomeIcon size="xs" :icon="faThumbTack" />
        </button>
        <button class="cursor-pointer hover:bg-gray-700 rounded-full py-0.5 px-1.5">
          <FontAwesomeIcon size="sm" :icon="faXmark" />
        </button>
      </header>
      <main class="flex justify-around items-center h-full px-7">
        <div class="flex flex-col items-center gap-2">
          <img class="h-[50px] w-[50px]" src="@/assets/airpods-earbuds.webp" />
          <Battery :battery="{ level: 50, charging: false }" />
        </div>
        <div class="flex flex-col items-center" v-if="deviceProperties?.caseBattery">
          <img class="h-[50px] w-[50px]" src="@/assets/airpods-case.webp" />
          <Battery :battery="deviceProperties.caseBattery" />
        </div>
      </main>
    </template>
    <Error
      v-else
      title="Disconnected"
      message="Please enable Bluetooth in your system settings."
    />
  </div>
</template>
