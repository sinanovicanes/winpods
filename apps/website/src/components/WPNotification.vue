<script setup lang="ts">
import type { Notification, NotificationKind } from '@/stores/notification.store';
import SuccessIcon from './icons/SuccessIcon.vue';
import InfoIcon from './icons/InfoIcon.vue';
import ErrorIcon from './icons/ErrorIcon.vue';
import WarningIcon from './icons/WarningIcon.vue';
import type { Component } from 'vue';

defineProps<Notification>();

const icons: Record<NotificationKind, Component> = {
  success: SuccessIcon,
  info: InfoIcon,
  error: ErrorIcon,
  warning: WarningIcon,
};
</script>

<template>
  <div :class="`alert alert-${kind}`">
    <component :is="icons[kind]" />
    <span>{{ message }}</span>
  </div>
</template>
