<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps({
  /** Determines the direction of the grid flow, by default `false`. */
  rows: {
    type: Boolean,
    default: false
  }
})

const classes = computed(() => {
  return {
    'columns': !props.rows,
    'rows': props.rows
  }
})
</script>

<template>
  <div class="a-grid" :class="classes">
    <slot />
  </div>
</template>

<style scoped>
.a-grid {
  display: grid;
  column-gap: var(--space-1);
  row-gap: var(--space-1);

  .columns {
    grid-auto-flow: column;
  }

  .rows {
    grid-auto-flow: row;
  }
}
</style>
