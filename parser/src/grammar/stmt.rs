use super::*;
use lexer::TokenKind;

pub(super) fn stmt(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at(TokenKind::Identifier) {
        Some(variable_def(p))
    } else {
        expr::expr(p)
    }
}

fn variable_def(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::Identifier));
    let m = p.start();
    p.bump();

    p.expect(TokenKind::Identifier);
    p.expect(TokenKind::Equals);

    expr::expr(p);

    p.expect(TokenKind::Semicolon);

    m.complete(p, SyntaxKind::VariableDef)
}
