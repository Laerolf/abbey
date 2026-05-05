import { useI18n } from 'vue-i18n'

/**
 * A composable function for handling locale-specific translations.
 * @param namespace - An optional namespace to prefix translation keys, allowing for organized and modular translation management.
 */
export default function useLocale(namespace?: string) {
  const { t } = useI18n({})

  /**
   * Translates a given key using the i18n instance, within a specified namespace.
   * @param key - The key to be translated, prefixed with the provided namespace.
   * @param context - An optional object containing values for interpolation in the translation string.
   * @returns The translated string corresponding to the provided key and context.
   */
  function translateInScope(key: string, context?: Record<string, unknown>): string {
    return t(`${namespace ? `${namespace}.` : ''}${key}`, context || {})
  }



  return {
    translate: t,
    translateInScope,
  }
}
