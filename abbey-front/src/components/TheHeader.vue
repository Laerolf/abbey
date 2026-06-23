<script setup lang="ts">
import LocaleSelector from '@/components/LocaleSelector.vue'

import useTranslations from '@/composables/useLocale'

const { translateInScope } = useTranslations('app')
</script>

<template>
  <header id="the-header">
    <a-grid class="content">
      <h1>{{ translateInScope('title') }}</h1>

      <a-grid class="actions">
        <locale-selector />
      </a-grid>
    </a-grid>
  </header>
</template>

<style scoped>
#the-header {
  padding: var(--space-1) var(--space-1);
  align-content: center;

  .content {
    justify-content: space-between;
  }

  .actions {
    grid-template-columns: min-content;
  }
}
</style>
