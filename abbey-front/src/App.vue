<script setup lang="ts">
import { computed } from 'vue'
import { useHead } from '@unhead/vue'

import { useI18n } from 'vue-i18n'

const { te, t } = useI18n()

useHead({
  titleTemplate: computed(() => {
    if (!te('app.title')) return '%s'
    return `%s | ${t('app.title')}`
  })
})
</script>

<template>
  <main>
    <router-view />
  </main>
</template>
