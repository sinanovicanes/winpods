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
  <header>
    <WPNav />
  </header>
  <RouterView />
</template>
