use crate::language::LanguageNodeIdent;
use crate::parse::LanguageParse;
use anyhow::Result;
use futures::{stream, Stream, StreamExt};
use std::path::PathBuf;
use tokio::fs;

pub async fn find_source_files(target_dir: PathBuf) -> Result<Vec<LanguageParse>> {
    let suffixes = [".ts", ".tsx", ".js", "jsx", ".vue"];
    let files_all = read_file_stream(target_dir)
        .await
        .filter_map(|file_item| async {
            file_item.ok().and_then(|file| {
                let path = file.display().to_string();
                if suffixes.iter().any(|suffix| path.ends_with(suffix)) {
                    let language = if path.contains("language.ts") {
                        LanguageNodeIdent::ObjectExpression(Default::default())
                    } else {
                        LanguageNodeIdent::CallExpression(Default::default())
                    };
                    Some(LanguageParse::new(path, language))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>()
        .await;

    Ok(files_all)
}

pub async fn read_file_stream(
    target_dir: PathBuf,
) -> impl Stream<Item = Result<PathBuf, std::io::Error>> {
    stream::unfold(vec![target_dir], |mut dir| async {
        if let Some(current_file) = dir.pop() {
            if current_file.is_dir() {
                match fs::read_dir(&current_file).await {
                    Ok(mut read_dir) => {
                        while let Ok(Some(entry)) = read_dir.next_entry().await {
                            let path = entry.path();
                            dir.push(path);
                        }
                    }
                    Err(err) => return Some((Err(err), dir)),
                }
            }
            return Some((Ok(current_file), dir));
        }
        None
    })
}
