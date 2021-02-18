use super::literals::{literal, STARTS_LIT};
use super::Parser;
use crate::lexer::SyntaxKind::*;

pub(super) fn pattern(p: &mut Parser) {
    pat_plain(p)
}

fn paren_or_tuple_pattern(p: &mut Parser) {
    assert!(p.at(L_PAREN));
    let c = p.checkpoint();
    p.bump(L_PAREN);
    if p.eat(R_PAREN) {
        p.finish_at(c, TUPLE_PAT);
        return;
    }
    pattern(p);
    if p.eat(R_PAREN) {
        p.finish_at(c, PAREN_PAT);
        return;
    }
    while p.eat(COMMA) {
        pattern(p);
    }
    // TODO Error
    p.bump(R_PAREN);
    p.finish_at(c, TUPLE_PAT)
}

fn pat_plain(p: &mut Parser) {
    let c = p.checkpoint();
    match p.current() {
        UNDERSCORE => {
            p.bump(UNDERSCORE);
            p.finish_at(c, WILDCARD_PAT)
        }
        IDENT => {
            p.bump(IDENT);
            p.finish_at(c, VAR_PAT)
        }
        t if STARTS_LIT.contains(t) => {
            literal(p);
            p.finish_at(c, LITERAL_PAT)
        }
        L_PAREN => paren_or_tuple_pattern(p),
        // TODO: Error
        _ => unreachable!(), // TODO: Error
    }
}
