use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

pub(crate) struct Lexer<'a> {
    inner: logos::Lexer<'a, SyntaxKind>,
    lookahead: Option<(SyntaxKind, &'a str)>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            inner: SyntaxKind::lexer(input),
            lookahead: None,
        }
    }
}

fn is_trailing(kind: SyntaxKind) -> bool {
    matches!(kind, SyntaxKind::Space | SyntaxKind::Tab)
}

fn is_whitespace(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        SyntaxKind::Space | SyntaxKind::Tab | SyntaxKind::Linefeed
    )
}

impl<'a> Lexer<'a> {
    fn inner_peek(&mut self) -> Option<(SyntaxKind, &'a str)> {
        match self.lookahead {
            Some(i) => Some(i),
            None => {
                self.lookahead = Some((self.inner.next()?, self.inner.slice()));
                self.lookahead
            }
        }
    }
    fn inner_next(&mut self) -> Option<(SyntaxKind, &'a str)> {
        match self.lookahead {
            Some(i) => {
                self.lookahead = None;
                Some(i)
            }
            None => Some((self.inner.next()?, self.inner.slice())),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (
        Vec<(SyntaxKind, &'a str)>,
        (SyntaxKind, &'a str),
        Vec<(SyntaxKind, &'a str)>,
    );

    fn next(&mut self) -> Option<Self::Item> {
        let mut leading = vec![];
        let mut trailing = vec![];
        let mut next_token = (SyntaxKind::Eof, "");

        while matches!(self.inner_peek(), Some(tkn) if is_whitespace(tkn.0)) {
            leading.push(self.inner_next().unwrap())
        }

        if let Some(tkn) = self.inner_next() {
            next_token = tkn;
        }

        while matches!(self.inner_peek(), Some(tkn) if is_trailing(tkn.0)) {
            trailing.push(self.inner_next().unwrap())
        }

        Some((leading, next_token, trailing))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Logos, FromPrimitive, ToPrimitive)]
#[repr(u16)]
pub(crate) enum SyntaxKind {
    #[regex(" +")]
    Space,

    #[regex("\t+")]
    Tab,

    #[regex("\n|\r\n")]
    Linefeed,

    #[token("func")]
    FnKw,

    #[token("let")]
    LetKw,

    #[token("switch")]
    SwitchKw,

    #[token("case")]
    CaseKw,

    #[token("true")]
    TrueKw,

    #[token("false")]
    FalseKw,

    #[regex("[A-Za-z][A-Za-z0-9]*")]
    Ident,

    #[regex("[0-9]+")]
    NumberLit,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("=")]
    Equals,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token(";")]
    Semicolon,

    #[token(",")]
    Comma,

    #[token("->")]
    Arrow,

    #[error]
    Error,

    Eof,

    // Composite nodes
    // Expressions
    Parenthesize, // a parenthesized expression
    LiteralE,     // a literal
    VarE,         // wraps a WORD token
    FuncE,        // a func expression
    ApplicationE, // a function application
    BlockE,       // A block expression

    // Declarations
    LetD, // A let-declaration
    VarD, // A variable declaration
    Root,

    // Patterns
    VarP,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, kind: SyntaxKind) {
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some((vec![], (kind, input), vec![])));
    }

    #[test]
    fn lex_fn_keyword() {
        check("func", SyntaxKind::FnKw);
    }

    #[test]
    fn lex_let_keyword() {
        check("let", SyntaxKind::LetKw);
    }

    #[test]
    fn lex_alphabetic_identifier() {
        check("abcd", SyntaxKind::Ident);
    }

    #[test]
    fn lex_alphanumeric_identifier() {
        check("ab123cde456", SyntaxKind::Ident);
    }

    #[test]
    fn lex_mixed_case_identifier() {
        check("ABCdef", SyntaxKind::Ident);
    }

    #[test]
    fn lex_single_char_identifier() {
        check("x", SyntaxKind::Ident);
    }

    #[test]
    fn lex_number() {
        check("123456", SyntaxKind::NumberLit);
    }

    #[test]
    fn lex_plus() {
        check("+", SyntaxKind::Plus);
    }

    #[test]
    fn lex_minus() {
        check("-", SyntaxKind::Minus);
    }

    #[test]
    fn lex_star() {
        check("*", SyntaxKind::Star);
    }

    #[test]
    fn lex_slash() {
        check("/", SyntaxKind::Slash);
    }

    #[test]
    fn lex_equals() {
        check("=", SyntaxKind::Equals);
    }

    #[test]
    fn lex_left_parenthesis() {
        check("(", SyntaxKind::LParen);
    }

    #[test]
    fn lex_right_parenthesis() {
        check(")", SyntaxKind::RParen);
    }

    #[test]
    fn lex_left_brace() {
        check("{", SyntaxKind::LBrace);
    }

    #[test]
    fn lex_right_brace() {
        check("}", SyntaxKind::RBrace);
    }

    #[test]
    fn lex_semicolon() {
        check(";", SyntaxKind::Semicolon);
    }

    #[test]
    fn lex_arrow() {
        check("->", SyntaxKind::Arrow);
    }
}
