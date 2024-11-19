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
