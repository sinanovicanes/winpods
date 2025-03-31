<template>
  <button
    :class="[
      'rounded-lg font-medium transition-all duration-300 focus:outline-none  flex items-center justify-center gap-1',
      variantStyles[variant],
      sizeStyles[size],
      isDisabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'
    ]"
    :disabled="isDisabled"
  >
    <LoadingIcon v-if="loading" />
    <slot></slot>
  </button>
</template>

<script setup lang="ts">
import { computed } from "vue";
import LoadingIcon from "../icons/LoadingIcon.vue";

type ButtonVariant = keyof typeof variantStyles;
type ButtonSize = keyof typeof sizeStyles;

interface ButtonProps {
  variant?: ButtonVariant;
  size?: ButtonSize;
  disabled?: boolean;
  loading?: boolean;
}

const props = withDefaults(defineProps<ButtonProps>(), {
  variant: "primary",
  size: "md",
  disabled: false,
  loading: false
});

const isDisabled = computed(() => props.disabled || props.loading);
const variantStyles = {
  primary: "bg-black text-white hover:bg-black/90",
  secondary: "bg-gray-200 text-black hover:bg-gray-300",
  blue: "bg-blue-500 text-white hover:bg-blue-600",
  danger: "bg-red-500 text-white hover:bg-red-600"
};

const sizeStyles = {
  xs: "text-xs py-1 px-2",
  sm: "text-sm py-2 px-4",
  md: "text-base py-2.5 px-5",
  lg: "text-lg py-3 px-6"
};
</script>
