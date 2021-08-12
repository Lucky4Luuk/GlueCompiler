mod expr;
mod stmt;
mod func;
mod code_block;

use crate::parser::marker::CompletedMarker;
use crate::Parser;
use syntax::SyntaxKind;

pub(crate) fn root(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    while !p.at_end() {
        func::func(p);
    }
    m.complete(p, SyntaxKind::Root)
}
