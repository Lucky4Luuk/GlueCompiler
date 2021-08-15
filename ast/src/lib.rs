use syntax::{SyntaxNode, SyntaxToken, SyntaxElement, SyntaxKind};

pub mod variable;
pub mod expr;
pub mod stmt;
pub mod code_block;
pub mod func;

pub use variable::*;
pub use expr::*;
pub use stmt::*;
pub use code_block::*;
pub use func::*;

#[derive(Debug)]
pub struct Root(SyntaxNode);

impl Root {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == SyntaxKind::Root {
            Some(Self(node))
        } else {
            None
        }
    }

    pub fn stmts(&self) -> impl Iterator<Item = Stmt> {
        self.0.children().filter_map(Stmt::cast)
    }

    pub fn funcs(&self) -> impl Iterator<Item = Func> {
        self.0.children().filter_map(Func::cast)
    }
}
