<script setup lang="ts">
defineOptions({ inheritAttrs: false })

const emit = defineEmits<{
  (event: "toggled", state: boolean): void,
}>()

const open = defineModel<boolean>()

function toggle(): void {
  open.value = !open.value
  emit("toggled", open.value)
}
</script>

<template>
  <slot name="trigger" v-bind="{ toggle }">
    <a-button @click="toggle">Open</a-button>
  </slot>

  <teleport to="body">
    <a-card v-bind="$attrs" class="a-modal" v-if="open">
      <template #header>
        <a-grid class="header">
          <slot name="header" />
          <a-button @click="toggle">X</a-button>
        </a-grid>
      </template>

      <slot />

      <template #footer>
        <slot name="actions" />
      </template>
    </a-card>
  </teleport>
</template>

<style scoped>
.a-modal {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  background-color: white;
  z-index: 1;

  .header {
    grid-template-columns: auto max-content;
  }
}
</style>
