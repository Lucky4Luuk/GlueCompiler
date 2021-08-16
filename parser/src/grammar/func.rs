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
    func_return_args(p);
    code_block::code_block(p);

    m.complete(p, SyntaxKind::FunctionDeclaration)
}

fn func_decl_args(p: &mut Parser) {
    if p.at(TokenKind::LParen) {
        p.bump();
        let args = p.start();
        'gather: loop {
            if p.at(TokenKind::RParen) {
                args.complete(p, SyntaxKind::FunctionDeclarationArgs);
                p.bump();
                break 'gather;
            }

            if p.at_end() {
                p.error();
                break 'gather;
            }

            if p.at(TokenKind::Identifier) {
                p.bump();
                p.expect(TokenKind::Identifier);
            }
        }
    } else {
        p.error();
    }
}

fn func_return_args(p: &mut Parser) {
    if p.at(TokenKind::Colon) {
        p.bump();
        let ret_args = p.start();

        if p.at(TokenKind::LParen) {
            //Multiple returns
            p.bump();
            'gather: loop {
                if p.at(TokenKind::RParen) {
                    p.bump();
                    ret_args.complete(p, SyntaxKind::FunctionDeclarationReturnArgs);
                    break 'gather;
                }

                if p.at(TokenKind::Identifier) { //Type
                    p.bump();
                    p.expect(TokenKind::Identifier); //Name
                }

                if p.at(TokenKind::Comma) {
                    p.bump();
                    p.expect(TokenKind::Identifier); //Type
                    p.expect(TokenKind::Identifier); //Name
                }
            }
        } else if p.at(TokenKind::Identifier) {
            //Single return
            p.bump();
            ret_args.complete(p, SyntaxKind::FunctionDeclarationReturnArgs);
        } else {
            p.error();
        }
    }
}
