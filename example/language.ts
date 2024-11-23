import baseLanguage from "./baseLanguage"
import {$i18n} from "./utils";
const base = {
    id: "base",
    dm: "base dmmmm"
}

const data = {
    d: {
        a: {
            id: "d.a",
            dm: "aadasdasd"
        }
    }
}

export default {
    ...base,
    ...baseLanguage,
    input: baseLanguage.input,
    xxx: {
        id: "lll.k.xxx",
        dm: "xxx"
    },
    jjj: {
        id: "asdasd",
        dm: "jjj"
    }
}




function useLanguage2() {

    return {
        input: $i18n.get({
            id: "l.k.input",
            dm: "输入"
        }),
        age: $i18n.get({
            id: "l.k.age",
            dm: "年龄"
        })
    }
}