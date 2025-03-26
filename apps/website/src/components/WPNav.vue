<script setup lang="ts">
import GithubIconWhite from '@/assets/icons/github-mark-white.svg';
import GithubIcon from '@/assets/icons/github-mark.svg';
import { useDark } from '@vueuse/core';
import { computed } from 'vue';
import { useRouter } from 'vue-router';
import WPThemeController from './WPThemeController.vue';
import { DownloadService } from '@/service/download.service';

interface Social {
  name: string;
  url: string;
  icon: string;
}

const routes = useRouter().getRoutes();
const isDark = useDark();
const socials = computed<Social[]>(() => [
  {
    name: 'Github',
    url: `https://github.com/${import.meta.env.VITE_GITHUB_REPO}`,
    icon: isDark.value ? GithubIconWhite : GithubIcon,
  },
]);
</script>

<template>
  <nav class="navbar bg-base-100 shadow-sm">
    <div class="navbar-start">
      <img src="/favicon.ico" alt="winpods" width="50px" />
    </div>
    <div class="navbar-center hidden lg:flex">
      <ul class="menu menu-horizontal px-1">
        <template v-for="route in routes" :key="route.path">
          <li>
            <router-link :to="route.path">
              {{ route.meta.title }}
            </router-link>
          </li>
        </template>
      </ul>
    </div>
    <div class="navbar-end gap-2">
      <a
        v-for="social in socials"
        :key="social.name"
        :href="social.url"
        target="_blank"
        rel="noopener noreferrer"
      >
        <img :src="social.icon" :alt="social.name" width="24px" />
      </a>
      <WPThemeController />
      <a class="btn btn-lg" @click.stop="DownloadService.startDownload">Download</a>
    </div>
  </nav>
</template>
