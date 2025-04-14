<script setup lang="ts">
import AirPodsImage from "@/components/AirPodsImage.vue";
import BatteryIcon from "@/components/icons/BatteryIcon.vue";
import Warning from "@/components/Warning.vue";
import { getModelDetails } from "@/models";
import { useDevice } from "@/stores/device";
import { debounce } from "@/utils";
import {
  faThumbTack,
  faThumbTackSlash,
  faXmark
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { Event, listen, TauriEvent, UnlistenFn } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { computed, onBeforeUnmount, onMounted, onUnmounted, ref, watch } from "vue";

const connectedDeviceStore = useDevice();
const device = computed(() => connectedDeviceStore.device);
const deviceProperties = computed(() => connectedDeviceStore.deviceProperties);
const modelDetails = computed(() => getModelDetails(device.value?.model ?? "Unknown"));
const batteryLevel = computed(() => {
  const properties = deviceProperties.value;

  if (!properties) {
    return 0;
  }

  return Math.min(properties.leftBattery.level, properties.rightBattery.level);
});

const batteryCharging = computed(() => {
  const properties = deviceProperties.value;

  if (!properties) {
    return false;
  }

  return properties.leftBattery.charging && properties.rightBattery.charging;
});

let destroyMovedHandler: UnlistenFn | undefined = undefined;

onMounted(async () => {
  destroyMovedHandler = await listen(
    TauriEvent.WINDOW_MOVED,
    debounce((_e: Event<{ x: number; y: number }>) => {
      // TODO: SAVE THE LATEST POSITION
    }, 1000)
  );
});

onUnmounted(() => {
  if (destroyMovedHandler) {
    destroyMovedHandler();
    destroyMovedHandler = undefined;
  }
});

const showDecorations = ref(false);
const widget = ref<HTMLElement | null>(null);
const pinned = ref(false);

watch(pinned, async () => {
  await getCurrentWindow().setAlwaysOnTop(pinned.value);
});

const onMouseEnter = () => {
  showDecorations.value = true;
};
const onMouseLeave = () => {
  showDecorations.value = false;
};

const hideWindow = () => {
  console.log("Hiding window");
  getCurrentWindow().hide();
};

const togglePin = async () => {
  pinned.value = !pinned.value;
};

onMounted(async () => {
  getCurrentWindow()
    .isAlwaysOnTop()
    .then(p => {
      pinned.value = p;
    });
  widget.value?.addEventListener("mouseenter", onMouseEnter);
  widget.value?.addEventListener("mouseleave", onMouseLeave);
});

onBeforeUnmount(() => {
  widget.value?.removeEventListener("mouseenter", onMouseEnter);
  widget.value?.removeEventListener("mouseleave", onMouseLeave);
});
</script>

<template>
  <div
    ref="widget"
    data-tauri-drag-region
    class="w-[300px] h-[125px] bg-black/40 text-white"
  >
    <header v-if="showDecorations" class="absolute top-1 right-1 flex">
      <button
        @click.stop="togglePin"
        class="cursor-pointer hover:bg-gray-700/40 rounded-sm py-0.25 px-2"
      >
        <FontAwesomeIcon size="xs" :icon="pinned ? faThumbTackSlash : faThumbTack" />
      </button>
      <button
        @click.stop="hideWindow"
        class="cursor-pointer hover:bg-red-700/40 rounded-sm py-0.25 px-2"
      >
        <FontAwesomeIcon size="sm" :icon="faXmark" />
      </button>
    </header>
    <template v-if="device">
      <main data-tauri-drag-region class="flex justify-around items-center h-full px-7">
        <div class="flex flex-col items-center gap-2">
          <AirPodsImage class="h-[50px]" :model="device.model" />
          <BatteryIcon
            v-if="deviceProperties"
            :level="batteryLevel"
            :charging="batteryCharging"
          />
          <div v-else class="animate-pulse rounded-lg w-[30px] p-1 bg-gray-100/40"></div>
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
          <BatteryIcon
            v-if="deviceProperties?.caseBattery"
            :level="deviceProperties.caseBattery.level"
            :charging="deviceProperties.caseBattery.charging"
          />
          <div v-else class="animate-pulse rounded-lg w-[30px] p-1 bg-gray-100/40"></div>
        </div>
      </main>
    </template>
    <Warning
      data-tauri-drag-region
      v-else
      title="No Device Selected"
      message="Please select a device from dashboard to view its status."
    />
  </div>
</template>
