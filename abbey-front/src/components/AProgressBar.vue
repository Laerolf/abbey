<script setup lang="ts">
import { computed } from 'vue'

import type { CSSProperties } from 'vue'

const props = defineProps<{ progress: number }>()

const formattedValue = computed<string>(() => `${props.progress.toFixed(2)}%`)

const computedFillStyle = computed<CSSProperties>(() => ({ transform: `scaleX(${props.progress / 100})` }))
</script>

<template>
  <a-grid class="a-progress-bar">
    <div class="fill" :style="computedFillStyle" />
    <span class="value">{{ formattedValue }}</span>
  </a-grid>
</template>

<style scoped>
.a-progress-bar {
  border: var(--border-width-1) solid var(--color-border);
  height: var(--space-8);
  align-items: center;
  border-radius: var(--radius-2);
  overflow: hidden;

  .fill,
  .value {
    grid-area: 1 / 1;
  }

  .fill {
    height: var(--space-full);
    background-color: var(--color-success);
    width: var(--space-full);
    transform: scaleX(0);
    transform-origin: left;
    transition: transform 1s linear;
    will-change: transform;
  }

  .value {
    z-index: 1;
    text-align: center;
  }
}
</style>
