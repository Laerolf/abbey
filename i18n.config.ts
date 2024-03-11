import localeMessages from "@/locales"

// https://vue-i18n.intlify.dev/api/composition.html#composeroptions
export default defineI18nConfig(() => ({
    legacy: false,
    locale: "en",
    fallbackLocale: "en",
    messages: localeMessages
}))
