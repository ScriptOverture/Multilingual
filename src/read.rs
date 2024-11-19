use std::path::{Path, PathBuf};
use anyhow::Result;
use std::fs;
use crate::parse::{
    LanguageParse,
    LanguageNode
};


pub fn find_source_files(
    target_dir: &Path
) -> Result<Vec<LanguageParse>> {
    let files_all = read_file(target_dir)?;
    let suffixes = [".txt", ".css", ".map"];

    let files_all = files_all.iter()
    .map(|files| files.display())
    .filter(|files| suffixes.iter().any(|suffix| !format!("{:?}", files).ends_with(suffix)))
    .map(|files| LanguageParse::new(
        files.to_string(),
        LanguageNode::default()
    ))
    .collect::<Vec<_>>();

    Ok(files_all)
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