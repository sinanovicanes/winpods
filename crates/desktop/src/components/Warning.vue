<script setup lang="ts">
interface Props {
  title?: string;
  message?: string;
}

withDefaults(defineProps<Props>(), {
  title: "Error",
  message: "An error occurred while connecting to the device."
});
</script>

<template>
  <div
    class="rounded-xl shadow-lg p-6 h-screen w-full flex flex-col justify-center items-center"
  >
    <div class="text-red-500 mb-2">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="h-8 w-8"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
        />
      </svg>
    </div>
    <p class="text-gray-700 font-medium">{{ title }}</p>
    <p class="text-gray-500 text-sm mt-1 text-center">
      {{ message }}
    </p>
  </div>
</template>
