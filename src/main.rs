mod cli;
mod language;
mod origin_language;
mod parse;
mod read;
mod utils;

use crate::cli::Opts;
use crate::language::LanaguageKeyValue;
use crate::origin_language::request_orgin_language;
use crate::read::find_source_files;
use anyhow::Result;
use clap::Parser;
use parse::{init_global_config, ParseConfig};
use rayon::prelude::*;
use serde_json::{Map, Value};
use std::cell::RefCell;
use std::path::PathBuf;
use std::sync::Arc;
use thread_local::ThreadLocal;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    init_global_config(ParseConfig {
        exclude_dirs: opts.exclude_dirs.unwrap_or(vec![]),
    });

    let origin_language_handle = tokio::spawn(request_orgin_language());

    let mut language_parses = find_source_files(PathBuf::from(opts.entry_path)).await?;

    let tls = Arc::new(ThreadLocal::new());
    language_parses.par_iter_mut().for_each(|language_parse| {
        let tls = tls.clone();
        let thread_local_data = tls.get_or(|| RefCell::new(Vec::new()));
        if language_parse.run().is_ok() {
            thread_local_data.borrow_mut().extend(
                language_parse
                    .language
                    .into_iter()
                    .collect::<Vec<LanaguageKeyValue>>(),
            );
        }
    });

    let origin_language = origin_language_handle.await??;

    let tls = Arc::try_unwrap(tls).unwrap();
    let total_hash_map = tls.into_iter().flat_map(|item| item.borrow().clone()).fold(
        Map::new(),
        |mut hash_map, item| {
            let LanaguageKeyValue { key, value } = item;

            if !hash_map.contains_key(&key) && !origin_language.contains(&key) {
                hash_map.insert(key, Value::String(value));
            }
            hash_map
        },
    );

    println!("language total: {}", total_hash_map.len());
    let dynamic_json = Value::Object(total_hash_map);
    let json_string = serde_json::to_string_pretty(&dynamic_json).unwrap();
    println!("output: {}", json_string);

    Ok(())
}
