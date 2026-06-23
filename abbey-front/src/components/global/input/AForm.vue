<template>
  <form class="a-form">
    <a-grid rows>
      <a-grid rows>
        <slot />
      </a-grid>

      <a-grid class="actions">
        <slot name="actions" />
      </a-grid>
    </a-grid>
  </form>
</template>

<style scoped>
.a-form {
  .actions {
    margin-top: var(--space-4);
    grid-template-columns: max-content;
  }
}
</style>
