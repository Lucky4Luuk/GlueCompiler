use syntax::{SyntaxNode, SyntaxToken, SyntaxElement, SyntaxKind};

use crate::expr::Expr;
use crate::variable::VariableDef;
use crate::code_block::CodeBlock;
use crate::return_stmt::Return;

#[derive(Debug)]
pub enum Stmt {
    VariableDef(VariableDef),
    Expr(Expr),
    CodeBlock(CodeBlock),
    Return(Return),
}

impl Stmt {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        let result = match node.kind() {
            SyntaxKind::VariableDef => Self::VariableDef(VariableDef(node)),
            SyntaxKind::CodeBlock => Self::CodeBlock(CodeBlock(node)),
            SyntaxKind::Return => Self::Return(Return(node)),
            _ => Self::Expr(Expr::cast(node)?),
        };

        Some(result)
    }
}
