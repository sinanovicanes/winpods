<script setup lang="ts">
import { ref, computed } from "vue";
import { MainPage, SettingsPage } from "./pages";
import { WPButton } from "@/components";
import { useUpdater } from "@/stores/updater";

enum DashboardPages {
  Main,
  Settings
}

interface NavItem {
  label: string;
  page: DashboardPages;
}

const navItems: NavItem[] = [
  {
    label: "Dashboard",
    page: DashboardPages.Main
  },
  {
    label: "Settings",
    page: DashboardPages.Settings
  }
];

const updater = useUpdater();
const activePage = ref<DashboardPages>(DashboardPages.Main);
const currentPage = computed(() => {
  switch (activePage.value) {
    case DashboardPages.Settings:
      return SettingsPage;
    default:
      return MainPage;
  }
});
</script>

<template>
  <div class="h-screen w-full bg-gray-50 font-sans">
    <div
      class="max-w-screen-lg h-[600px] w-[800px] mx-auto bg-white shadow-md rounded-2xl overflow-hidden flex flex-col"
    >
      <nav class="bg-white border-b border-gray-200 px-8 h-16 flex items-center">
        <div class="flex space-x-10 w-full gap-2">
          <button
            v-for="item in navItems"
            :key="item.label"
            @click="activePage = item.page"
            :class="[
              'text-sm font-medium transition-colors duration-200 relative py-2 cursor-pointer',
              item.page === activePage
                ? 'text-blue-500'
                : 'text-gray-600 hover:text-gray-900'
            ]"
          >
            <span>{{ item.label }}</span>
            <span
              v-if="activePage === item.page"
              class="absolute bottom-0 left-0 w-full h-0.5 bg-blue-500 rounded-full"
            ></span>
          </button>
        </div>
      </nav>
      <main class="flex-1 overflow-y-auto p-8">
        <component :is="currentPage" />
      </main>
      <footer
        class="bg-white border-t border-gray-200 px-8 py-2 flex items-center justify-between"
      >
        <span>v{{ updater.currentVersion }}</span>
        <WPButton
          v-if="updater.isUpdateAvailable"
          :loading="updater.isUpdating"
          @click.stop="updater.update"
          variant="blue"
          size="xs"
          >Update is available</WPButton
        >
      </footer>
    </div>
  </div>
</template>
