<template>
  <button
    :class="[
      'rounded-lg font-medium transition-all duration-300 focus:outline-none cursor-pointer flex items-center justify-center gap-1',
      variantStyles[variant],
      sizeStyles[size]
    ]"
    :disabled="disabled || loading"
  >
    <!-- TODO: Update loading icon -->
    <svg
      v-if="loading"
      class="animate-spin h-5 w-5 text-white"
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
    >
      <circle
        class="opacity-25"
        cx="12"
        cy="12"
        r="10"
        stroke="currentColor"
        stroke-width="4"
      ></circle>
      <path
        class="opacity-75"
        fill="currentColor"
        d="M4 12a8 8 0 1 1 16 0A8 8 0 0 1 4 12z"
      ></path>
    </svg>
    <slot></slot>
  </button>
</template>

<script setup lang="ts">
type ButtonVariant = keyof typeof variantStyles;
type ButtonSize = keyof typeof sizeStyles;

interface ButtonProps {
  variant?: ButtonVariant;
  size?: ButtonSize;
  disabled?: boolean;
  loading?: boolean;
}

withDefaults(defineProps<ButtonProps>(), {
  variant: "primary",
  size: "md",
  disabled: false,
  loading: false
});

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
