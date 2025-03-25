<script setup lang="ts">
import WPNav from '@/components/WPNav.vue';
import { RouterView } from 'vue-router';
</script>

<template>
  <header>
    <WPNav />
  </header>
  <RouterView />
</template>
