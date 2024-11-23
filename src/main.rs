mod cli;
mod language;
mod parse;
mod read;
mod utils;

use crate::cli::Opts;
use crate::language::LanaguageKeyValue;
use crate::read::find_source_files;
use anyhow::Result;
use clap::Parser;
use rayon::prelude::*;
use serde_json::{json, Map, Value};
use std::cell::RefCell;
use std::path::PathBuf;
use std::sync::Arc;
use thread_local::ThreadLocal;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();
    let mut language_parses = find_source_files(PathBuf::from(opts.entry_path)).await?;

    let tls = Arc::new(ThreadLocal::new());
    language_parses.par_iter_mut().for_each(|language_parse| {
        let tls = tls.clone();
        let thread_local_data = tls.get_or(|| RefCell::new(Vec::new()));
        if language_parse.run().is_ok() {
            let mut v = Vec::new();

            for lan in language_parse.language.into_iter() {
                println!("language: {:?} path {}", lan, language_parse.path);
                v.push(lan);
            }

            thread_local_data.borrow_mut().extend(v);
        }
    });

    let mut map = Map::new();
    let tls = Arc::try_unwrap(tls).unwrap();
    let total = tls.into_iter().fold(Vec::new(), |mut x, y| {
        let value = y.borrow();
        x.extend(value.clone());
        x
    });
    println!("total: {}", total.len());
    total.into_iter().for_each(|lan| {
        let key = lan.key;
        let value = lan.value;

        if !map.contains_key(&key) {
            map.insert(key, Value::String(value));
        }
    });

    let dynamic_json = Value::Object(map);

    // 将 Value 序列化为 JSON 字符串
    let json_string = serde_json::to_string_pretty(&dynamic_json).unwrap();
    println!("{}", json_string);
    // println!("language: {:?}", total);

    Ok(())
}
