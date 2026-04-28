<script setup lang="ts">
import { useField } from 'vee-validate'
import { computed } from 'vue';

const props = defineProps<{
  name: string,
  secret?: boolean
}>()

const { value, errorMessage, handleBlur, meta } = useField(() => props.name, undefined, {
  syncVModel: true
})

const type = computed(() => props.secret ? 'password' : 'text')

const classes = computed(() => ({
  error: meta.dirty && !meta.valid
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

    <p class="error-message" v-if="errorMessage">{{ errorMessage }}</p>
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
    label {
      color: var(--color-error);
    }

    input {
      border-color: var(--color-error);
    }
  }
}
</style>
