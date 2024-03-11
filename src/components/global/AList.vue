<script setup lang="ts" generic="T extends {key: unknown, label: string } & Record<string, unknown>">
type Props<T> = {
  items?: T[]
  vertical?: boolean
}

withDefaults(defineProps<Props<T>>(), {
    items: () => [],
    vertical: false
})
</script>

<template>
  <a-card>
    <a-grid :rows="vertical">
      <slot
        v-for="({key, label, ...otherProps}) in items"
        :key="`a-list-item${key}`"
        :item="{key,label, ...otherProps}"
      >
        <p>{{ label }}</p>
      </slot>
    </a-grid>
  </a-card>
</template>

<style scoped>

</style>
