use super::Parser;
use crate::lexer::SyntaxKind;

pub(super) fn expr(p: &mut Parser) {
    match p.peek() {
        SyntaxKind::Number => literal_number(p),
        _ => unreachable!("Unknown expr.")
    }
}

fn literal_number(p: &mut Parser) {
    p.start_node(SyntaxKind::LitE);
    p.bump(SyntaxKind::Number);
    p.finish_node();
}
