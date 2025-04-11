<script setup lang="ts">
import { Error } from "@/components";
import AirPodsImage from "@/components/AirPodsImage.vue";
import BatteryIcon from "@/components/BatteryIcon.vue";
import { getModelDetails } from "@/models";
import { useDevice } from "@/stores/device";
import { faThumbTack, faXmark } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { computed } from "vue";

const connectedDeviceStore = useDevice();
const device = computed(() => connectedDeviceStore.device);
const deviceProperties = computed(() => connectedDeviceStore.deviceProperties);
const modelDetails = computed(() => getModelDetails(device.value?.model ?? "Unknown"));
</script>

<template>
  <div class="w-[300px] h-[125px] bg-black/20 backdrop-opacity-40 text-white">
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
          <AirPodsImage class="h-[50px]" :model="device.model" />
          <BatteryIcon />
        </div>
        <div
          class="flex flex-col items-center gap-2"
          v-if="modelDetails.widget.caseImage"
        >
          <img
            class="h-[50px]"
            :src="modelDetails.widget.caseImage"
            :alt="`${device.model}-case`"
          />
          <BatteryIcon />
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
