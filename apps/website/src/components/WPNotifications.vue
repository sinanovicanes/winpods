<script setup lang="ts">
import { useNotifications } from '@/stores/notification.store';
import WPNotification from './WPNotification.vue';

const notificationStore = useNotifications();
</script>

<template>
  <div class="toast bottom-2 end-2">
    <WPNotification
      v-for="(notification, i) in notificationStore.notifications"
      :key="`${notification}:${i}`"
      :message="notification.message"
      :kind="notification.kind"
    />
  </div>
</template>
