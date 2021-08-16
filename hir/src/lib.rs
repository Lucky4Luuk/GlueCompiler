pub mod expr;
pub mod stmt;
pub mod code_block;
pub mod func;

use stmt::Stmt;
use func::Func;

/// Root stores everything contained in the root.
/// This currently includes:
/// - function declarations
/// - statements
#[derive(Debug)]
pub struct Root {
    pub functions: Vec<Func>,
    pub statements: Vec<Stmt>,
}

pub fn lower(ast: ast::Root) -> Root {
    let stmts = ast.stmts().filter_map(Stmt::lower).collect::<Vec<_>>();
    let funcs = ast.funcs().filter_map(Func::lower).collect::<Vec<_>>();
    Root {
        functions: funcs,
        statements: stmts,
    }
}
