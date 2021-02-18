use super::{Parser, TokenSet};
use crate::lexer::SyntaxKind::*;

// TODO: All literals
pub(crate) const STARTS_LIT: TokenSet = TokenSet::new(&[NULL_KW, TRUE_KW, FALSE_KW, NUMBER_LIT]);

pub fn literal(p: &mut Parser) {
    let c = p.checkpoint();
    if p.at_ts(STARTS_LIT) {
        p.bump_any();
        p.finish_at(c, LITERAL)
    } else {
        // TODO: Error
        unreachable!()
    }
}
