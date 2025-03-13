<template>
  <img v-if="image" v-bind="$attrs" :src="image" :alt="model" />
</template>

<script setup lang="ts">
import AirpodsPro2UsbC from "@/assets/airpods_pro_2.webp";
import { AirPodsModel } from "@/constants";
import { computed } from "vue";

interface Props {
  model?: keyof typeof AirPodsModel;
}

const props = defineProps<Props>();

function getAirPodsImageFromModel(model: keyof typeof AirPodsModel): string {
  switch (model) {
    case "AirPodsPro2UsbC":
      return AirpodsPro2UsbC;
    default:
      return "";
  }
}

const image = computed(() => props.model && getAirPodsImageFromModel(props.model));
</script>
