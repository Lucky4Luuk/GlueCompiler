use syntax::{SyntaxNode, SyntaxToken, SyntaxElement, SyntaxKind};

#[derive(Debug)]
pub struct CodeBlock(SyntaxNode);

impl CodeBlock {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == SyntaxKind::CodeBlock {
            Some(Self(node))
        } else {
            None
        }
    }
}
