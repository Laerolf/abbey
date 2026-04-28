<script setup lang="ts">
import { computed } from 'vue'
import { useForm } from 'vee-validate'
import { toTypedSchema } from '@vee-validate/valibot'
import * as v from 'valibot'

import useTranslations from '@/composables/useLocale'

const { translate } = useTranslations("pages.userRegistration")

const validationSchema = computed(() => toTypedSchema(v.pipe(
  v.object({
    email: v.pipe(v.optional(v.string(), ''), v.string(), v.nonEmpty(translate("form.fields.email.validations.required")), v.email(translate("form.fields.email.validations.email"))),
    password: v.pipe(v.optional(v.string(), ''), v.string(), v.nonEmpty(translate("form.fields.password.validations.required"))),
    confirmPassword: v.pipe(v.optional(v.string(), ''), v.string(), v.nonEmpty(translate("form.fields.confirmPassword.validations.required")))
  }),
  v.forward(v.partialCheck([['password'], ['confirmPassword']], input => input.password === input.confirmPassword, translate("form.fields.confirmPassword.validations.match")), ['confirmPassword'])
)))

const { handleSubmit, meta } = useForm({
  validationSchema
})

const onSubmit = handleSubmit((values) => console.log(values))
</script>

<template>
  <a-card>
    <template #header>
      <h2>{{ translate("title") }}</h2>
    </template>

    <a-form @submit.prevent="onSubmit">
      <a-text-field name="email">
        {{ translate("form.fields.email.label") }}
      </a-text-field>

      <a-text-field secret name="password">
        {{ translate("form.fields.password.label") }}
      </a-text-field>

      <a-text-field secret name="confirmPassword">
        {{ translate("form.fields.confirmPassword.label") }}
      </a-text-field>


      <template #actions>
        <a-button :disabled="meta.dirty && !meta.valid" type="submit">{{ translate("form.actions.submit") }}</a-button>
      </template>
    </a-form>
  </a-card>
</template>
