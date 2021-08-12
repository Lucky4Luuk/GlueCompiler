use super::*;
use lexer::TokenKind;

pub(super) fn code_block(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at(TokenKind::LBrace) {
        Some(code_block_parse(p))
    } else {
        stmt::stmt(p)
    }
}

pub(super) fn code_block_parse(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::LBrace));
    let m = p.start();
    p.bump();

    'search: loop {
        if p.at(TokenKind::RBrace) {
            p.bump();
            break 'search;
        }

        if p.at_end() {
            p.error();
            break 'search;
        }

        code_block(p);
    }

    m.complete(p, SyntaxKind::CodeBlock)
}
