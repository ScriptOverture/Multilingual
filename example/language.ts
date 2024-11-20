import baseLanguage from "./baseLanguage"

const base = {
    key: "base",
    dm: "base dmmmm"
}

const data = {
    d: {
        a: {
            key: "d.a",
            dm: "aadasdasd"
        }
    }
}

export default {
    ...base,
    ...baseLanguage,
    input: baseLanguage.input,
    xxx: {
        key: "lll.k.xxx",
        dm: "xxx"
    },
    jjj: {
        key: "asdasd",
        dm: "jjj"
    }
}