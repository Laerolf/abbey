import { useI18n } from "vue-i18n";

export default function useLocale(namespace?: string) {
  const { t } = useI18n({})

  function translate(key: string, context?: Record<string, unknown>): string {
    return t(`${namespace ? `${namespace}.` : ''}${key}`, context || {})
  }

  return {
    translate
  }
}
