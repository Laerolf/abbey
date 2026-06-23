<script setup lang="ts">
import { computed } from 'vue'
import { storeToRefs } from 'pinia'
import { useHead } from '@unhead/vue'

import { useGameStore } from '@/stores/gameStore'

import useLocale from '@/composables/useLocale'

import ASource from '@/components/ASource.vue'

import type { SourceDto } from '@/api'

const gameStore = useGameStore()

const { translateInScope } = useLocale('pages.surroundings')

useHead({
  title: computed(() => translateInScope('title')),
})

const { surroundings } = storeToRefs(gameStore)

const sources = computed<SourceDto[]>(() => surroundings.value?.sources ?? [])
</script>

<template>
  <a-grid rows>
    <a-card>
      <template #header>
        <h2>{{ translateInScope('title') }}</h2>
      </template>

      <a-source
        v-for="source in sources"
        :key="`surroundings-sources-${source.id}`"
        :source="source"
      />
    </a-card>
  </a-grid>
</template>
