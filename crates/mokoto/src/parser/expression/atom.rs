use super::*;


pub(crate) const LITERAL_FIRST: TokenSet =
    TokenSet::new(&[TrueKw, FalseKw, NumberLit]);

pub(crate) fn literal(p: &mut Parser) -> bool {
    if !p.at_ts(LITERAL_FIRST) {
        return false
    }
    p.start_node(LiteralE);
    p.bump_any();
    p.finish_node();
    true
}
