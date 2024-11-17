### 多语言收集

## input
```javascript
import baseLanguage from '../xx';

export default {
  ...baseLanguage,
  "xx": {
    key: "xxxxxxx",
    dm: "xxxxx"
  },
  ...more
}

// or

Component
<div>
  <div>{ $i18n.get({key: "xxx", dm: "xxx"}) }</div>
  ... more
```


## output

```javasctip
const allLanguage = {
  [languageKey1]: configRecord,
  [languageKey2]: configRecord,
  ...more
}
```
