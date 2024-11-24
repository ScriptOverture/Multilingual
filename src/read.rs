use crate::language::LanguageNodeIdent;
use crate::parse::LanguageParse;
use anyhow::Result;
use futures::{stream, Stream, StreamExt};
use serde_json::Map;
use swc_ecma_parser::token;
use std::{collections::HashSet, path::PathBuf};
use tokio::fs;
use std::process::Command;
use std::env;

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



pub async fn read_file_parse_json(target_file_path: &PathBuf) -> anyhow::Result<Map<String, serde_json::Value>> {
    let file_content = fs::read_to_string(target_file_path).await?;
    let json_string = file_content.replace("module.exports =", "").replace("}", "}");
    println!("json_string:{}", json_string);
    let json = serde_json::from_str(&json_string)?;

    Ok(json)

}



pub fn get_git_changed_file_paths(project_path: &str) -> Vec<LanguageParse> {
    env::set_current_dir(project_path).expect("Failed to change directory");

    let author = "曹明睿"; 
    let start_date = "2024-10-01";

    // 执行 git log 命令以获取指定作者在特定日期之后的提交哈希值
    let output = Command::new("git")
        .arg("log")
        .arg("--author=".to_owned() + author)
        .arg("--after=".to_owned() + start_date)
        .arg("--pretty=format:%H")
        .output()
        .expect("Failed to execute git log command");

    let mut change_hash_set = HashSet::new();
    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);

        // 逐行获取每个提交的哈希值，并获取变更的文件路径
        for commit_hash in output_str.lines() {
            let diff_output = Command::new("git")
                .arg("diff-tree")
                .arg("--no-commit-id")
                .arg("--name-only")
                .arg("--diff-filter=AM")
                .arg("-r")
                .arg(commit_hash)
                .output()
                .expect("Failed to execute git diff-tree command");
            
            if diff_output.status.success() {
                let diff_output_str = String::from_utf8_lossy(&diff_output.stdout);
                for line in diff_output_str.lines() {
                    if !change_hash_set.contains(line) {
                        change_hash_set.insert(line.to_string());
                    }
                }
            } else {
                let error_str = String::from_utf8_lossy(&diff_output.stderr);
                eprintln!("Error: {}", error_str);
            }
        }
    } else {
        let error_str = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error: {}", error_str);
    }

    let suffixes = [".ts", ".tsx", ".js", "jsx", ".vue"];
    change_hash_set.into_iter().filter_map(|path| {
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
    }).collect::<Vec<_>>()
}