use super::*;
use lexer::TokenKind;

pub(super) fn stmt(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at(TokenKind::Identifier) {
        Some(variable(p))
    } else {
        expr::expr(p)
    }
}

fn variable(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::Identifier));
    let m = p.start();
    p.bump();

    let result;

    // p.expect(TokenKind::Identifier);
    if p.at(TokenKind::Identifier) {
        //We know now there is a 2nd identifier, so it should be in the format <ident> <ident> = <expr>;
        result = SyntaxKind::VariableDef;
        p.bump();
    } else {
        //There is no 2nd identifier, so it should be in the format <ident> = <expr>;
        result = SyntaxKind::VariableAssignment;
    }
    p.expect(TokenKind::Equals);

    expr::expr(p);

    p.expect(TokenKind::Semicolon);

    m.complete(p, result)
}
