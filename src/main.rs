mod cli;
mod read;
mod parse;
use std::path::Path;

use crate::cli::Opts;
use crate::read::find_source_files;
use clap::Parser;
use anyhow::Result;

fn main() -> Result<()> {
    let opts = Opts::parse();
    let _ = find_source_files(
        Path::new(&opts.entry_path)
    )?;

    Ok(())
}
