<script setup lang="ts">
import { useRouter } from 'vue-router';

const routes = useRouter().getRoutes();
</script>

<template>
  <nav class="w-full text-md flex items-center justify-center">
    <router-link v-for="route in routes" :key="route.path" :to="route.path">
      {{ route.meta.title }}
    </router-link>
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
