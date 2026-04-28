<script setup lang="ts">
import { useForm } from 'vee-validate'
import { toTypedSchema } from '@vee-validate/valibot'
import * as v from 'valibot'

const { handleSubmit, meta } = useForm({
  validationSchema: toTypedSchema(v.pipe(
    v.object({
      email: v.pipe(v.optional(v.string(), ''), v.string(), v.nonEmpty("An email is required."), v.email("Please provide a valid email address.")),
      password: v.pipe(v.optional(v.string(), ''), v.string(), v.nonEmpty("A password is required.")),
      confirmPassword: v.pipe(v.optional(v.string(), ''), v.string())
    }),
    v.forward(v.partialCheck([['password'], ['confirmPassword']], input => input.password === input.confirmPassword, 'Please confirm your password correctly.'), ['confirmPassword'])
  ))
})

const onSubmit = handleSubmit((values) => console.log(values))
</script>

<template>
  <a-card>
    <template #header>
      <h1>Register</h1>
    </template>

    <a-form @submit.prevent="onSubmit">
      <a-text-field name="email">
        Email
      </a-text-field>

      <a-text-field secret name="password">
        Password
      </a-text-field>

      <a-text-field secret name="confirmPassword">
        Confirm password
      </a-text-field>


      <template #actions>
        <a-button :disabled="!meta.valid" type="submit">Register</a-button>
      </template>
    </a-form>
  </a-card>
</template>
