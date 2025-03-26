import { acceptHMRUpdate, defineStore } from 'pinia';
import { ref } from 'vue';

export type NotificationKind = 'info' | 'success' | 'warning' | 'error';
export interface Notification {
  kind: NotificationKind;
  message: string;
}

const NOTIFICATION_TIMEOUT = 5000;

export const useNotifications = defineStore('notification', () => {
  const notifications = ref<Notification[]>([]);

  function add(notification: Notification) {
    notifications.value.push(notification);
    setTimeout(() => notifications.value.pop(), NOTIFICATION_TIMEOUT);
  }

  return {
    notifications,
    add,
  };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useNotifications, import.meta.hot));
}
