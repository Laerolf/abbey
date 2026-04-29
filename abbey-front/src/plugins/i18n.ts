import { nextTick } from 'vue'
import { createI18n } from 'vue-i18n'

import type { Plugin } from 'vue'

/**
 * The list of supported locales in the application.
 */
export const SUPPORTED_LOCALES = ['en', 'ja'] as const

/**
 * The type representing the supported locales, derived from the SUPPORTED_LOCALES array. It ensures that only valid locale strings can be used throughout the application.
 */
export type SupportedLocale = (typeof SUPPORTED_LOCALES)[number]

export const i18n = createI18n({
  legacy: false,
  locale: 'en',
  availableLocales: SUPPORTED_LOCALES,
})

/**
 * Loads the locale messages for the specified locale and sets it as the current locale messages in the i18n instance.
 * @param locale - The locale for which to load the messages.
 */
export async function loadLocaleMessages(locale: SupportedLocale): Promise<void> {
  try {
    if (!SUPPORTED_LOCALES.includes(locale)) {
      console.warn(
        `Locale ${locale} is not supported. Supported locales are: ${SUPPORTED_LOCALES.join(', ')}`,
      )
      return
    }

    const messages = await import(
      /* webpackChunkName: "locale-[request]" */ `@/locales/${locale}.json`
    )

    i18n.global.setLocaleMessage(locale, messages.default)

    return nextTick()
  } catch (error) {
    throw new Error(
      `Failed to load locale messages for ${locale}: ${error instanceof Error ? error.message : String(error)}`,
    )
  }
}

export default {
  install(app) {
    app.use(i18n)
  },
} as Plugin
