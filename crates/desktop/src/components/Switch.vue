<template>
  <div class="inline-block my-2 mx-3">
    <label class="relative inline-block w-12 h-7">
      <input
        type="checkbox"
        :checked="modelValue"
        @change="$emit('update:modelValue', ($event.target as HTMLInputElement).checked)"
        class="opacity-0 w-0 h-0"
      />
      <span
        class="absolute cursor-pointer inset-0 rounded-full transition-colors duration-300 ease-in-out"
        :class="modelValue ? 'bg-green-500' : 'bg-gray-300 bg-opacity-40'"
      >
        <span
          class="absolute w-6 h-6 bg-white rounded-full shadow-md transition-transform duration-300 ease-in-out left-0.5 top-0.5"
          :class="modelValue ? 'transform translate-x-5' : ''"
        ></span>
      </span>
    </label>
  </div>
</template>

<script setup lang="ts">
interface SwitchProps {
  modelValue: boolean;
}

defineProps<SwitchProps>();
defineEmits<{
  (e: "update:modelValue", value: boolean): void;
}>();
</script>
