use crate::utils;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

#[derive(Default)]
pub struct CallExpressionLanguageNode {
    pub nodes: Vec<ObjectLit>,
}

impl Visit for CallExpressionLanguageNode {
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

#[derive(Default)]
pub struct ObjectExpressionLanguageNode {
    nodes: Vec<ObjectLit>,
}

impl Visit for ObjectExpressionLanguageNode {
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

#[derive(Default, Debug, Clone)]
pub struct LanaguageKeyValue {
    pub key: String,
    pub value: String,
}

pub struct LanguageNodeIter<'a> {
    pub inner: &'a [ObjectLit],
    pub index: usize,
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

impl<'a> IntoIterator for &'a CallExpressionLanguageNode {
    type Item = LanaguageKeyValue;
    type IntoIter = LanguageNodeIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LanguageNodeIter {
            inner: &self.nodes,
            index: 0,
        }
    }
}

impl<'a> IntoIterator for &'a ObjectExpressionLanguageNode {
    type Item = LanaguageKeyValue;
    type IntoIter = LanguageNodeIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LanguageNodeIter {
            inner: &self.nodes,
            index: 0,
        }
    }
}

pub enum LanguageNodeIdent {
    CallExpression(CallExpressionLanguageNode),
    ObjectExpression(ObjectExpressionLanguageNode),
}

impl<'a> IntoIterator for &'a LanguageNodeIdent {
    type Item = LanaguageKeyValue;
    type IntoIter = LanguageNodeIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            LanguageNodeIdent::CallExpression(node) => LanguageNodeIter {
                inner: &node.nodes,
                index: 0,
            },
            LanguageNodeIdent::ObjectExpression(node) => LanguageNodeIter {
                inner: &node.nodes,
                index: 0,
            },
        }
    }
}