<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'

import { SUPPORTED_LOCALES, loadLocaleMessages } from '@/plugins/i18n'
import useLocale from '@/composables/useLocale'

import type { SupportedLocale } from '@/plugins/i18n'

const { locale } = useI18n()
const { translate } = useLocale('shared.locales')

const selectedLocale = ref<SupportedLocale>(locale.value as SupportedLocale)

const localeOptions: Record<SupportedLocale, string> = SUPPORTED_LOCALES.reduce(
  (acc, locale) => {
    acc[locale] = translate(locale)
    return acc
  },
  {} as Record<SupportedLocale, string>,
)

watch(selectedLocale, async (newLocale) => {
  await loadLocaleMessages(newLocale as SupportedLocale)

  locale.value = newLocale
})
</script>

<template>
  <a-select-field v-model="selectedLocale" name="locale" :options="localeOptions" />
</template>
