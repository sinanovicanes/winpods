<script setup lang="ts">
import GithubIcon from '@/assets/icons/github-mark-white.svg';
import WPButton from '@/components/WPButton.vue';
import WPNav from '@/components/WPNav.vue';
import { RouterView } from 'vue-router';

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
  <header class="container grid grid-cols-3 items-center justify-between p-4">
    <img src="/favicon.ico" alt="winpods" width="50px" />
    <WPNav />
    <div class="flex justify-self-end gap-2 items-center justify-end">
      <a
        v-for="social in socials"
        :key="social.name"
        :href="social.url"
        target="_blank"
        rel="noopener noreferrer"
      >
        <img :src="social.icon" :alt="social.name" width="24px" />
      </a>
      <WPButton variant="secondary">Download</WPButton>
    </div>
  </header>
  <RouterView />
</template>
