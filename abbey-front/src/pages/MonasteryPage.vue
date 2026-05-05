<script setup lang="ts">
import { computed } from 'vue'
import { storeToRefs } from 'pinia'
import { useHead } from '@unhead/vue'

import { useGameStore } from '@/stores/gameStore'

import useLocale from '@/composables/useLocale'

import AMonk from '@/components/AMonk.vue'

const gameStore = useGameStore()

const { translateInScope } = useLocale('pages.gameMonastery')

useHead({
  title: computed(() => translateInScope('title')),
})

const { monastery } = storeToRefs(gameStore)
</script>

<template>
  <a-grid rows>
    <a-card>
      <template #header>
        <h2>{{ translateInScope('title') }}</h2>
      </template>

      <h3>{{ translateInScope('monks') }}</h3>

      <a-grid id="monastery-monks">
        <a-monk v-for="monk in monastery?.monks" :key="`monk-${monk.id}`" :monk="monk" />
      </a-grid>
    </a-card>
  </a-grid>
</template>

<style scoped>
#monastery-monks {
  grid-auto-flow: initial;
  grid-template-columns: repeat(4, 1fr);
}
</style>
