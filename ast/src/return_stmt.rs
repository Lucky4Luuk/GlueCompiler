use crate::expr::Expr;

use syntax::{SyntaxNode, SyntaxToken, SyntaxKind};

#[derive(Debug)]
pub struct Return(pub SyntaxNode);

impl Return {
    pub fn value(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }
}
