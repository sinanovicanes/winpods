<script setup lang="ts">
import WPNav from '@/components/WPNav.vue';
import { RouterView } from 'vue-router';
import WPNotifications from './components/WPNotifications.vue';
</script>

<template>
  <header>
    <WPNav />
  </header>
  <RouterView />
  <WPNotifications />
</template>
