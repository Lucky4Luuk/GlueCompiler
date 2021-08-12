use super::*;
use lexer::TokenKind;

pub(super) fn func(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at(TokenKind::KeywordFunc) {
        func_decl(p);
        if p.at(TokenKind::LBrace) {
            Some(code_block::code_block_parse(p))
        } else {
            p.error();
            None
        }
    } else {
        code_block::code_block(p)
    }
}

fn func_decl(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::KeywordFunc));
    let m = p.start();
    p.bump();

    p.expect(TokenKind::Identifier);

    func_decl_args(p);

    m.complete(p, SyntaxKind::FunctionDeclaration)
}

fn func_decl_args(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at(TokenKind::LParen) {
        let args = p.start();
        p.bump();
        'search: loop {
            if p.at(TokenKind::RParen) {
                p.bump();
                break 'search;
            }

            if p.at_end() {
                p.error();
                break 'search;
            }

            //TODO: Gather arguments
        }
        Some(args.complete(p, SyntaxKind::FunctionDeclarationArgs))
    } else {
        p.error();
        None
    }
}
