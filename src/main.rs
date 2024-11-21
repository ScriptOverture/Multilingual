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
            thread_local_data.borrow_mut().extend(
                language_parse
                    .language
                    .into_iter()
                    .collect::<Vec<LanaguageKeyValue>>(),
            );
        }
    });

    let tls = Arc::try_unwrap(tls).unwrap();
    let total = tls.into_iter().fold(Vec::new(), |mut x, y| {
        let value = y.borrow();
        x.extend(value.clone());
        x
    });

    println!("language: {:?}", total);
    println!("total: {}", total.len());

    Ok(())
}
