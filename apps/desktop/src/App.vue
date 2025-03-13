<script setup lang="ts">
import { AppView, views } from "@/views";
import { Window } from "@tauri-apps/api/window";
import { computed, onBeforeMount, ref } from "vue";

const windowLabel = ref<AppView>("main");

onBeforeMount(() => {
  windowLabel.value = Window.getCurrent().label as AppView;
});

const View = computed(() => views[windowLabel.value] || views.main);
</script>

<template>
  <View />
</template>
