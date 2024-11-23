use swc_ecma_ast::*;

/// 提取字符串值的辅助函数
pub fn extract_str_value(expr: &Expr) -> String {
    if let Expr::Lit(Lit::Str(lit)) = expr {
        lit.value.to_string()
    } else {
        String::new()
    }
}

/// 匹配AST函数调用
pub fn match_visit_call_expr(call_expr: &CallExpr) -> Option<(&str, &str)> {
    if let Callee::Expr(callee_expr) = &call_expr.callee {
        if let Expr::Member(member_expr) = &**callee_expr {
            if let Expr::Ident(object_ident) = &*member_expr.obj {
                if let MemberProp::Ident(property_ident) = &member_expr.prop {
                    return Some((object_ident.sym.as_ref(), property_ident.sym.as_ref()));
                }
            }
        }
    }

    None
}

// 简单的 “$i18n.get 函数提取” 的包装
pub fn handle_i18n_get_call_expr(call_expr: &CallExpr, nodes: &mut Vec<ObjectLit>) {
    if let Some((object_ident, property_ident)) = match_visit_call_expr(call_expr) {
        if object_ident == "$i18n" && property_ident == "get" {
            for arg in &call_expr.args {
                if let Expr::Object(obj_lit) = &*arg.expr {
                    let language_nodes = dfs_object_expression_node(obj_lit, "id")
                        .into_iter()
                        .cloned()
                        .collect::<Vec<ObjectLit>>();
                    nodes.extend(language_nodes);
                }
            }
        }
    }
}

/// languzge.ts 配置文件 深度遍历查找
pub fn dfs_object_expression_node<'ast>(
    obj_lit: &'ast ObjectLit,
    key_ident: &str,
) -> Vec<&'ast ObjectLit> {
    let mut result = Vec::new();
    for prop in &obj_lit.props {
        if let PropOrSpread::Prop(boxed_prop) = prop {
            if let Prop::KeyValue(key_value_prop) = &**boxed_prop {
                if let PropName::Ident(ident) = &key_value_prop.key {
                    if ident.sym == key_ident {
                        result.push(obj_lit);
                        break;
                    }
                }
                // if let Expr::Object(obj_lit) = &*key_value_prop.value {
                //     result.extend(dfs_object_expression_node(obj_lit, key_ident));
                // }
            }
        }
    }
    result
}
