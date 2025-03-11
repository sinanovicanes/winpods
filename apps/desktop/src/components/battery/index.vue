<template>
  <div class="flex items-center space-x-2">
    <div class="relative">
      <!-- Battery outline -->
      <svg width="24" height="12" viewBox="0 0 24 12" class="battery-icon">
        <!-- Battery body -->
        <rect
          x="0"
          y="0"
          width="22"
          height="12"
          rx="2"
          ry="2"
          fill="none"
          stroke="currentColor"
          stroke-width="1"
          class="battery-outline"
        />
        <!-- Battery cap -->
        <rect
          x="22"
          y="3.5"
          width="2"
          height="5"
          rx="1"
          ry="1"
          fill="currentColor"
          class="battery-cap"
        />
        <!-- Battery fill based on percentage -->
        <rect
          x="2"
          y="2"
          :width="`${Math.min(18 * (percentage / 100), 18)}`"
          height="8"
          rx="1"
          ry="1"
          :class="batteryFillClass"
        />

        <!-- Lightning bolt for charging state -->
        <g v-if="isCharging" transform="translate(7.5, 0)" class="lightning-bolt">
          <path d="M4.5,0 L0,7 L3,7 L1.5,12 L6,5 L3,5 L4.5,0 Z" fill="white" />
        </g>
      </svg>
    </div>
    <span class="text-sm font-medium" :class="textColorClass"> {{ percentage }}% </span>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";

interface Props {
  percentage: number;
  showWarning?: boolean;
  isCharging?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  showWarning: true,
  isCharging: false
});

const batteryFillClass = computed(() => {
  // When charging, always use green color
  if (props.isCharging) {
    return "fill-green-500";
  }

  if (!props.showWarning) {
    return "fill-green-500";
  }

  if (props.percentage <= 10) {
    return "fill-red-500";
  } else if (props.percentage <= 20) {
    return "fill-orange-500";
  } else {
    return "fill-green-500";
  }
});

const textColorClass = computed(() => {
  // When charging, always use green text
  if (props.isCharging) {
    return "text-green-500";
  }

  if (!props.showWarning) {
    return "text-gray-700";
  }

  if (props.percentage <= 10) {
    return "text-red-500";
  } else if (props.percentage <= 20) {
    return "text-orange-500";
  } else {
    return "text-gray-700";
  }
});
</script>

<style scoped>
.battery-icon {
  filter: drop-shadow(0 1px 1px rgba(0, 0, 0, 0.05));
}

@media (prefers-color-scheme: dark) {
  .battery-outline,
  .battery-cap {
    color: #f3f4f6;
  }
}

@keyframes pulse {
  0% {
    opacity: 1;
  }
  50% {
    opacity: 0.7;
  }
  100% {
    opacity: 1;
  }
}

.lightning-bolt {
  animation: pulse 2s infinite ease-in-out;
}
</style>
