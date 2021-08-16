use syntax::{SyntaxNode, SyntaxToken, SyntaxElement, SyntaxKind};

use crate::stmt::Stmt;

#[derive(Debug, Clone)]
pub struct CodeBlock(SyntaxNode);

impl CodeBlock {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == SyntaxKind::CodeBlock {
            Some(Self(node))
        } else {
            None
        }
    }

    pub fn stmts(&self) -> impl Iterator<Item = Stmt> {
        self.0.children().filter_map(Stmt::cast)
    }
}
