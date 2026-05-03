<script setup lang="ts">
import { onBeforeMount } from 'vue'

import { useGameStore } from '@/stores/gameStore'

const { loadSessionGame } = useGameStore()

onBeforeMount(async () => {
  await loadSessionGame()
})
</script>

<template>
  <a-grid rows class="layout">
    <a-grid class="content">
      <router-view />
    </a-grid>
  </a-grid>
</template>

<style scoped>
.layout {
  row-gap: var(--space-4);

  .content {
    grid-template-columns: 50vw;
    justify-content: center;
  }
}
</style>
