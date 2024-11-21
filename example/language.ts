import baseLanguage from "./baseLanguage"
import {$i18n} from "./utils";
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




function useLanguage2() {

    return {
        input: $i18n.get({
            key: "l.k.input",
            dm: "输入"
        }),
        age: $i18n.get({
            key: "l.k.age",
            dm: "年龄"
        })
    }
}