<template>
  <img v-if="image" v-bind="$attrs" :src="image" :alt="model" />
</template>

<script setup lang="ts">
import AirPods from "@/assets/airpods.webp";
import AirPodsPro from "@/assets/airpods_pro_2.webp";
import AirPodsMax from "@/assets/airpods_max.png";
import { AirPodsModel } from "@/constants";
import { computed } from "vue";

interface Props {
  model?: keyof typeof AirPodsModel;
}

const props = defineProps<Props>();

function getAirPodsImageFromModel(model: keyof typeof AirPodsModel): string {
  switch (model) {
    case "AirPods1":
    case "AirPods2":
    case "AirPods3":
      return AirPods;
    case "AirPodsPro":
    case "AirPodsPro2":
    case "AirPodsPro2UsbC":
      return AirPodsPro;
    case "AirPodsMax":
      return AirPodsMax;
    default:
      return "";
  }
}

const image = computed(() => props.model && getAirPodsImageFromModel(props.model));
</script>
