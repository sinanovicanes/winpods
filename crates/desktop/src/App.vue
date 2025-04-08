<script setup lang="ts">
import { AppView, ErrorView, views } from "@/views";
import { Window } from "@tauri-apps/api/window";
import { computed, onBeforeMount, ref } from "vue";
import { useBluetooth } from "./stores/bluetooth";

const bluetooth = useBluetooth();
const windowLabel = ref<AppView>("main");

onBeforeMount(() => {
  windowLabel.value = Window.getCurrent().label as AppView;
});

const CurrentView = computed(() => views[windowLabel.value] || views.error);
</script>

<template>
  <ErrorView
    v-if="!bluetooth.isActive"
    title="Disconnected"
    message="Please enable Bluetooth in your system settings."
  />
  <CurrentView v-else />
</template>
