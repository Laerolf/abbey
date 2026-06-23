import { i18n, loadLocaleMessages } from '@/plugins/i18n'

import { useAuthStore } from '@/stores/authStore'
import { useCatalogStore } from '@/stores/catalogStore'

import useLogger from '@/composables/useLogger'

import type { NavigationGuard } from 'vue-router'
import type { SupportedLocale } from '@/plugins/i18n'

/**
 * A navigation guard loading the i18n locales.
 */
export const localeGuard: NavigationGuard = async () => {
  await loadLocaleMessages(i18n.global.locale.value as SupportedLocale)
}

/**
 * A navigation guard loading the Catalog.
 */
export const catalogGuard: NavigationGuard = async (to) => {
  if (to.meta.requiresCatalog) {
    const catalogStore = useCatalogStore()
    await catalogStore.loadTheCatalog()
  }
}

/**
 * A navigation guard handling authentication.
 */
export const authGuard: NavigationGuard = async (to) => {
  const logger = useLogger('@/middleware/guards/authGuard')

  if (!to.meta.secure) {
    logger.debug('Carry on, no need for authentication', { path: to.fullPath })
    return
  }

  logger.debug('IDENTIFY! Authentication is required for where you are going', {
    path: to.fullPath,
  })

  const authStore = useAuthStore()

  if (!authStore.authenticated) {
    try {
      logger.debug("Let's see if you really are, who you say you are", { path: to.fullPath })
      await authStore.refreshUser()
    } catch {
      return { name: 'Login' }
    }
  }

  if (!authStore.authenticated) {
    logger.debug('Bye-o-nara!', { path: to.fullPath })
    return { name: 'Login' }
  }

  logger.debug('We are sorry for the trouble... Have fun! :)', { path: to.fullPath })
}
