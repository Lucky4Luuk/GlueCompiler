use super::code_block::code_block;
use super::*;
use lexer::TokenKind;

enum UnaryOp {
    Neg,
}

impl UnaryOp {
    fn binding_power(&self) -> ((), u8) {
        match self {
            Self::Neg => ((), 5),
        }
    }
}

enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinaryOp {
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Add | Self::Sub => (1, 2),
            Self::Mul | Self::Div => (3, 4),
        }
    }
}

pub(super) fn expr(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at(TokenKind::LBrace) {
        code_block(p)
    } else {
        expr_binding_power(p, 0)
    }
}

fn expr_binding_power(p: &mut Parser, minimum_binding_power: u8) -> Option<CompletedMarker> {
    let mut lhs = lhs(p)?;

    loop {
        let op = if p.at(TokenKind::Plus) {
            BinaryOp::Add
        } else if p.at(TokenKind::Minus) {
            BinaryOp::Sub
        } else if p.at(TokenKind::Asterix) {
            BinaryOp::Mul
        } else if p.at(TokenKind::Slash) {
            BinaryOp::Div
        } else {
            //We are not at an operator. We don't know what to do next, so we return and let the caller decide
            break;
        };

        let (left_binding_power, right_binding_power) = op.binding_power();

        if left_binding_power < minimum_binding_power {
            break;
        }

        // Eat the operator’s token.
        p.bump();

        let m = lhs.precede(p);
        let parsed_rhs = expr_binding_power(p, right_binding_power).is_some();
        lhs = m.complete(p, SyntaxKind::InfixExpr);

        if !parsed_rhs {
            break;
        }
    }

    Some(lhs)
}

fn lhs(p: &mut Parser) -> Option<CompletedMarker> {
    let cm = if p.at(TokenKind::Literal) {
        literal(p)
    } else if p.at(TokenKind::Identifier) {
        identifier(p)
    }else if p.at(TokenKind::Minus) {
        prefix_expr(p)
    } else if p.at(TokenKind::LParen) {
        paren_expr(p)
    } else {
        p.error();
        return None;
    };

    Some(cm)
}

fn literal(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::Literal));
    let m = p.start();
    p.bump();
    m.complete(p, SyntaxKind::Literal)
}

fn identifier(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::Identifier));
    let mut kind = SyntaxKind::VariableRef;
    let m = p.start();
    p.bump();
    if p.at(TokenKind::LParen) {
        //Function call
        kind = SyntaxKind::FunctionCall;
        p.bump();
        let args = p.start();

        'gather: loop {
            if p.at(TokenKind::RParen) {
                args.complete(p, SyntaxKind::FunctionCallArgs);
                p.bump();
                break 'gather;
            }

            if p.at(TokenKind::Identifier) {
                let tmp = p.start();
                p.bump();
                tmp.complete(p, SyntaxKind::Identifier);
            }

            if p.at(TokenKind::Literal) {
                let tmp = p.start();
                p.bump();
                tmp.complete(p, SyntaxKind::Literal);
            }
        }
    }
    m.complete(p, kind)
}

fn prefix_expr(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::Minus));
    let m = p.start();

    let op = UnaryOp::Neg;
    let ((), right_binding_power) = op.binding_power();

    // Eat the operator’s token.
    p.bump();

    expr_binding_power(p, right_binding_power);

    m.complete(p, SyntaxKind::PrefixExpr)
}

fn paren_expr(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::LParen));

    let m = p.start();
    p.bump();
    expr_binding_power(p, 0);
    p.expect(TokenKind::RParen);

    m.complete(p, SyntaxKind::ParenExpr)
}
