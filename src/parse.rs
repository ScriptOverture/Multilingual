use crate::utils;
use std::fs::read_to_string;
use swc_common::BytePos;
use swc_ecma_ast::*;
use swc_ecma_parser::{Parser, StringInput, Syntax, TsSyntax};
use swc_ecma_visit::{Visit, VisitWith};

#[derive(Default)]
pub struct LanguageNode {
    pub nodes: Vec<ObjectLit>,
}

#[allow(dead_code)]
impl LanguageNode {
    fn new() -> Self {
        Self::default()
    }
}

pub struct LanguageNodeIter<'a> {
    pub inner: &'a [ObjectLit],
    pub index: usize,
}

#[derive(Default)]
pub struct LanaguageKeyValue {
    pub key: String,
    pub value: String,
}

impl<'a> Iterator for LanguageNodeIter<'a> {
    type Item = LanaguageKeyValue;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.inner.len() {
            return None;
        }

        let obj_lit = &self.inner[self.index];
        let mut result = LanaguageKeyValue::default();

        for prop in &obj_lit.props {
            if let PropOrSpread::Prop(prop) = prop {
                if let Prop::KeyValue(key_value) = &**prop {
                    if let PropName::Ident(key) = &key_value.key {
                        match key.sym.to_string().as_str() {
                            "key" => result.key = utils::extract_str_value(&key_value.value),
                            "dm" => result.value = utils::extract_str_value(&key_value.value),
                            _ => {}
                        }
                    }
                }
            }
        }

        self.index += 1;
        Some(result)
    }
}

impl<'a> IntoIterator for &'a LanguageNode {
    type Item = LanaguageKeyValue;
    type IntoIter = LanguageNodeIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LanguageNodeIter {
            inner: &self.nodes,
            index: 0,
        }
    }
}

impl Visit for LanguageNode {
    fn visit_call_expr(&mut self, call_expr: &CallExpr) {
        if let Some((object_ident, property_ident)) = utils::match_visit_call_expr(call_expr) {
            if object_ident == "$i18n" && property_ident == "get" {
                for arg in &call_expr.args {
                    if let Expr::Object(obj_lit) = &*arg.expr {
                        self.nodes.push(obj_lit.clone());
                    }
                }
            }
        }

        // 继续递归访问子节点
        call_expr.visit_children_with(self);
    }
}

pub struct LanguageParse {
    path: String,
    pub language: LanguageNode,
}

impl LanguageParse {
    pub fn new(path: String, language: LanguageNode) -> Self {
        Self { path, language }
    }

    pub fn run(&mut self) -> anyhow::Result<Module> {
        let module = self.get_module()?;
        module.visit_with(&mut self.language);

        // for node in &self.language {
        //     println!("key- {:?} value - {}", node.key, node.value);
        // }
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
            .map_err(|err| anyhow::anyhow!("{:?}", err))
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
            Default::default(),
        );

        language_parse.run().unwrap();
        assert_eq!(language_parse.language.nodes.len(), 2);
    }

    // 验证函数式 语言调用匹配内容是否对应
    #[test]
    fn match_target_calls_verify_content() {
        let mut language_parse = LanguageParse::new(
            String::from("./example/useLanguage.tsx"),
            Default::default(),
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
