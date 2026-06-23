<script setup lang="ts">
import useNotifications from '@/composables/useNotifications'
import type { ANotification } from '@/composables/useNotifications'
import { computed } from 'vue'

const props = defineProps<{ notification: ANotification }>()

const emit = defineEmits<{
  (event: 'close', id: ANotification['id']): void
}>()

const { remove } = useNotifications()

let timeoutId: number | undefined = undefined

const classes = computed(() => ({
  success: props.notification.variant === 'success',
  warning: props.notification.variant === 'warning',
  error: props.notification.variant === 'error',
}))

function handleClose(): void {
  if (timeoutId) {
    clearTimeout(timeoutId)
  }

  emit('close', props.notification.id)
}

if (props.notification.lifespan > 0) {
  timeoutId = setTimeout(() => remove(props.notification.id), props.notification.lifespan)
}
</script>

<template>
  <a-grid class="a-notification" :class="classes">
    <p>{{ notification.content }}</p>

    <a-button @click="handleClose">X</a-button>
  </a-grid>
</template>

<style scoped>
.a-notification {
  align-items: center;
  padding: var(--space-1);
  background-color: green;
  border: var(--border-width-1) solid var(--color-border);
  border-radius: var(--radius-2);

  .a-button {
    border: none;
  }

  &.success {
    color: var(--color-success);
    border-color: var(--color-border-success);
    background-color: var(--color-bg-success);
  }

  &.warning {
    color: var(--color-warning);
    border-color: var(--color-border-warning);
    background-color: var(--color-bg-warning);
  }

  &.error {
    color: var(--color-error);
    border-color: var(--color-border-error);
    background-color: var(--color-bg-error);
  }
}
</style>
