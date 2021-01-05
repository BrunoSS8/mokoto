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
            if tkn.1 .0 == SyntaxKind::EOF {
                tokens.push(tkn);
                return tokens;
            }
            tokens.push(tkn);
        }
    }
}

fn is_trailing(kind: SyntaxKind) -> bool {
    matches!(kind, SyntaxKind::SPACE | SyntaxKind::TAB)
}

fn is_whitespace(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        SyntaxKind::SPACE | SyntaxKind::TAB | SyntaxKind::LINEFEED
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
        let mut next_token = (SyntaxKind::EOF, "");

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
#[allow(bad_style, missing_docs, unreachable_pub)]
pub enum SyntaxKind {
    #[regex(" +")]
    SPACE,

    #[regex("\t+")]
    TAB,

    #[regex("\n|\r\n")]
    LINEFEED,

    #[token("func")]
    FUNC_KW,

    #[token("let")]
    LET_KW,

    #[token("var")]
    VAR_KW,

    #[token("switch")]
    SWITCH_KW,

    #[token("case")]
    CASE_KW,

    #[token("true")]
    TRUE_KW,

    #[token("false")]
    FALSE_KW,

    #[token("shared")]
    SHARED_KW,

    #[token("query")]
    QUERY_KW,

    #[token("async")]
    ASYNC_KW,

    #[token("object")]
    OBJECT_KW,

    #[token("class")]
    CLASS_KW,

    #[token("actor")]
    ACTOR_KW,

    #[token("module")]
    MODULE_KW,

    #[token("prim")]
    PRIM_KW,

    #[regex("[A-Za-z][A-Za-z0-9]*")]
    IDENT,

    #[regex("[0-9]+")]
    NUMBER_LIT,

    #[token(".")]
    DOT,

    #[token("+")]
    PLUS,

    #[token("-")]
    MINUS,

    #[token("*")]
    STAR,

    #[token("/")]
    SLASH,

    #[token("=")]
    EQUALS,

    #[token("==")]
    DOUBLE_EQUALS,

    #[token("(")]
    L_PAREN,

    #[token(")")]
    R_PAREN,

    #[token("[")]
    L_BRACKET,

    #[token("]")]
    R_BRACKET,

    #[token("{")]
    L_BRACE,

    #[token("}")]
    R_BRACE,

    #[token("<")]
    L_ANGLE,

    #[token(">")]
    R_ANGLE,

    #[token("<:")]
    SUB,

    #[token("#")]
    HASH,

    #[token("?")]
    QUESTION,

    #[token("!")]
    BANG,

    #[token(":")]
    COLON,

    #[token(";")]
    SEMICOLON,

    #[token(",")]
    COMMA,

    #[token("->")]
    ARROW,

    #[error]
    ERROR,

    EOF,

    // Composite nodes
    // Types
    OPTIONAL_TYPE,
    PAREN_TYPE,
    ASYNC_TYPE,
    PATH_TYPE,
    ARRAY_TYPE,
    FUNC_TYPE,
    OBJECT_TYPE,
    VARIANT_TYPE,
    TUPLE_TYPE,
    NAMED_TYPE,
    PRIM_TYPE,

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
    PATH,
    NAME,
    MutModifier,
    TYPE_ARGS,
    TYPE_PARAMS,
    TYPE_BIND,
    TYPE_BOUND,
    TYPE_TAG,
    TYPE_FIELD,
    TYPE_FIELD_FUNC,
    TYPE_ANNOTATION,
    FUNC_ARG,
    FUNC_RESULT,
    FUNC_SORT,
    OBJECT_SORT,
}

#[macro_export]
macro_rules ! T {
    [;] => { SyntaxKind::SEMICOLON };
    [,] => { SyntaxKind::COMMA };
    ['('] => { SyntaxKind::L_PAREN };
    [')'] => { SyntaxKind::R_PAREN };
    ['{'] => { SyntaxKind::L_BRACE };
    ['}'] => { SyntaxKind::R_BRACE };
    ['['] => { SyntaxKind::L_BRACKET };
    [']'] => { SyntaxKind::R_BRACKET };
    [<] => { SyntaxKind::L_ANGLE };
    [>] => { SyntaxKind::R_ANGLE };
    [#] => { SyntaxKind::HASH };
    [?] => { SyntaxKind::QUESTION };
    [+] => { SyntaxKind::PLUS };
    [*] => { SyntaxKind::STAR };
    [/] => { SyntaxKind::SLASH };
    [^] => { SyntaxKind::CARET };
    [%] => { SyntaxKind::PERCENT };
    [_] => { SyntaxKind::UNDERSCORE };
    [.] => { SyntaxKind::DOT };
    [:] => { SyntaxKind::COLON };
    [=] => { SyntaxKind::EQUALS };
    [==] => { SyntaxKind::DOUBLE_EQUALS };
    [!] => { SyntaxKind::BANG };
    [-] => { SyntaxKind::MINUS };
    [->] => { SyntaxKind::ARROW };
    [<:] => { SyntaxKind::SUB };
    [actor] => { SyntaxKind::ACTOR_KW };
    [class] => { SyntaxKind::CLASS_KW };
    [object] => { SyntaxKind::OBJECT_KW };
    [false] => { SyntaxKind::FALSE_KW };
    [async] => { SyntaxKind::ASYNC_KW };
    [false] => { SyntaxKind::FALSE_KW };
    [func] => { SyntaxKind::FUNC_KW };
    [for] => { SyntaxKind::FOR_KW };
    [if] => { SyntaxKind::IF_KW };
    [else] => { SyntaxKind::ELSE_KW };
    [let] => { SyntaxKind::LET_KW };
    [switch] => { SyntaxKind::SWITCH_KW };
    [module] => { SyntaxKind::MODULE_KW };
    [var] => { SyntaxKind::VAR_KW };
    [prim] => { SyntaxKind::PRIM_KW };
    [query] => { SyntaxKind::QUERY_KW };
    [shared] => { SyntaxKind::SHARED_KW };
    [return] => { SyntaxKind::RETURN_KW };
    [true] => { SyntaxKind::TRUE_KW };
    [try] => { SyntaxKind::TRY_KW };
    [type] => { SyntaxKind::TYPE_KW };
    [while] => { SyntaxKind::WHILE_KW };
    [ident] => { SyntaxKind::IDENT };
}
