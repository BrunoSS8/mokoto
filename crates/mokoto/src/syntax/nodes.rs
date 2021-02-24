use super::{
    ast::{self, support, AstChildren, AstNode},
    SyntaxKind::{self, *},
    SyntaxNode, SyntaxToken,
};
use crate::T;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name {
    pub(crate) syntax: SyntaxNode,
}
impl Name {
    pub fn ident_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![ident])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Literal {
    pub(crate) syntax: SyntaxNode,
}
impl Literal {
    pub fn number_lit_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![number_lit])
    }
    pub fn true_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![true])
    }
    pub fn false_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![false])
    }
    pub fn null_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![null])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OptionalType {
    pub(crate) syntax: SyntaxNode,
}
impl OptionalType {
    pub fn question_mark_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![?])
    }
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParenType {
    pub(crate) syntax: SyntaxNode,
}
impl ParenType {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AsyncType {
    pub(crate) syntax: SyntaxNode,
}
impl AsyncType {
    pub fn async_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![async])
    }
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathType {
    pub(crate) syntax: SyntaxNode,
}
impl PathType {
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
    pub fn type_args(&self) -> Option<TypeArgs> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleType {
    pub(crate) syntax: SyntaxNode,
}
impl TupleType {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn fields(&self) -> AstChildren<Type> {
        support::children(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayType {
    pub(crate) syntax: SyntaxNode,
}
impl ArrayType {
    pub fn l_brack_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['['])
    }
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
    pub fn r_brack_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![']'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncType {
    pub(crate) syntax: SyntaxNode,
}
impl FuncType {
    pub fn shared_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![shared])
    }
    pub fn query_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![query])
    }
    pub fn type_params(&self) -> Option<TypeParams> {
        support::child(&self.syntax)
    }
    pub fn func_arg(&self) -> Option<FuncArg> {
        support::child(&self.syntax)
    }
    pub fn arrow_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![->])
    }
    pub fn func_result(&self) -> Option<FuncResult> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObjectType {
    pub(crate) syntax: SyntaxNode,
}
impl ObjectType {
    pub fn object_sort(&self) -> Option<ObjectSort> {
        support::child(&self.syntax)
    }
    pub fn l_brace_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['{'])
    }
    pub fn fields(&self) -> AstChildren<TypeField> {
        support::children(&self.syntax)
    }
    pub fn r_brace_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['}'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariantType {
    pub(crate) syntax: SyntaxNode,
}
impl VariantType {
    pub fn l_brace_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['{'])
    }
    pub fn variants(&self) -> AstChildren<TypeTag> {
        support::children(&self.syntax)
    }
    pub fn r_brace_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['}'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamedType {
    pub(crate) syntax: SyntaxNode,
}
impl NamedType {
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
    }
    pub fn colon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![:])
    }
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrimType {
    pub(crate) syntax: SyntaxNode,
}
impl PrimType {
    pub fn prim_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![prim])
    }
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path {
    pub(crate) syntax: SyntaxNode,
}
impl Path {
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeArgs {
    pub(crate) syntax: SyntaxNode,
}
impl TypeArgs {
    pub fn l_angle_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![<])
    }
    pub fn args(&self) -> AstChildren<Type> {
        support::children(&self.syntax)
    }
    pub fn r_angle_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![>])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeParams {
    pub(crate) syntax: SyntaxNode,
}
impl TypeParams {
    pub fn l_angle_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![<])
    }
    pub fn params(&self) -> AstChildren<TypeBind> {
        support::children(&self.syntax)
    }
    pub fn r_angle_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![>])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncArg {
    pub(crate) syntax: SyntaxNode,
}
impl FuncArg {
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncResult {
    pub(crate) syntax: SyntaxNode,
}
impl FuncResult {
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeBind {
    pub(crate) syntax: SyntaxNode,
}
impl TypeBind {
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
    pub fn type_bound(&self) -> Option<TypeBound> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeBound {
    pub(crate) syntax: SyntaxNode,
}
impl TypeBound {
    pub fn sub_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![<:])
    }
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObjectSort {
    pub(crate) syntax: SyntaxNode,
}
impl ObjectSort {
    pub fn object_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![object])
    }
    pub fn class_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![class])
    }
    pub fn actor_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![actor])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeField {
    pub(crate) syntax: SyntaxNode,
}
impl TypeField {
    pub fn var_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![var])
    }
    pub fn type_annotation(&self) -> Option<TypeAnnotation> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeFieldFunc {
    pub(crate) syntax: SyntaxNode,
}
impl TypeFieldFunc {
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
    }
    pub fn type_params(&self) -> Option<TypeParams> {
        support::child(&self.syntax)
    }
    pub fn func_type(&self) -> Option<FuncType> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeAnnotation {
    pub(crate) syntax: SyntaxNode,
}
impl TypeAnnotation {
    pub fn colon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![:])
    }
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeTag {
    pub(crate) syntax: SyntaxNode,
}
impl TypeTag {
    pub fn hash_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![#])
    }
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
    }
    pub fn type_annotation(&self) -> Option<TypeAnnotation> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WildcardPat {
    pub(crate) syntax: SyntaxNode,
}
impl WildcardPat {
    pub fn underscore_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![_])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VarPat {
    pub(crate) syntax: SyntaxNode,
}
impl VarPat {
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LiteralPat {
    pub(crate) syntax: SyntaxNode,
}
impl LiteralPat {
    pub fn literal(&self) -> Option<Literal> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParenPat {
    pub(crate) syntax: SyntaxNode,
}
impl ParenPat {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn pattern(&self) -> Option<Pattern> {
        support::child(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TuplePat {
    pub(crate) syntax: SyntaxNode,
}
impl TuplePat {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn patterns(&self) -> AstChildren<Pattern> {
        support::children(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObjectPat {
    pub(crate) syntax: SyntaxNode,
}
impl ObjectPat {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn fields(&self) -> AstChildren<PatternField> {
        support::children(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PatternFieldPun {
    pub(crate) syntax: SyntaxNode,
}
impl PatternFieldPun {
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
    }
    pub fn type_annotation(&self) -> Option<TypeAnnotation> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PatternFieldPat {
    pub(crate) syntax: SyntaxNode,
}
impl PatternFieldPat {
    pub fn ident_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![ident])
    }
    pub fn type_annotation(&self) -> Option<TypeAnnotation> {
        support::child(&self.syntax)
    }
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![=])
    }
    pub fn pattern(&self) -> Option<Pattern> {
        support::child(&self.syntax)
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ObjectField {
    TypeField(TypeField),
    TypeFieldFunc(TypeFieldFunc),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Pattern {
    WildcardPat(WildcardPat),
    VarPat(VarPat),
    LiteralPat(LiteralPat),
    ParenPat(ParenPat),
    TuplePat(TuplePat),
    ObjectPat(ObjectPat),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatternField {
    PatternFieldPun(PatternFieldPun),
    PatternFieldPat(PatternFieldPat),
}
impl AstNode for Name {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == NAME
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
impl AstNode for Literal {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LITERAL
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
impl AstNode for OptionalType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == OPTIONAL_TYPE
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
impl AstNode for ParenType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PAREN_TYPE
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
impl AstNode for AsyncType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ASYNC_TYPE
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
impl AstNode for PathType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PATH_TYPE
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
impl AstNode for TupleType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TUPLE_TYPE
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
impl AstNode for ArrayType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ARRAY_TYPE
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
impl AstNode for FuncType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == FUNC_TYPE
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
impl AstNode for ObjectType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == OBJECT_TYPE
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
impl AstNode for VariantType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VARIANT_TYPE
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
impl AstNode for NamedType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == NAMED_TYPE
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
impl AstNode for PrimType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PRIM_TYPE
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
impl AstNode for Path {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PATH
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
impl AstNode for TypeArgs {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TYPE_ARGS
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
impl AstNode for TypeParams {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TYPE_PARAMS
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
impl AstNode for FuncArg {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == FUNC_ARG
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
impl AstNode for FuncResult {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == FUNC_RESULT
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
impl AstNode for TypeBind {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TYPE_BIND
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
impl AstNode for TypeBound {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TYPE_BOUND
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
impl AstNode for ObjectSort {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == OBJECT_SORT
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
impl AstNode for TypeField {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TYPE_FIELD
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
impl AstNode for TypeFieldFunc {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TYPE_FIELD_FUNC
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
impl AstNode for TypeAnnotation {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TYPE_ANNOTATION
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
impl AstNode for TypeTag {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TYPE_TAG
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
impl AstNode for WildcardPat {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == WILDCARD_PAT
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
impl AstNode for VarPat {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VAR_PAT
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
impl AstNode for LiteralPat {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LITERAL_PAT
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
impl AstNode for ParenPat {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PAREN_PAT
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
impl AstNode for TuplePat {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TUPLE_PAT
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
impl AstNode for ObjectPat {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == OBJECT_PAT
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
impl AstNode for PatternFieldPun {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PATTERN_FIELD_PUN
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
impl AstNode for PatternFieldPat {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PATTERN_FIELD_PAT
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
impl From<OptionalType> for Type {
    fn from(node: OptionalType) -> Type {
        Type::OptionalType(node)
    }
}
impl From<ParenType> for Type {
    fn from(node: ParenType) -> Type {
        Type::ParenType(node)
    }
}
impl From<AsyncType> for Type {
    fn from(node: AsyncType) -> Type {
        Type::AsyncType(node)
    }
}
impl From<PathType> for Type {
    fn from(node: PathType) -> Type {
        Type::PathType(node)
    }
}
impl From<TupleType> for Type {
    fn from(node: TupleType) -> Type {
        Type::TupleType(node)
    }
}
impl From<ArrayType> for Type {
    fn from(node: ArrayType) -> Type {
        Type::ArrayType(node)
    }
}
impl From<FuncType> for Type {
    fn from(node: FuncType) -> Type {
        Type::FuncType(node)
    }
}
impl From<ObjectType> for Type {
    fn from(node: ObjectType) -> Type {
        Type::ObjectType(node)
    }
}
impl From<VariantType> for Type {
    fn from(node: VariantType) -> Type {
        Type::VariantType(node)
    }
}
impl From<NamedType> for Type {
    fn from(node: NamedType) -> Type {
        Type::NamedType(node)
    }
}
impl From<PrimType> for Type {
    fn from(node: PrimType) -> Type {
        Type::PrimType(node)
    }
}
impl AstNode for Type {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            OPTIONAL_TYPE | PAREN_TYPE | ASYNC_TYPE | PATH_TYPE | TUPLE_TYPE | ARRAY_TYPE
            | FUNC_TYPE | OBJECT_TYPE | VARIANT_TYPE | NAMED_TYPE | PRIM_TYPE => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            OPTIONAL_TYPE => Type::OptionalType(OptionalType { syntax }),
            PAREN_TYPE => Type::ParenType(ParenType { syntax }),
            ASYNC_TYPE => Type::AsyncType(AsyncType { syntax }),
            PATH_TYPE => Type::PathType(PathType { syntax }),
            TUPLE_TYPE => Type::TupleType(TupleType { syntax }),
            ARRAY_TYPE => Type::ArrayType(ArrayType { syntax }),
            FUNC_TYPE => Type::FuncType(FuncType { syntax }),
            OBJECT_TYPE => Type::ObjectType(ObjectType { syntax }),
            VARIANT_TYPE => Type::VariantType(VariantType { syntax }),
            NAMED_TYPE => Type::NamedType(NamedType { syntax }),
            PRIM_TYPE => Type::PrimType(PrimType { syntax }),
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
impl From<TypeField> for ObjectField {
    fn from(node: TypeField) -> ObjectField {
        ObjectField::TypeField(node)
    }
}
impl From<TypeFieldFunc> for ObjectField {
    fn from(node: TypeFieldFunc) -> ObjectField {
        ObjectField::TypeFieldFunc(node)
    }
}
impl AstNode for ObjectField {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TYPE_FIELD | TYPE_FIELD_FUNC => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TYPE_FIELD => ObjectField::TypeField(TypeField { syntax }),
            TYPE_FIELD_FUNC => ObjectField::TypeFieldFunc(TypeFieldFunc { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            ObjectField::TypeField(it) => &it.syntax,
            ObjectField::TypeFieldFunc(it) => &it.syntax,
        }
    }
}
impl From<WildcardPat> for Pattern {
    fn from(node: WildcardPat) -> Pattern {
        Pattern::WildcardPat(node)
    }
}
impl From<VarPat> for Pattern {
    fn from(node: VarPat) -> Pattern {
        Pattern::VarPat(node)
    }
}
impl From<LiteralPat> for Pattern {
    fn from(node: LiteralPat) -> Pattern {
        Pattern::LiteralPat(node)
    }
}
impl From<ParenPat> for Pattern {
    fn from(node: ParenPat) -> Pattern {
        Pattern::ParenPat(node)
    }
}
impl From<TuplePat> for Pattern {
    fn from(node: TuplePat) -> Pattern {
        Pattern::TuplePat(node)
    }
}
impl From<ObjectPat> for Pattern {
    fn from(node: ObjectPat) -> Pattern {
        Pattern::ObjectPat(node)
    }
}
impl AstNode for Pattern {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            WILDCARD_PAT | VAR_PAT | LITERAL_PAT | PAREN_PAT | TUPLE_PAT | OBJECT_PAT => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            WILDCARD_PAT => Pattern::WildcardPat(WildcardPat { syntax }),
            VAR_PAT => Pattern::VarPat(VarPat { syntax }),
            LITERAL_PAT => Pattern::LiteralPat(LiteralPat { syntax }),
            PAREN_PAT => Pattern::ParenPat(ParenPat { syntax }),
            TUPLE_PAT => Pattern::TuplePat(TuplePat { syntax }),
            OBJECT_PAT => Pattern::ObjectPat(ObjectPat { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Pattern::WildcardPat(it) => &it.syntax,
            Pattern::VarPat(it) => &it.syntax,
            Pattern::LiteralPat(it) => &it.syntax,
            Pattern::ParenPat(it) => &it.syntax,
            Pattern::TuplePat(it) => &it.syntax,
            Pattern::ObjectPat(it) => &it.syntax,
        }
    }
}
impl From<PatternFieldPun> for PatternField {
    fn from(node: PatternFieldPun) -> PatternField {
        PatternField::PatternFieldPun(node)
    }
}
impl From<PatternFieldPat> for PatternField {
    fn from(node: PatternFieldPat) -> PatternField {
        PatternField::PatternFieldPat(node)
    }
}
impl AstNode for PatternField {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PATTERN_FIELD_PUN | PATTERN_FIELD_PAT => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            PATTERN_FIELD_PUN => PatternField::PatternFieldPun(PatternFieldPun { syntax }),
            PATTERN_FIELD_PAT => PatternField::PatternFieldPat(PatternFieldPat { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            PatternField::PatternFieldPun(it) => &it.syntax,
            PatternField::PatternFieldPat(it) => &it.syntax,
        }
    }
}
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ObjectField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PatternField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for OptionalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ParenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AsyncType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PathType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TupleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ArrayType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for FuncType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for VariantType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for NamedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PrimType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TypeArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TypeParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for FuncArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for FuncResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TypeBind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TypeBound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ObjectSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TypeField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TypeFieldFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TypeAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TypeTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for WildcardPat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for VarPat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for LiteralPat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ParenPat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TuplePat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ObjectPat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PatternFieldPun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PatternFieldPat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
