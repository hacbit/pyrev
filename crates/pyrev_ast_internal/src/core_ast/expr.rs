//! Python expression.

use super::expr_types::{Arguments, BinOpType, BoolOpType, Const, ExprContextType, UnaryOpType};
use super::seq::{ComprehensionSeq, ExprSeq, IntSeq, KeywordSeq};
use super::{AstNode, Ident};

pub struct Expr {
    pub value: ExprType,
    pub lineno: usize,
    pub col_offset: usize,
    pub end_lineno: usize,
    pub end_col_offset: usize,
}

impl AstNode for Expr {}

pub enum ExprType {
    BoolOp(BoolOp),
    NamedExpr(NamedExpr),
    BinOp(BinOp),
    UnaryOp(UnaryOp),
    Lambda(Lambda),
    IfExp(IfExp),
    Dict(Dict),
    Set(Set),
    ListComp(ListComp),
    SetComp(SetComp),
    DictComp(DictComp),
    GeneratorExp(GeneratorExp),
    Await(Await),
    Yield(Yield),
    YieldFrom(YieldFrom),
    Compare(Compare),
    Call(Call),
    FormattedValue(FormattedValue),
    JoinedStr(JoinedStr),
    Constant(Constant),
    Attribute(Attribute),
    Subscript(Subscript),
    Starred(Starred),
    Name(Name),
    List(List),
    Tuple(Tuple),
    Slice(Slice),
}

pub struct BoolOp {
    pub op: BoolOpType,
    pub values: Vec<ExprSeq>,
}

pub struct NamedExpr {
    pub target: Box<ExprType>,
    pub value: Box<ExprType>,
}

pub struct BinOp {
    pub left: Box<ExprType>,
    pub op: BinOpType,
    pub right: Box<ExprType>,
}

pub struct UnaryOp {
    pub op: UnaryOpType,
    pub operand: Box<ExprType>,
}

pub struct Lambda {
    pub args: Vec<Arguments>,
    pub body: Box<ExprType>,
}

pub struct IfExp {
    pub test: Box<ExprType>,
    pub body: Box<ExprType>,
    pub or_else: Box<ExprType>,
}

pub struct Dict {
    pub keys: Vec<ExprSeq>,
    pub values: Vec<ExprSeq>,
}

pub struct Set {
    pub elts: Vec<ExprSeq>,
}

pub struct ListComp {
    pub elt: Box<ExprType>,
    pub generators: Vec<ComprehensionSeq>,
}

pub struct SetComp {
    pub elt: Box<ExprType>,
    pub generators: Vec<ComprehensionSeq>,
}

pub struct DictComp {
    pub key: Box<ExprType>,
    pub value: Box<ExprType>,
    pub generators: Vec<ComprehensionSeq>,
}

pub struct GeneratorExp {
    pub elt: Box<ExprType>,
    pub generators: Vec<ComprehensionSeq>,
}

pub struct Await {
    pub value: Box<ExprType>,
}

pub struct Yield {
    pub value: Option<Box<ExprType>>,
}

pub struct YieldFrom {
    pub value: Box<ExprType>,
}

pub struct Compare {
    pub left: Box<ExprType>,
    pub ops: Vec<IntSeq>,
    pub comparators: Vec<ExprSeq>,
}

pub struct Call {
    pub func: Box<ExprType>,
    pub args: Vec<ExprSeq>,
    pub keywords: Vec<KeywordSeq>,
}

pub struct FormattedValue {
    pub value: Box<ExprType>,
    pub conversion: usize,
    pub format_spec: Option<Box<ExprType>>,
}

pub struct JoinedStr {
    pub values: Vec<ExprSeq>,
}

pub struct Constant {
    pub value: Const,
    pub kind: String,
}

pub struct Attribute {
    pub value: Box<ExprType>,
    pub attr: Ident,
    pub ctx: ExprContextType,
}

pub struct Subscript {
    pub value: Box<ExprType>,
    pub slice: Box<ExprType>,
    pub ctx: ExprContextType,
}

pub struct Starred {
    pub value: Box<ExprType>,
    pub ctx: ExprContextType,
}

pub struct Name {
    pub id: Ident,
    pub ctx: ExprContextType,
}

pub struct List {
    pub elts: Vec<ExprSeq>,
    pub ctx: ExprContextType,
}

pub struct Tuple {
    pub elts: Vec<ExprSeq>,
    pub ctx: ExprContextType,
}

pub struct Slice {
    pub lower: Option<Box<ExprType>>,
    pub upper: Option<Box<ExprType>>,
    pub step: Option<Box<ExprType>>,
}
