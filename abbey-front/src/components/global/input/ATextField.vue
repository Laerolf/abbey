<script setup lang="ts">
import { computed } from 'vue'
import { useField } from 'vee-validate'

const props = defineProps<{
  name: string
  secret?: boolean
}>()

defineModel<string>()

const { value, errorMessage, handleBlur } = useField(() => props.name, undefined, {
  syncVModel: true,
})

const type = computed(() => (props.secret ? 'password' : 'text'))

const classes = computed(() => ({
  error: !!errorMessage.value,
}))
</script>

<template>
  <a-grid rows class="a-text-field" :class="classes">
    <a-grid rows class="content">
      <label :for="name">
        <slot />
      </label>

      <input :id="name" :name="name" v-model="value" @blur="handleBlur" :type="type" />
    </a-grid>

    <p class="error-message" v-show="errorMessage">{{ errorMessage }}</p>
  </a-grid>
</template>

<style scoped>
.a-text-field {
  gap: var(--space-0);

  .content {
    gap: var(--space-0);

    label {
      font-family: var(--font-ui);
    }

    input {
      border: var(--border-width-1) solid var(--color-border);
      border-radius: var(--radius-1);
    }
  }

  .error-message {
    color: var(--color-error);
  }

  &.error {
    input {
      border-color: var(--color-error);
    }
  }
}
</style>
