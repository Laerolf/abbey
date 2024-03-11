import "vue-router"

export {}

declare module "#app" {
    interface PageMeta {
        /**
         * Does this page require authentication?
         */
        $secured?: boolean
    }
}

declare module "vue-router" {
    interface RouteMeta {
        /**
         * Does this page require authentication?
         */
        $secured?: boolean
    }
}
