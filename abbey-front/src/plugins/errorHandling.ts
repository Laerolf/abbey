import { i18n } from '@/plugins/i18n'

import useLogger from '@/composables/useLogger'
import useNotifications from '@/composables/useNotifications'

import { isAppError } from '@/utils/mapper'

import type { Plugin } from 'vue'

const LOG_SCOPE = '@/plugins/errorHandling.ts'

export default {
  install(app) {
    app.config.errorHandler = (error) => {
      const logger = useLogger(LOG_SCOPE)

      if (isAppError(error)) {
        const { add } = useNotifications()

        add({ content: i18n.global.t(error.code), variant: 'error' })
      } else {
        logger.error('Something went terribly wrong!', error)
      }
    }
  },
} as Plugin
