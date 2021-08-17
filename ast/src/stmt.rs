use syntax::{SyntaxNode, SyntaxToken, SyntaxElement, SyntaxKind};

use crate::expr::Expr;
use crate::variable::VariableDef;
use crate::code_block::CodeBlock;

#[derive(Debug)]
pub enum Stmt {
    VariableDef(VariableDef),
    Expr(Expr),
    CodeBlock(CodeBlock),
}

impl Stmt {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        let result = match node.kind() {
            SyntaxKind::VariableDef => Self::VariableDef(VariableDef(node)),
            SyntaxKind::CodeBlock => Self::CodeBlock(CodeBlock(node)),
            _ => Self::Expr(Expr::cast(node)?),
        };

        Some(result)
    }
}
