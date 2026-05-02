import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { createHead } from '@unhead/vue/client'

import App from '@/App.vue'

import router from '@/router'
import i18n from '@/plugins/i18n'
import errorHandling from '@/plugins/errorHandling'

import '@/assets/styling/main.css'


const head = createHead({ init: [{ titleTemplate: "%s | Abbey" }] })

const app = createApp(App)

app.use(createPinia())
app.use(router)
app.use(i18n)
app.use(errorHandling)
app.use(head)

app.mount('#app')
