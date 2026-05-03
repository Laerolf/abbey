<template>
  <a-grid rows class="a-card">
    <a-grid v-if="$slots.header" class="a-card-header">
      <slot name="header" />
    </a-grid>

    <a-grid rows class="a-card-body">
      <slot />
    </a-grid>

    <a-grid v-if="$slots.footer" class="a-card-footer">
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
}
</style>
