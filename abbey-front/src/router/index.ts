
import { createRouter, createWebHistory } from 'vue-router'

import { i18n, loadLocaleMessages } from '@/plugins/i18n'

import type { RouteRecordRaw } from 'vue-router'
import type { SupportedLocale } from '@/plugins/i18n'

/**
 * Authentication-related routes.
 */
const authRoutes: readonly RouteRecordRaw[] = [
  {
    path: '/',
    component: () => import('@/layouts/DefaultLayout.vue'),
    children: [
      {
        path: "",
        redirect: { name: 'Login' }
      },
      {
        path: '/login',
        name: 'Login',
        component: () => import('@/pages/LoginPage.vue'),
      },
      {
        path: '/register',
        name: 'Register',
        component: () => import('@/pages/RegisterPage.vue'),
      }
    ],
  },
]

const routes: readonly RouteRecordRaw[] = [...authRoutes]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

router.beforeEach(async () => {
  await loadLocaleMessages(i18n.global.locale.value as SupportedLocale)
})

export default router
