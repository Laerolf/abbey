import { createRouter, createWebHistory } from 'vue-router'

import { authGuard, localeGuard } from '@/middleware/guards'

import type { RouteRecordRaw } from 'vue-router'

/**
 * Authentication-related routes.
 */
const authRoutes: readonly RouteRecordRaw[] = [
  {
    path: '/auth',
    component: () => import('@/layouts/DefaultLayout.vue'),
    children: [
      {
        path: '',
        redirect: { name: 'Login' },
      },
      {
        name: 'Login',
        path: 'login',
        component: () => import('@/pages/LoginPage.vue'),
      },
      {
        name: 'Register',
        path: 'register',
        component: () => import('@/pages/RegisterPage.vue'),
      },
    ],
  },
]

/**
 * Game-related routes.
 */
const gameRoutes: readonly RouteRecordRaw[] = [
  {
    path: '/game',
    component: () => import('@/layouts/GameLayout.vue'),
    meta: {
      secure: true,
    },
    children: [
      {
        path: '',
        redirect: { name: 'Monastery' },
      },
      {
        name: 'Monastery',
        path: 'monastery',
        component: () => import('@/pages/MonasteryPage.vue'),
        meta: {
          secure: true,
        },
      },
    ],
  },
]

const routes: readonly RouteRecordRaw[] = [
  {
    path: '',
    redirect: { name: 'Login' },
  },
  ...authRoutes,
  ...gameRoutes,
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

router.beforeEach(localeGuard)
router.beforeEach(authGuard)

export default router
