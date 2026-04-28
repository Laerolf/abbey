import { nextTick } from "vue";
import { createI18n } from "vue-i18n";

import type { Plugin } from "vue";

export const SUPPORTED_LOCALES = ["en", "ja"] as const;

export type SupportedLocale = typeof SUPPORTED_LOCALES[number];

const i18n = createI18n({
  legacy: false,
  locale: "en",
  availableLocales: SUPPORTED_LOCALES,
})

export async function loadLocaleMessages(locale: SupportedLocale): Promise<void> {
  const messages = await import(
    /* webpackChunkName: "locale-[request]" */ `@/locales/${locale}.json`
  )

  i18n.global.setLocaleMessage(locale, messages.default)

  return nextTick()
}

export default {
  install(app) {
    app.use(i18n);
  }
} as Plugin;
