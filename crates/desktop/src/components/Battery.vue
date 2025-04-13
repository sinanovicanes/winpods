<script setup lang="ts">
import { computed } from "vue";
import BatteryIcon from "@/components/icons/BatteryIcon.vue";

interface Props {
  level: number;
  charging?: boolean;
  showWarning?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  showWarning: true
});

const textColorClass = computed(() => {
  // When charging, always use green text
  if (props.charging) {
    return "text-green-500";
  }

  if (!props.showWarning) {
    return "text-gray-700";
  }

  if (props.level <= 10) {
    return "text-red-500";
  } else if (props.level <= 20) {
    return "text-orange-500";
  } else {
    return "text-gray-700";
  }
});
</script>

<template>
  <div class="flex items-center space-x-2">
    <div class="relative">
      <BatteryIcon :level="level" :charging="charging" :showWarning="showWarning" />
    </div>
    <span class="text-sm font-medium" :class="textColorClass"> {{ level }}% </span>
  </div>
</template>
