<script setup lang="ts">
import { computed } from "vue";

interface LoadingIconProps {
  size?: string;
  color?: string;
}

const props = withDefaults(defineProps<LoadingIconProps>(), {
  size: "md",
  color: "currentColor"
});

const sizeClass = computed(() => {
  switch (props.size) {
    case "xs":
      return "w-3 h-3";
    case "sm":
      return "w-4 h-4";
    case "md":
      return "w-6 h-6";
    case "lg":
      return "w-8 h-8";
    case "xl":
      return "w-12 h-12";
    default:
      return "w-6 h-6";
  }
});
</script>

<template>
  <svg
    :class="['animate-spin', sizeClass]"
    :style="{ color: color }"
    viewBox="0 0 24 24"
    xmlns="http://www.w3.org/2000/svg"
  >
    <circle
      class="opacity-25"
      cx="12"
      cy="12"
      r="10"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
    />
    <path
      class="opacity-75"
      fill="currentColor"
      d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
    />
  </svg>
</template>
