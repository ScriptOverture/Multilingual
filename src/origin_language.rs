use serde_json::Value;
use std::collections::{HashMap, HashSet};

// 获取远端多语言配置
pub async fn request_orgin_language() -> anyhow::Result<HashSet<String>> {
    let orgin_request_path =
        "https://bailingual.marmot-cloud.com/text/ics-mada-pc/latest/ics-mada-pc.js";
    let res = reqwest::get(orgin_request_path).await?.text().await?;

    let object_str = res.replace(r#"window["ics-mada-pc"]="#, "");

    let parsed: HashMap<String, Value> = serde_json::from_str(&object_str)?;
    // 获取远端不同语言配置不同key
    let language_set: HashSet<String> = parsed
        .values()
        .filter_map(|v| v.as_object())
        .flat_map(|v| v.keys().cloned())
        .collect();

    Ok(language_set)
}
