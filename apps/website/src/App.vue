<script setup lang="ts">
import WPNav from '@/components/WPNav.vue';
import { RouterView } from 'vue-router';
import WPNotifications from './components/WPNotifications.vue';
</script>

<template>
  <header>
    <WPNav />
  </header>
  <main class="py-5 min-h-[87vh]">
    <RouterView />
  </main>
  <WPNotifications />
  <footer class="footer sm:footer-horizontal footer-center bg-base-200 text-base-content p-4">
    <aside>
      <p>Copyright © {{ new Date().getFullYear() }} - All rights reserved</p>
    </aside>
  </footer>
</template>
