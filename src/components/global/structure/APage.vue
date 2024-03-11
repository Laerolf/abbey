<script setup lang="ts">
type Props = {
  title?: string
}

withDefaults(defineProps<Props>(), {
    title: ""
})
</script>

<template>
  <a-grid
    class="a-page"
    rows
  >
    <slot name="title">
      <h2>{{ title }}</h2>
    </slot>

    <slot />
  </a-grid>
</template>

<style scoped>
.a-page {
  padding: 0.5rem;
}
</style>
