use crate::lexer::SyntaxKind;
use crate::syntax::nodes::{Pattern, PatternField};
use num_traits::{FromPrimitive, ToPrimitive};
use rowan::SmolStr;

pub mod ast;
pub mod nodes;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum MotokoLanguage {}

pub type SyntaxNode = rowan::SyntaxNode<MotokoLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<MotokoLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<MotokoLanguage>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<MotokoLanguage>;
pub type SyntaxElementChildren = rowan::SyntaxElementChildren<MotokoLanguage>;

impl rowan::Language for MotokoLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::from_u16(raw.0).unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_u16().unwrap())
    }
}

impl nodes::Path {
    pub fn segments(&self) -> Vec<String> {
        let idents = self.syntax.children_with_tokens().filter_map(|n| {
            n.as_token().and_then(|t| {
                if t.kind() == crate::T![ident] {
                    Some(t.text().to_string())
                } else {
                    None
                }
            })
        });
        idents.collect()
    }
}

impl nodes::Pattern {
    pub fn idents(&self) -> Vec<nodes::Name> {
        match self {
            Pattern::WildcardPat(_) => vec![],
            Pattern::VarPat(n) => vec![n.name().unwrap()],
            Pattern::LiteralPat(_) => vec![],
            Pattern::ParenPat(p) => p.pattern().unwrap().idents(),
            Pattern::TuplePat(p) => {
                let mut res = vec![];
                for p in p.patterns() {
                    res.append(&mut p.idents())
                }
                res
            }
            Pattern::ObjectPat(p) => {
                let mut res = vec![];
                for field in p.fields() {
                    let mut names = match field {
                        PatternField::PatternFieldPun(p) => vec![p.name().unwrap()],
                        PatternField::PatternFieldPat(p) => p.pattern().unwrap().idents(),
                    };
                    res.append(&mut names);
                }
                res
            }
            Pattern::VariantPat(p) => {
                p.pattern().map_or(vec![], |p| p.idents())
            }
        }
    }
}
