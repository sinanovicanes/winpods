<script setup lang="ts">
import { getModelDetails } from "@/models";
import { computed } from "vue";

interface Props {
  model: AirPodsModel;
}

const props = defineProps<Props>();
const modelDetails = computed(() => getModelDetails(props.model));
</script>

<template>
  <div v-if="modelDetails.widget.repeat" v-bind="$attrs" class="flex gap-2">
    <img class="h-full w-full" :src="modelDetails.widget.image" :alt="model" />
    <img
      class="h-full w-full scale-x-[-1]"
      :src="modelDetails.widget.image"
      :alt="model"
    />
  </div>
  <img v-else v-bind="$attrs" :src="modelDetails.widget.image" :alt="model" />
</template>
