mod atom;
use super::*;

pub(super) fn expr(p: &mut Parser) {
    if atom::literal(p) {
        return;
    }
    unreachable!("Can only do literals so far.")
}
