use super::Parser;
use crate::lexer::SyntaxKind;

pub(super) fn pat(p: &mut Parser) {
    match p.peek() {
        SyntaxKind::Ident => var_pattern(p),
        _ => unreachable!("Unknown pattern."),
    }
}

fn var_pattern(p: &mut Parser) {
    p.start_node(SyntaxKind::VarP);
    p.bump(SyntaxKind::Ident);
    p.finish_node();
}
