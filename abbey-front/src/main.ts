import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { createHead } from '@unhead/vue/client'

import App from '@/App.vue'

import router from '@/router'
import i18n from '@/plugins/i18n'
import errorHandling from '@/plugins/errorHandling'

import { useAuthStore } from '@/stores/authStore'

import '@/assets/styling/main.css'
import { client } from './api/client.gen'

/// TODO: Improve
client.setConfig({
  baseUrl: import.meta.env.VITE_API_URL,
  credentials: 'include',
})

client.interceptors.request.use((request) => {
  const authStore = useAuthStore()

  if (authStore.sessionToken) {
    request.headers.set('Authorization', `Bearer ${authStore.sessionToken}`)
  }

  return request
})

const head = createHead({ init: [{ titleTemplate: '%s | Abbey' }] })

const app = createApp(App)

app.use(createPinia())
app.use(router)
app.use(i18n)
app.use(errorHandling)
app.use(head)

app.mount('#app')
