<script setup lang="ts">
import type {StyleValue} from "vue"

type Props = {
  rows?: boolean
  rowGap?: string | null
  alignStart?: boolean
  justifyCenter?: boolean
}

const props = withDefaults(defineProps<Props>(), {
    rows: false,
    rowGap: null,
    alignStart: false,
    justifyCenter: false
})

const useColumns = computed(() => !props.rows)

const computedClasses = computed(() => ({
    "rows": !useColumns.value,
    "columns": useColumns.value,
    "align-start-items": props.alignStart,
    "justify-center-items": props.justifyCenter
}))

const computedStyling = computed<StyleValue>(() => ({
    "row-gap": props.rowGap
}) as StyleValue)
</script>

<template>
  <div
    class="a-grid"
    :class="computedClasses"
    :style="computedStyling"
  >
    <slot />
  </div>
</template>

<style scoped lang="scss">
.a-grid {
  display: grid;
  width: 100%;

  grid-auto-columns: 1fr;
  grid-auto-rows: min-content;

  align-items: baseline;
  justify-items: normal;

  &.columns {
    grid-auto-flow: column;
  }

  &.rows {
    grid-auto-flow: row;
  }

  &.align-start-items {
    align-items: start;
  }

  &.justify-center-items {
    justify-items: center;
  }
}
</style>
