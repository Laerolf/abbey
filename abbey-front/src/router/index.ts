import { createRouter, createWebHistory } from 'vue-router'

import { authGuard, catalogGuard, localeGuard } from '@/middleware/guards'

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
        component: () => import('@/pages/authentication/LoginPage.vue'),
      },
      {
        name: 'Register',
        path: 'register',
        component: () => import('@/pages/authentication/RegisterPage.vue'),
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
      requiresCatalog: true,
    },
    children: [
      {
        path: '',
        redirect: { name: 'Surroundings' },
      },
      {
        name: 'Monastery',
        path: 'monastery',
        component: () => import('@/pages/game/MonasteryPage.vue'),
        meta: {
          secure: true,
          requiresCatalog: true,
        },
      },
      {
        name: 'Surroundings',
        path: 'surroundings',
        component: () => import('@/pages/game/SurroundingsPage.vue'),
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
router.beforeEach(catalogGuard)
router.beforeEach(authGuard)

export default router
