<script setup lang="ts">
import { useRouter } from 'vue-router';

interface Props {
  withIcon?: boolean;
}

withDefaults(defineProps<Props>(), {
  withIcon: true,
});

const githubURL = import.meta.env.VITE_GITHUB_URL;
const routes = useRouter().getRoutes();
</script>

<template>
  <nav class="w-full text-md px-50 flex justify-between items-center">
    <img v-if="withIcon" src="/favicon.ico" alt="winpods" width="50px" />
    <div class="flex items-center justify-center">
      <router-link v-for="route in routes" :key="route.path" :to="route.path">
        {{ route.meta.title }}
      </router-link>
    </div>
    <div>
      <a :href="githubURL" target="_blank" rel="noopener noreferrer">
        <img src="@/assets/github-mark-white.svg" alt="winpods" width="24px" />
      </a>
    </div>
  </nav>
</template>

<style>
nav a.router-link-exact-active {
  color: var(--color-text);
}

nav a.router-link-exact-active:hover {
  background-color: transparent;
}

nav a {
  display: inline-block;
  padding: 0 1rem;
  border-left: 1px solid var(--color-border);
}

nav a:first-of-type {
  border: 0;
}
</style>
