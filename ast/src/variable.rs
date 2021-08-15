use syntax::{SyntaxNode, SyntaxToken, SyntaxElement};

use crate::expr::Expr;

#[derive(Debug)]
pub struct VariableDef(SyntaxNode);

impl VariableDef {
    fn children(&self) -> Vec<SyntaxToken> {
        let children: Vec<_> = self.0.children_with_tokens().filter_map(SyntaxElement::into_token).collect();
        children
    }

    pub fn kind(&self) -> Option<SyntaxToken> {
        self.0.first_token()
    }

    //TODO: This can be done so much better
    pub fn name(&self) -> Option<SyntaxToken> {
        let children = self.children();
        if children.len() > 1 {
            Some(children[1].clone())
        } else {
            None
        }
    }

    pub fn value(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }
}
