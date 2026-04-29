<script setup lang="ts">
import { computed } from 'vue'
import { useField } from 'vee-validate'

const props = defineProps<{
  name: string
  options: Record<string, string>
}>()

defineModel<string>()

const { value, errorMessage, handleBlur, meta } = useField(() => props.name, undefined, {
  syncVModel: true,
})

const classes = computed(() => ({
  error: meta.dirty && !meta.valid,
}))
</script>

<template>
  <a-grid rows class="a-select-field" :class="classes">
    <a-grid rows class="content">
      <label :for="name">
        <slot />
      </label>

      <select :id="name" :name="name" v-model="value" @blur="handleBlur">
        <option v-for="([key, label], index) in Object.entries(options)" :key="`${index}-${key}`" :value="key">
          {{ label }}
        </option>
      </select>
    </a-grid>

    <p class="error-message" v-show="errorMessage">{{ errorMessage }}</p>
  </a-grid>
</template>

<style scoped>
.a-select-field {
  gap: var(--space-0);

  .content {
    gap: var(--space-0);

    label {
      font-family: var(--font-ui);
    }

    select {
      border: var(--border-width-1) solid var(--color-border);
      border-radius: var(--radius-1);
    }
  }

  .error-message {
    color: var(--color-error);
  }

  &.error {
    label {
      color: var(--color-error);
    }

    select {
      border-color: var(--color-error);
    }
  }
}
</style>
