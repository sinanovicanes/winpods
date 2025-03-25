<script setup lang="ts">
import { RouterView } from 'vue-router';
import WPNav from './components/WPNav.vue';
import GithubIcon from '@/assets/icons/github-mark-white.svg';
import WPButton from './components/WPButton.vue';

interface Social {
  name: string;
  url: string;
  icon: string;
}

const socials: Social[] = [
  {
    name: 'Github',
    url: import.meta.env.VITE_GITHUB_URL,
    icon: GithubIcon,
  },
];
</script>

<template>
  <header class="w-full flex items-center justify-between py-4 px-50">
    <img src="/favicon.ico" alt="winpods" width="50px" />
    <WPNav />
    <div class="flex gap-1 items-center justify-end w-1/3">
      <WPButton variant="secondary">Download</WPButton>
      <a
        v-for="social in socials"
        :key="social.name"
        :href="social.url"
        target="_blank"
        rel="noopener noreferrer"
      >
        <img :src="social.icon" :alt="social.name" width="24px" />
      </a>
    </div>
  </header>
  <RouterView />
</template>
