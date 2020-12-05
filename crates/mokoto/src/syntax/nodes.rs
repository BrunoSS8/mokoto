use rowan::SmolStr;

use crate::lexer::SyntaxKind::{self, *};
use crate::syntax::SyntaxNode;

use super::ast::{support, AstNode};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OptionalType {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for OptionalType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == OptionalT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

impl OptionalType {
    pub fn typ(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParenType {
    pub(crate) syntax: SyntaxNode,
}

impl ParenType {
    pub fn typ(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
}

impl AstNode for ParenType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ParenT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AsyncType {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for AsyncType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == AsyncT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathType {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for PathType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PathT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

impl PathType {
    pub fn path(&self) -> Path {
        support::child(&self.syntax).unwrap()
    }

    pub fn type_args(&self) -> Option<Vec<Type>> {
        let type_args = self.syntax.children().find(|n| n.kind() == TypArgs)?;
        let tys = support::children(&type_args);
        Some(tys.collect())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for Path {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Path
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

impl Path {
    pub fn segments(&self) -> Vec<SmolStr> {
        let idents = self.syntax.children_with_tokens().filter_map(|n| {
            n.as_token().and_then(|t| {
                if t.kind() == Ident {
                    Some(t.text().clone())
                } else {
                    None
                }
            })
        });
        idents.collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleType {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for TupleType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TupT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayType {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for ArrayType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ArrayT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncType {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for FuncType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == FuncT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

impl FuncType {
    pub fn arg_type(&self) -> Option<Type> {
        self.syntax.children().find_map(|c| {
            if c.kind() == FuncArg {
                support::child(&c)
            } else {
                None
            }
        })
    }
    pub fn result_type(&self) -> Option<Type> {
        self.syntax.children().find_map(|c| {
            if c.kind() == FuncResult {
                support::child(&c)
            } else {
                None
            }
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObjectType {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for ObjectType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ObjT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariantType {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for VariantType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VariantT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamedType {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for NamedType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == NamedT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrimType {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for PrimType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PrimT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    OptionalType(OptionalType),
    ParenType(ParenType),
    AsyncType(AsyncType),
    PathType(PathType),
    TupleType(TupleType),
    ArrayType(ArrayType),
    FuncType(FuncType),
    ObjectType(ObjectType),
    VariantType(VariantType),
    NamedType(NamedType),
    PrimType(PrimType),
}

impl AstNode for Type {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            OptionalT | ParenT | AsyncT | PathT | TupT | ArrayT | FuncT | ObjT | VariantT
            | NamedT | PrimT => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            OptionalT => Type::OptionalType(OptionalType { syntax }),
            ParenT => Type::ParenType(ParenType { syntax }),
            AsyncT => Type::AsyncType(AsyncType { syntax }),
            PathT => Type::PathType(PathType { syntax }),
            TupT => Type::TupleType(TupleType { syntax }),
            ArrayT => Type::ArrayType(ArrayType { syntax }),
            FuncT => Type::FuncType(FuncType { syntax }),
            ObjT => Type::ObjectType(ObjectType { syntax }),
            VariantT => Type::VariantType(VariantType { syntax }),
            NamedT => Type::NamedType(NamedType { syntax }),
            PrimT => Type::PrimType(PrimType { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Type::OptionalType(it) => &it.syntax,
            Type::ParenType(it) => &it.syntax,
            Type::AsyncType(it) => &it.syntax,
            Type::PathType(it) => &it.syntax,
            Type::TupleType(it) => &it.syntax,
            Type::ArrayType(it) => &it.syntax,
            Type::FuncType(it) => &it.syntax,
            Type::ObjectType(it) => &it.syntax,
            Type::VariantType(it) => &it.syntax,
            Type::NamedType(it) => &it.syntax,
            Type::PrimType(it) => &it.syntax,
        }
    }
}
