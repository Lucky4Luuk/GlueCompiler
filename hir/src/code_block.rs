use crate::Stmt;

#[derive(Debug)]
pub struct CodeBlock {
    pub stmts: Vec<Stmt>,
}

impl CodeBlock {
    pub fn lower(ast: ast::CodeBlock) -> Self {
        let stmts = ast.stmts().filter_map(Stmt::lower).collect::<Vec<_>>();

        Self {
            stmts: stmts,
        }
    }
}
