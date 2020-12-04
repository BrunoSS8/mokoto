use crate::lexer::SyntaxKind;
use num_traits::{FromPrimitive, ToPrimitive};
use rowan::SmolStr;

pub mod ast;
pub mod nodes;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum MotokoLanguage {}

pub(crate) type SyntaxNode = rowan::SyntaxNode<MotokoLanguage>;
pub(crate) type SyntaxToken = rowan::SyntaxToken<MotokoLanguage>;
pub(crate) type SyntaxElement = rowan::SyntaxElement<MotokoLanguage>;
pub(crate) type SyntaxNodeChildren = rowan::SyntaxNodeChildren<MotokoLanguage>;
pub(crate) type SyntaxElementChildren = rowan::SyntaxElementChildren<MotokoLanguage>;

impl rowan::Language for MotokoLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::from_u16(raw.0).unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_u16().unwrap())
    }
}
