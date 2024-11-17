use std::path::{Path, PathBuf};
use rayon::prelude::*;
use anyhow::Result;
use std::fs;
use crate::parse::{
    LanguageParse,
    LanguageNode
};


pub fn find_source_files(
    target_dir: &Path
) -> Result<()> {
    let files_all = read_file(target_dir)?;
    let suffixes = [".txt", ".css", ".map"];

    let mut files_all = files_all.iter()
    .map(|files| format!("{:?}", files.display()))
    .filter(|files| suffixes.iter().any(|suffix| files.ends_with(suffix)))
    .map(|files| LanguageParse::new(
        files.to_string(),
        LanguageNode::default()
    ))
    .collect::<Vec<_>>();

    files_all.par_iter_mut().for_each(|language_parse| {
        language_parse.run().unwrap();
    });

    Ok(())
}


pub fn read_file(file_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    if file_dir.is_dir() {
        for entry in fs::read_dir(file_dir)? {
            let entry = entry?;
            let path = entry.path();

            files.extend(read_file(&path)?);
        }
    } else {
        files.push(file_dir.to_path_buf());
    }

    Ok(files)
}