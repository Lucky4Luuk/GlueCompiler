pub mod expr;
pub mod stmt;

// use expr::Expr;
use stmt::Stmt;

pub fn lower(ast: ast::Root) -> impl Iterator<Item = Stmt> {
    ast.stmts().filter_map(Stmt::lower)
}
