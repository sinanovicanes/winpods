<template>
  <img v-if="image" class="w-4/5" :src="image" :alt="model" />
</template>

<script setup lang="ts">
import AirpodsPro2UsbC from "@/assets/airpods_pro_2.webp";
import { computed } from "vue";

interface Props {
  model?: string;
}

const props = defineProps<Props>();

function getAirPodsImageFromModel(model: string): string {
  switch (model) {
    case "AirPodsPro2UsbC":
      return AirpodsPro2UsbC;
    default:
      return "";
  }
}

const image = computed(() => props.model && getAirPodsImageFromModel(props.model));
</script>
