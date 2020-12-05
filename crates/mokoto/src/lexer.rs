use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

pub(crate) type Token<'a> = (
    Vec<(SyntaxKind, &'a str)>,
    (SyntaxKind, &'a str),
    Vec<(SyntaxKind, &'a str)>,
);

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

    pub(crate) fn lex(input: &'a str) -> Vec<Token<'a>> {
        let mut lexer = Lexer::new(input);
        let mut tokens = vec![];
        loop {
            let tkn = lexer.next().unwrap();
            if tkn.1 .0 == SyntaxKind::Eof {
                tokens.push(tkn);
                return tokens;
            }
            tokens.push(tkn);
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
pub enum SyntaxKind {
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

    #[token("var")]
    VarKw,

    #[token("switch")]
    SwitchKw,

    #[token("case")]
    CaseKw,

    #[token("true")]
    TrueKw,

    #[token("false")]
    FalseKw,

    #[token("shared")]
    SharedKw,

    #[token("query")]
    QueryKw,

    #[token("async")]
    AsyncKw,

    #[token("object")]
    ObjectKw,

    #[token("class")]
    ClassKw,

    #[token("actor")]
    ActorKw,

    #[token("module")]
    ModuleKw,

    #[token("prim")]
    PrimKw,

    #[regex("[A-Za-z][A-Za-z0-9]*")]
    Ident,

    #[regex("[0-9]+")]
    NumberLit,

    #[token(".")]
    Dot,

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

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("<")]
    Lt,

    #[token(">")]
    Gt,

    #[token("<:")]
    Sub,

    #[token("#")]
    Hash,

    #[token("?")]
    Question,

    #[token(":")]
    Colon,

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
    // Types
    OptionalT,
    ParenT,
    AsyncT,
    PathT,
    ArrayT,
    FuncT,
    ObjT,
    VariantT,
    TupT,
    NamedT,
    PrimT,

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

    // Modifiers
    Path,
    Name,
    MutModifier,
    TypArgs,
    TypParams,
    TypBind,
    TypTag,
    TypField,
    TypFieldFunc,
    TypAnnot,
    FuncArg,
    FuncResult,
    FuncSort,
    ObjSort,
}
