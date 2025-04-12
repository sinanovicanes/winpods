<script setup lang="ts">
import { computed } from "vue";

interface Props {
  level?: number;
  charging?: boolean;
  showWarning?: boolean;
  size?: "xs" | "sm" | "md" | "lg" | "xl" | number;
}

const props = withDefaults(defineProps<Props>(), {
  level: 75,
  charging: false,
  showWarning: true,
  size: "md"
});

const batteryFillClass = computed(() => {
  // When charging, always use green color
  if (props.charging) {
    return "fill-[#37C058]";
  }

  if (!props.showWarning) {
    return "fill-[#37C058]";
  }

  if (props.level <= 10) {
    return "fill-[#FA3532]";
  } else if (props.level <= 20) {
    return "fill-[#FDC633]";
  } else {
    return "fill-[#37C058]";
  }
});

const sizeClass = computed(() => {
  if (typeof props.size === "number") {
    return `w-[${props.size}px] h-[${props.size}px]`;
  } else {
    switch (props.size) {
      case "xs":
        return "w-4";
      case "sm":
        return "w-6";
      case "md":
        return "w-8";
      case "lg":
        return "w-10";
      case "xl":
        return "w-12";
      default:
        return "w-8";
    }
  }
});
</script>

<template>
  <svg
    v-bind="$attrs"
    :class="sizeClass"
    viewBox="0 0 72 36"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
  >
    <rect
      width="63.73"
      height="31.865"
      x="1.611"
      y="2.059"
      stroke="#fff"
      stroke-opacity=".4"
      stroke-width="2.655"
      rx="9.294"
    />
    <rect
      :width="`${Math.min(55.764 * (level / 100), 55.764)}`"
      height="23.899"
      x="5.594"
      y="6.042"
      rx="5.311"
      :class="batteryFillClass"
    />
    <path
      fill="#fff"
      fill-opacity=".4"
      d="M67.997 12.68c1.056 0 2.07.56 2.816 1.556.747.996 1.167 2.347 1.167 3.755 0 1.409-.42 2.76-1.167 3.756-.747.996-1.76 1.555-2.816 1.555V12.68Z"
    />
    <path
      v-if="charging"
      class="animate-pulse"
      fill="#fff"
      d="M24.182 19.893c0 .328.11.601.328.82.218.209.489.313.811.313h8.415l-4.414 12.318c-.152.428-.18.795-.085 1.104.104.308.28.532.527.67.246.14.522.17.826.09.313-.08.602-.293.868-.641l13.569-17.448c.266-.348.399-.696.399-1.044 0-.328-.11-.596-.328-.805-.209-.219-.48-.328-.811-.328h-8.4l4.399-12.318c.161-.428.19-.79.085-1.089-.095-.308-.266-.532-.513-.671-.246-.14-.526-.17-.84-.09-.303.08-.588.289-.854.627L24.595 18.864c-.275.348-.413.69-.413 1.029Z"
    />
  </svg>
</template>
