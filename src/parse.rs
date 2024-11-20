use crate::language::LanguageNodeIdent;
use std::fs::read_to_string;
use swc_common::BytePos;
use swc_ecma_ast::*;
use swc_ecma_parser::{Parser, StringInput, Syntax, TsSyntax};
use swc_ecma_visit::VisitWith;

pub struct LanguageParse {
    path: String,
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
}
