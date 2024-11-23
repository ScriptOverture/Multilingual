use crate::language::LanguageNodeIdent;
use std::ffi::OsStr;
use std::fs::read_to_string;
use std::path::Path;
use std::string::String;
use std::sync::OnceLock;
use swc_common::BytePos;
use swc_ecma_ast::*;
use swc_ecma_parser::{Parser, StringInput, Syntax, TsSyntax};
use swc_ecma_visit::VisitWith;

pub static PARSE_CONFIG: OnceLock<ParseConfig> = OnceLock::new();

#[derive(Debug, Default)]
pub struct ParseConfig {
    pub exclude_dirs: Vec<String>,
}

impl ParseConfig {
    pub fn should_exclude_dir(&self, target_path: &Path) -> bool {
        target_path.components().any(|component| {
            self.exclude_dirs
                .iter()
                .any(|dir| <String as AsRef<OsStr>>::as_ref(dir) == component.as_os_str())
        })
    }
}

pub fn init_global_config(parse_config: ParseConfig) {
    PARSE_CONFIG
        .set(parse_config)
        .expect("Global instance already initialized!");
}

pub struct LanguageParse {
    pub path: String,
    pub language: LanguageNodeIdent,
}

impl LanguageParse {
    pub fn new(path: String, language: LanguageNodeIdent) -> Self {
        Self { path, language }
    }

    pub fn run(&mut self) -> anyhow::Result<Module> {
        let module = self.get_module()?;
        match &mut self.language {
            LanguageNodeIdent::CallExpression(ref mut node) => {
                module.visit_with(node);
            }
            LanguageNodeIdent::ObjectExpression(ref mut node) => {
                module.visit_with(node);
            }
        }

        Ok(module)
    }

    fn get_module(&self) -> anyhow::Result<Module> {
        let path = self.path.clone();
        let content = read_to_string(&path)?;
        let ts_syntax = TsSyntax {
            tsx: true,
            ..Default::default()
        };

        let mut parser = Parser::new(
            Syntax::Typescript(ts_syntax),
            StringInput::new(&content, BytePos(0), BytePos(content.len() as u32)),
            None,
        );

        parser
            .parse_module()
            .map_err(|err| anyhow::anyhow!("path - {:?} error - {:?}", path, err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 验证函数式 语言调用匹配数量是否对应
    #[test]
    fn match_target_calls() {
        let mut language_parse = LanguageParse::new(
            String::from("./example/useLanguage.tsx"),
            LanguageNodeIdent::CallExpression(Default::default()),
        );

        language_parse.run().unwrap();
        assert_eq!(language_parse.language.into_iter().count(), 2);
    }

    // 验证函数式 语言调用匹配内容是否对应
    #[test]
    fn match_target_calls_verify_content() {
        let mut language_parse = LanguageParse::new(
            String::from("./example/useLanguage.tsx"),
            LanguageNodeIdent::CallExpression(Default::default()),
        );

        language_parse.run().unwrap();

        for node in &language_parse.language {
            match node.key.as_str() {
                "l.k.input" => {
                    assert_eq!(node.value, "输入");
                }
                "l.k.age" => {
                    assert_eq!(node.value, "年龄");
                }
                _ => {}
            }
        }
    }

    // 检验language.ts语言配置是否正确
    #[test]
    fn match_target_object() {
        let mut language_parse = LanguageParse::new(
            String::from("./example/language.ts"),
            LanguageNodeIdent::ObjectExpression(Default::default()),
        );

        language_parse.run().unwrap();

        assert_eq!(
            language_parse
                .language
                .into_iter()
                .map(|item| {
                    println!("{:?}", item);
                    item
                })
                .count(),
            6
        );
    }

    // 测试全局 exclude-dirs 配置
    #[tokio::test]
    async fn match_global_config() {
        use crate::read::find_source_files;
        use std::path::PathBuf;
        init_global_config(ParseConfig {
            exclude_dirs: vec!["testDirs".to_string()],
        });

        let language_parses = find_source_files(PathBuf::from("./example/testDirs"))
            .await
            .unwrap();

        assert_eq!(language_parses.len(), 0);
    }
}
