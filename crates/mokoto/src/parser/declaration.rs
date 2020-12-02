use super::expression;
use super::pattern;
use super::Parser;
use crate::lexer::SyntaxKind;

pub(super) fn decl(p: &mut Parser) {
    match p.current() {
        SyntaxKind::LetKw => let_decl(p),
        _ => unreachable!("Unknown decl."),
    }
}

fn ident(p: &mut Parser) {
    if !p.eat(SyntaxKind::Ident) {
        p.error("Expected an ident")
    }
}

fn let_decl(p: &mut Parser) {
    p.start_node(SyntaxKind::LetD);
    p.bump(SyntaxKind::LetKw);
    pattern::pat(p);
    if !p.eat(SyntaxKind::Equals) {
        p.error("Expected an =");
    }
    expression::expr(p);
    p.finish_node();
}
