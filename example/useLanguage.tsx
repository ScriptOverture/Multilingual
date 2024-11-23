import {$i18n} from "./utils";


function useLanguage(config: any) {
    return Object.keys(config).reduce((obj, item) => {
        obj[item] = $i18n.get(config[item]);
        return obj
    }, {});
}

export default useLanguage;


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