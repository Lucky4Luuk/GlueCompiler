use syntax::{SyntaxNode, SyntaxToken, SyntaxKind};

use crate::expr::Expr;

#[derive(Debug)]
pub struct VariableDef(pub SyntaxNode);

impl VariableDef {
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

    pub fn kind(&self) -> Option<SyntaxToken> {
        self.0.first_token() //The variable type is always the first token
    }

    pub fn name(&self) -> Option<SyntaxToken> {
        //The variable name is always the second token of type identifier (there might be whitespace)
        let idents = self.tokens_of_type(SyntaxKind::Identifier);
        if idents.len() > 1 {
            Some(idents[1].clone())
        } else {
            None
        }
    }

    pub fn value(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }
}
