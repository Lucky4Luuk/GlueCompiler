use hir::stmt::Stmt;

pub fn build_stmt(hir_stmt: &Stmt) -> String {
    match hir_stmt {
        Stmt::VariableDef { name, value, type_string } => {
            let mut tmp = String::new();
            tmp.push_str(super::type_to_c_type(type_string));
            tmp.push_str(" ");
            tmp.push_str(&name);
            tmp.push_str(" = ");
            tmp.push_str(&super::build_expr(value));
            tmp.push_str(";");
            tmp
        },
        Stmt::Return(expr) => {
            format!("return {};", &super::build_expr(expr))
        },
        _ => unimplemented!(),
    }
}
