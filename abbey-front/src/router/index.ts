import { createRouter, createWebHistory } from 'vue-router'

import type { RouteRecordRaw } from 'vue-router'

/**
 * Authentication-related routes.
 */
const authRoutes: readonly RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Index',
    component: () => import('@/pages/RegisterPage.vue')
  }
]

const routes: readonly RouteRecordRaw[] = [
  ...authRoutes,
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

export default router
