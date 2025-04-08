<script setup lang="ts">
import { AppView, views } from "@/views";
import { Window } from "@tauri-apps/api/window";
import { computed, onBeforeMount, ref } from "vue";
import { useBluetooth } from "./stores/bluetooth";

const bluetooth = useBluetooth();
const windowLabel = ref<AppView>("main");

onBeforeMount(() => {
  windowLabel.value = Window.getCurrent().label as AppView;
});

const View = computed(() => views[windowLabel.value] || views.main);
//TODO: Create a warning page for when the bluetooth is not active
</script>

<template>
  <View v-if="bluetooth.isActive" />
  <div v-else>Bluetooth is not active. Please enable Bluetooth to use this feature.</div>
</template>
