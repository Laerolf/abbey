export default function (scope?: string) {
    const {t} = useI18n()

    function translate(key: string, ...context: unknown[]): string {
        return t([scope, key].filter(keyPart => keyPart).join("."), context)
    }

    return {
        translate
    }
}
