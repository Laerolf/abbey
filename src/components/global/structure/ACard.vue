<script setup lang="ts">
type Props = {
  title?: string | null
}

withDefaults(defineProps<Props>(), {
    title: null
})
</script>

<template>
  <a-grid
    rows
    class="a-card"
  >
    <a-grid
      v-if="title || $slots.header"
      class="a-card-header"
    >
      <slot
        name="header"
      >
        <label>{{ title }}</label>
      </slot>
    </a-grid>

    <a-grid class="a-card-content">
      <slot />
    </a-grid>

    <a-grid
      v-if="$slots.footer"
      class="a-card-footer"
    >
      <slot
        name="footer"
      />
    </a-grid>
  </a-grid>
</template>

<style scoped lang="scss">
.a-card {
  border: solid black 0.1rem;

  &-header {
    padding: 0.25rem 0.5rem;
    border-bottom: solid black 0.1rem;
  }

  &-content {
    padding: 0.25rem 0.5rem;
  }

  &-footer {
    padding: 0.25rem 0.5rem;
    border-top: solid black 0.1rem;
  }
}
</style>
