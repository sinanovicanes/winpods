<script setup lang="ts">
import Warning from "@/components/Warning.vue";
import { AppView, views } from "@/views";
import { Window } from "@tauri-apps/api/window";
import { computed, onBeforeMount, ref } from "vue";
import { useBluetooth } from "./stores/bluetooth";

const bluetooth = useBluetooth();
const windowLabel = ref<AppView>("main");

onBeforeMount(() => {
  windowLabel.value = Window.getCurrent().label as AppView;
});

const CurrentView = computed(() => views[windowLabel.value] || Warning);
</script>

<template>
  <Warning
    v-if="!bluetooth.isActive"
    title="Disconnected"
    message="Please enable Bluetooth in your system settings."
  />
  <CurrentView class="select-none" v-else />
</template>
