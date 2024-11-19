mod cli;
mod parse;
mod read;
mod utils;

use crate::cli::Opts;
use crate::read::find_source_files;
use anyhow::Result;
use clap::Parser;
use rayon::prelude::*;
use std::path::Path;
use std::sync::Arc;
use thread_local::ThreadLocal;

fn main() -> Result<()> {
    let opts = Opts::parse();
    let mut language_parses = find_source_files(Path::new(&opts.entry_path))?;

    let tls = Arc::new(ThreadLocal::new());
    language_parses.par_iter_mut().for_each(|language_parse| {
        let tls = tls.clone();
        let thread_local_data = tls.get_or(|| Box::new(std::cell::RefCell::new(0)));
        language_parse.run().unwrap();
        *thread_local_data.borrow_mut() += language_parse.language.nodes.len();
    });

    let tls = Arc::try_unwrap(tls).unwrap();
    let total = tls.into_iter().fold(0, |x, y| {
        let value = *y.borrow();
        x + value
    });

    println!("language total: {}", total);

    Ok(())
}
