<template>
  <button class="a-button">
    <slot />
  </button>
</template>

<style scoped>
.a-button {
  border: var(--border-width-1) solid var(--color-border);
  border-radius: var(--radius-1);
  padding: var(--space-1) var(--space-2);
  font-family: var(--font-ui);
  cursor: pointer;

  &:hover {
    background-color: var(--color-ui-hover);
  }
}
</style>
