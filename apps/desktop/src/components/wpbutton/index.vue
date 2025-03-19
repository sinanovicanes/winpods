<template>
  <button
    :class="[
      'rounded-lg font-medium transition-all duration-300 focus:outline-none cursor-pointer',
      variantStyles[variant],
      sizeStyles[size]
    ]"
  >
    <slot></slot>
  </button>
</template>

<script setup lang="ts">
type ButtonVariant = keyof typeof variantStyles;
type ButtonSize = keyof typeof sizeStyles;

interface ButtonProps {
  variant?: ButtonVariant;
  size?: ButtonSize;
}

withDefaults(defineProps<ButtonProps>(), {
  variant: "primary",
  size: "md"
});

const variantStyles = {
  primary: "bg-black text-white hover:bg-black/90",
  secondary: "bg-gray-200 text-black hover:bg-gray-300",
  blue: "bg-blue-600 text-white hover:bg-blue-700",
  danger: "bg-red-500 text-white hover:bg-red-600"
};

const sizeStyles = {
  sm: "text-sm py-2 px-4",
  md: "text-base py-2.5 px-5",
  lg: "text-lg py-3 px-6"
};
</script>
