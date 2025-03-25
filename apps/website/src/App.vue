<script setup lang="ts">
import { RouterView } from 'vue-router';
import WPNav from './components/WPNav.vue';
</script>

<template>
  <header>
    <WPNav />
  </header>
  <RouterView />
</template>
