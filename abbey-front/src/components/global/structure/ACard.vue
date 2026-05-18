<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{ dense?: boolean }>()

const classes = computed(() => ({
  dense: props.dense
}))
</script>

<template>
  <a-grid rows class="a-card" :class="classes">
    <a-grid v-if="$slots.header" class="a-card-header">
      <slot name="header" />
    </a-grid>

    <a-grid rows class="a-card-body">
      <slot />
    </a-grid>

    <a-grid v-if="$slots.footer" class="footer">
      <slot name="footer" />
    </a-grid>
  </a-grid>
</template>

<style scoped>
.a-card {
  border: var(--border-width-2) solid var(--color-border);
  border-radius: var(--radius-1);

  * {
    padding: var(--space-2) var(--space-4);
  }

  .footer {
    grid-auto-columns: max-content;
  }

  .dense {
    * {
      padding: var(--space-0) var(--space-1)
    }
  }
}
</style>
