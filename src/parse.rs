use std::fs::read_to_string;
use swc_common::BytePos;
use swc_ecma_parser::{Parser, StringInput, Syntax, TsSyntax};
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct LanguageNode {
    pub nodes: Vec<ObjectLit>,
}

impl LanguageNode {
    fn new() -> Self {
       Self::default()
    }
}


pub struct LanguageNodeIter<'a> {
    pub inner: &'a [ObjectLit],
    pub index: usize
}

struct LanaguageKeyValue {
    key: String,
    value: String
}

impl<'a> Iterator for LanguageNodeIter<'a> {
    type Item = &'a ObjectLit;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.inner.len() {
            let result = &self.inner[self.index];
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}


impl<'a> IntoIterator for &'a LanguageNode {

    type Item = &'a ObjectLit;
    type IntoIter = LanguageNodeIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LanguageNodeIter {
            inner: &self.nodes,
            index: 0,
        }
    }
}


impl Default for LanguageNode {
    fn default() -> Self {
        LanguageNode {
            nodes: Vec::new(),
        }
    }
}


impl Visit for LanguageNode {
    fn visit_call_expr(&mut self, call_expr: &CallExpr) {
        if let Some((object_ident, property_ident)) = match_visit_call_expr(call_expr) {
            if object_ident == "$i18n" && property_ident == "get" {

                for arg in &call_expr.args {
                    if let Expr::Object(obj_lit) = &*arg.expr {

                        self.nodes.push(obj_lit.clone());
                        // for prop in &obj_lit.props {
                        //     if let PropOrSpread::Prop(prop) = prop {
                        //         if let Prop::KeyValue(prop) = &**prop {
                        //             println!("Found key: {:?}", prop);  
                        //             // self.nodes.push(prop.clone());
                        //         }
                        //     }
                        // }
                    }
                }
            }
        }

        // 继续递归访问子节点
        call_expr.visit_children_with(self);
    }
}


fn match_visit_call_expr(call_expr: &CallExpr) -> Option<(&str, &str)> {
    if let Callee::Expr(callee_expr) = &call_expr.callee {
        if let Expr::Member(member_expr) = &**callee_expr {
            if let Expr::Ident(object_ident) = &*member_expr.obj {
                if let MemberProp::Ident(property_ident) = &member_expr.prop {

                    return Some((
                        object_ident.sym.as_ref(), 
                        property_ident.sym.as_ref()
                    ));
                }
            }
        }
    }
    
    None
}


pub struct LanguageParse {
    path: String,
    language: LanguageNode
}

impl LanguageParse {
    pub fn new(
        path: String,
        language: LanguageNode
    ) -> Self {
        Self {
            path,
            language
        }
    }

    pub fn run(&mut self) -> anyhow::Result<Module> {
        let module = self.get_module()?;
        module.visit_with(&mut self.language);

        for node in &self.language {
            println!("{:?}", node);
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
            StringInput::new(
                    &content, 
                    BytePos(0),
                    BytePos(content.len() as u32)
                ),
             None
        );

        parser.parse_module().map_err(|err| anyhow::anyhow!("{:?}", err))
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_target_calls() {
        let mut language_parse = LanguageParse::new(
            String::from("./example/useLanguage.tsx"),
            Default::default()
        );

        language_parse.run().unwrap();
        assert_eq!(language_parse.language.nodes.len(), 2);
    }
}