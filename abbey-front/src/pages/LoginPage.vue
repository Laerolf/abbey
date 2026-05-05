<script setup lang="ts">
import { computed } from 'vue'
import { useForm } from 'vee-validate'
import { toTypedSchema } from '@vee-validate/valibot'
import * as v from 'valibot'
import { useHead } from '@unhead/vue'
import { useRouter } from 'vue-router'

import { useAuthStore } from '@/stores/authStore'

import useTranslations from '@/composables/useLocale'
import useNotifications from '@/composables/useNotifications'

const { translateInScope } = useTranslations('pages.userLogin')

useHead({
  title: computed(() => translateInScope('title')),
})

const validationSchema = computed(() =>
  toTypedSchema(
    v.pipe(
      v.object({
        email: v.pipe(
          v.optional(v.string(), ''),
          v.string(),
          v.nonEmpty(translateInScope('form.fields.email.validations.required')),
          v.email(translateInScope('form.fields.email.validations.email')),
        ),
        password: v.pipe(
          v.optional(v.string(), ''),
          v.string(),
          v.nonEmpty(translateInScope('form.fields.password.validations.required')),
        ),
      }),
    ),
  ),
)

const { handleSubmit, meta } = useForm({
  validationSchema,
})

const { loginUser } = useAuthStore()
const { add } = useNotifications()
const { push } = useRouter()

const onSubmit = handleSubmit(async (values) => {
  await loginUser({
    email: values.email,
    password: values.password,
  })

  add({ content: translateInScope('feedback.success.login'), variant: 'success' })

  await push({ path: '/game' })
})
</script>

<template>
  <a-grid rows>
    <a-card>
      <template #header>
        <h2>{{ translateInScope('title') }}</h2>
      </template>

      <a-form @submit.prevent="onSubmit">
        <a-text-field name="email">
          {{ translateInScope('form.fields.email.label') }}
        </a-text-field>

        <a-text-field secret name="password">
          {{ translateInScope('form.fields.password.label') }}
        </a-text-field>

        <template #actions>
          <a-button :disabled="meta.dirty && !meta.valid" type="submit">{{
            translateInScope('form.actions.submit')
          }}</a-button>
        </template>
      </a-form>
    </a-card>

    <ul>
      <li>
        <router-link :to="{ name: 'Register' }">{{ translateInScope('links.register') }}</router-link>
      </li>
    </ul>
  </a-grid>
</template>
