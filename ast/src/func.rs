use syntax::{SyntaxNode, SyntaxToken, SyntaxElement, SyntaxKind};

use crate::code_block::CodeBlock;

#[derive(Debug)]
pub struct Func(SyntaxNode);

impl Func {
    fn children_of_type(&self, kind: SyntaxKind) -> Vec<SyntaxNode> {
        self.0.children().filter_map(|node| if node.kind() == SyntaxKind::Identifier { Some(node) } else { None }).collect::<Vec<_>>()
    }

    //TODO: This function is not very neat.
    fn tokens_of_type(&self, kind: SyntaxKind) -> Vec<SyntaxToken> {
        let mut children: Vec<SyntaxToken> = Vec::new();
        for element in self.0.children_with_tokens() {
            if element.kind() == kind {
                children.push(element.as_token().expect("Unreachable!").clone());
            }
        }
        children
    }

    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == SyntaxKind::FunctionDeclaration {
            Some(Self(node))
        } else {
            None
        }
    }

    pub fn name(&self) -> Option<SyntaxToken> {
        let idents = self.tokens_of_type(SyntaxKind::Identifier);
        let ident_count = idents.len();
        if ident_count > 1 {
            Some(idents[1].clone())
        } else if ident_count > 0 {
            Some(idents[0].clone())
        } else {
            None
        }
    }

    pub fn code_blocks(&self) -> impl Iterator<Item = CodeBlock> {
        self.0.children().filter_map(CodeBlock::cast)
    }
}
