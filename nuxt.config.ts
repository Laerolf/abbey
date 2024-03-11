// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    devtools: {enabled: true},

    srcDir: "src/",

    modules: [
        // https://i18n.nuxtjs.org/docs/getting-started
        "@nuxtjs/i18n",
        //https://nuxt.com/docs/getting-started/testing#unit-testing
        "@nuxt/test-utils/module"
    ],

    // https://i18n.nuxtjs.org/docs/getting-started/usage
    i18n: {
        vueI18n: "./i18n.config.ts",
        strategy: "no_prefix"
    },

    components: [
        {
            path: "@/components/global",
            pathPrefix: false
        }
    ],

    router: {
        options: {
            linkActiveClass: "active",
            linkExactActiveClass: "active-exact"
        }
    },

    css: ["@/assets/styling/_main.scss"],

    typescript: {
        typeCheck: true,
        strict: true
    }
})
