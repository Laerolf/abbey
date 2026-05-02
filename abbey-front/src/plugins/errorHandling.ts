import useLogger from "@/composables/useLogger";
import type { Plugin } from "vue";

const LOG_SCOPE = "@/plugins/errorHandling.ts"

export default {
  install(app) {
    app.config.errorHandler = (error) => {
      useLogger(LOG_SCOPE).error(error as string)
    }
  }
} as Plugin
