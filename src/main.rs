mod cli;
mod language;
mod parse;
mod read;
mod utils;

use crate::cli::Opts;
use crate::language::LanaguageKeyValue;
use crate::read::{find_source_files, read_file_parse_json, get_git_changed_file_paths};
use anyhow::Result;
use clap::Parser;
use rayon::prelude::*;
use serde_json::{Map, Value};
use std::cell::RefCell;
use std::path::PathBuf;
use std::sync::Arc;
use thread_local::ThreadLocal;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    let project_path = &opts.entry_path;

    let mut language_parses = get_git_changed_file_paths(project_path);

    println!("change_hash_set: {:?}", language_parses.len());
    // return Ok(());

    // let entry_path = PathBuf::from(opts.entry_path);
    // let origin_hash_map = read_file_parse_json(&PathBuf::from(opts.origin_language_path)).await?;

    // return Ok(());
    // let mut language_parses = find_source_files(entry_path).await?;

    let tls = Arc::new(ThreadLocal::new());
    language_parses.par_iter_mut().for_each(|language_parse| {
        let tls = tls.clone();
        let thread_local_data = tls.get_or(|| RefCell::new(Vec::new()));
        if language_parse.run().is_ok() {
            let mut v = Vec::new();

            for lan in language_parse.language.into_iter() {
                // println!("language: {:?} path {}", lan, language_parse.path);
                v.push(lan);
            }

            thread_local_data.borrow_mut().extend(v);
        }
    });

    let tls = Arc::try_unwrap(tls).unwrap();
    let total_hash_map = tls.into_iter().flat_map(|item| item.borrow().clone()).fold(
        Map::new(),
        |mut hash_map, item| {
            let LanaguageKeyValue { key, value } = item;

            if !hash_map.contains_key(&key) {
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
