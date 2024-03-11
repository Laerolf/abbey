<script setup lang="ts">
type Props = {
  rows?: boolean
}

const props = withDefaults(defineProps<Props>(), {
    rows: false
})

const computedClasses = computed(() => ({
    "rows": props.rows
}))
</script>

<template>
  <div
    class="a-grid"
    :class="computedClasses"
  >
    <slot />
  </div>
</template>

<style scoped lang="scss">
.a-grid {
  display: grid;
  grid-auto-flow: column;

  align-items: baseline;

  &.rows {
    grid-auto-flow: row;
  }
}
</style>
