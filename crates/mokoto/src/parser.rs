mod declaration;
mod literals;
mod expression;
mod patterns;
mod token_set;
mod types;

use crate::lexer::{Lexer, SyntaxKind, SyntaxKind::*, Token};
use crate::syntax::{MotokoLanguage, SyntaxNode};
use declaration::decl;
use rowan::{Checkpoint, GreenNode, GreenNodeBuilder, Language};
use token_set::TokenSet;

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    builder: GreenNodeBuilder<'static>,
    // TODO: Be smarter here
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut tkns = Lexer::lex(input);
        tkns.reverse();
        Self {
            tokens: tkns,
            builder: GreenNodeBuilder::new(),
            errors: vec![],
        }
    }

    pub fn parse(mut self) -> Parse {
        self.start_node(SyntaxKind::Root);

        decl(&mut self);

        self.finish_node();

        Parse {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }

    pub fn parse_typ(mut self) -> Parse {
        types::typ(&mut self);

        Parse {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }

    pub fn parse_pattern(mut self) -> Parse {
        patterns::pattern(&mut self);

        Parse {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(MotokoLanguage::kind_to_raw(kind));
    }

    fn start_node_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
        self.builder
            .start_node_at(checkpoint, MotokoLanguage::kind_to_raw(kind));
    }

    fn finish_node(&mut self) {
        self.builder.finish_node();
    }

    fn finish_at(&mut self, c: Checkpoint, kind: SyntaxKind) {
        self.start_node_at(c, kind);
        self.finish_node();
    }

    fn checkpoint(&self) -> Checkpoint {
        self.builder.checkpoint()
    }

    fn bump_any(&mut self) {
        let (leading, (kind, text), trailing) = self.tokens.pop().unwrap();

        for (kind, text) in leading {
            self.builder
                .token(MotokoLanguage::kind_to_raw(kind), text.into());
        }

        self.builder
            .token(MotokoLanguage::kind_to_raw(kind), text.into());

        for (kind, text) in trailing {
            self.builder
                .token(MotokoLanguage::kind_to_raw(kind), text.into());
        }
    }

    fn bump(&mut self, kind: SyntaxKind) {
        assert!(self.eat(kind))
    }

    fn eat(&mut self, kind: SyntaxKind) -> bool {
        if self.current() != kind {
            return false;
        }
        self.bump_any();
        true
    }

    fn nth(&self, n: usize) -> SyntaxKind {
        let len = self.tokens.len();
        if n >= len {
            SyntaxKind::EOF
        } else {
            self.tokens[len - n - 1].1 .0
        }
    }

    fn nth_at(&self, n: usize, kind: SyntaxKind) -> bool {
        self.nth(n) == kind
    }

    fn current(&self) -> SyntaxKind {
        self.nth(0)
    }

    fn at(&self, kind: SyntaxKind) -> bool {
        self.nth_at(0, kind)
    }

    fn at_ts(&mut self, kinds: TokenSet) -> bool {
        kinds.contains(self.current())
    }

    fn error(&mut self, msg: &str) {
        self.errors.push(msg.to_string())
    }
}

pub struct Parse {
    green_node: GreenNode,
    errors: Vec<String>,
}

impl Parse {
    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }

    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let formatted = format!("{:#?}", syntax_node);

        // We cut off the last byte because formatting the SyntaxNode adds on a newline at the end.
        formatted[0..formatted.len() - 1].to_string()
    }
}
