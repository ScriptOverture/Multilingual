import useLanguage from "./useLanguage";
import language from "./language";
import { $i18n } from "./utils";

function App() {
    const lan = useLanguage(language);

    return (
        <div>
            <p>{ lan.input }</p>
            <p>{ $i18n.get(language.input) }</p>
            <p>{ $i18n.get({
                key: "xxxx",
                dm: "test"
            }) }</p>
        </div>
    );
}


export default App;