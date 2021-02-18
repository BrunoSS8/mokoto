use super::Parser;
use super::literals::{STARTS_LIT, literal};
use crate::lexer::SyntaxKind::*;

pub(super) fn pattern(p: &mut Parser) {
    pat_plain(p)
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
        }, // TODO: Error
        _ => unreachable!(), // TODO: Error
    }
}
