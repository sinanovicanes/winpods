<script setup lang="ts">
import { AppView, views } from "@/views";
import { Window } from "@tauri-apps/api/window";
import { computed, onBeforeMount, ref } from "vue";
import { useBluetooth } from "./stores/bluetooth";
import { Error } from "./components";

const bluetooth = useBluetooth();
const windowLabel = ref<AppView>("main");

onBeforeMount(() => {
  windowLabel.value = Window.getCurrent().label as AppView;
});

const CurrentView = computed(() => views[windowLabel.value] || Error);
</script>

<template>
  <Error
    v-if="!bluetooth.isActive"
    title="Disconnected"
    message="Please enable Bluetooth in your system settings."
  />
  <CurrentView class="select-none" v-else />
</template>
