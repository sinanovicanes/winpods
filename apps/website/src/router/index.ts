import { createRouter, createWebHistory } from 'vue-router';
import HomeView from '../views/HomeView.vue';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      meta: {
        title: 'Home',
      },
      component: HomeView,
    },
    {
      path: '/faq',
      name: 'faq',
      meta: {
        title: 'FAQ',
      },
      component: () => import('../views/FAQView.vue'),
    },
  ],
});

export default router;
