/// <reference types="vite/client" />
import 'vue-router'

export { }

declare module 'vue-router' {
  interface RouteMeta {
    /**
     * Does this route require authentication?
     */
    secure?: boolean
    /**
     * Does this route require the Catalog?
     */
    requiresCatalog?: boolean
  }
}
