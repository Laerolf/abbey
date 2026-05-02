<script setup lang="ts">
import { computed } from 'vue'
import { useForm } from 'vee-validate'
import { toTypedSchema } from '@vee-validate/valibot'
import * as v from 'valibot'
import { useHead } from '@unhead/vue'

import { useAuthStore } from '@/stores/authStore'

import useTranslations from '@/composables/useLocale'
import useNotifications from '@/composables/useNotifications'

const { translate } = useTranslations('pages.userLogin')

useHead({
  title: computed(() => translate("title"))
})

const validationSchema = computed(() =>
  toTypedSchema(
    v.pipe(
      v.object({
        email: v.pipe(
          v.optional(v.string(), ''),
          v.string(),
          v.nonEmpty(translate('form.fields.email.validations.required')),
          v.email(translate('form.fields.email.validations.email')),
        ),
        password: v.pipe(
          v.optional(v.string(), ''),
          v.string(),
          v.nonEmpty(translate('form.fields.password.validations.required')),
        ),
      })
    ),
  ),
)

const { handleSubmit, meta } = useForm({
  validationSchema,
})

const { loginUser } = useAuthStore()
const { add } = useNotifications()

const onSubmit = handleSubmit(async (values) => {
  await loginUser({
    email: values.email,
    password: values.password
  })

  add({ content: translate("feedback.success.login"), variant: 'success' })
})
</script>

<template>
  <a-grid rows>
    <a-card>
      <template #header>
        <h2>{{ translate('title') }}</h2>
      </template>

      <a-form @submit.prevent="onSubmit">
        <a-text-field name="email">
          {{ translate('form.fields.email.label') }}
        </a-text-field>

        <a-text-field secret name="password">
          {{ translate('form.fields.password.label') }}
        </a-text-field>

        <template #actions>
          <a-button :disabled="meta.dirty && !meta.valid" type="submit">{{
            translate('form.actions.submit')
            }}</a-button>
        </template>
      </a-form>
    </a-card>

    <ul>
      <li><router-link to="Register">{{ translate("links.register") }}</router-link></li>
    </ul>
  </a-grid>
</template>
