use crate::expr::Expr;

#[derive(Debug)]
pub enum Stmt {
    VariableDef { name: String, value: Expr },
    Expr(Expr),
}

impl Stmt {
    pub fn lower(ast: ast::Stmt) -> Option<Self> {
        let result = match ast {
            ast::Stmt::VariableDef(ast) => Self::VariableDef {
                name: ast.name()?.text().to_string(),
                value: Expr::lower(ast.value()),
            },
            ast::Stmt::Expr(ast) => Self::Expr(Expr::lower(Some(ast)))
        };

        Some(result)
    }
}
