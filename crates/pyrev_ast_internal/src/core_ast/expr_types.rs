//! This module contains the types for the expression AST nodes.

use super::{
    expr::ExprType,
    pattern::Pattern,
    seq::{ArgSeq, ExprSeq, StmtSeq},
    Ident,
};

pub enum ExprContextType {
    Load,
    Store,
    Del,
}

pub enum BoolOpType {
    And,
    Or,
}

pub enum BinOpType {
    Add,
    Sub,
    Mult,
    MatMult,
    Div,
    Mod,
    Pow,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
    FloorDiv,
}

pub enum UnaryOpType {
    Invert,
    Not,
    UAdd,
    USub,
}

pub enum CmpOpType {
    Eq,
    NotEq,
    Lt,
    LtE,
    Gt,
    GtE,
    Is,
    IsNot,
    In,
    NotIn,
}

pub struct Comprehension {
    pub target: ExprType,
    pub iter: ExprType,
    pub ifs: Vec<ExprSeq>,
    pub is_async: bool,
}

pub struct ExceptHandler {
    pub ty: Option<ExprType>,
    pub name: Ident,
    pub body: Vec<StmtSeq>,
    pub lineno: usize,
    pub col_offset: usize,
    pub end_lineno: usize,
    pub end_col_offset: usize,
}

pub struct Arguments {
    pub pos_only_args: Vec<ArgSeq>,
    pub args: Vec<ArgSeq>,
    pub var_arg: Arg,
    pub kw_only_args: Vec<ArgSeq>,
    pub kw_defaults: Vec<ExprSeq>,
    pub kw_arg: Arg,
    pub defaults: Vec<ExprSeq>,
}

pub struct Arg {
    pub arg: Ident,
    pub annotation: Option<ExprType>,
    pub type_comment: Option<String>,
    pub lineno: usize,
    pub col_offset: usize,
    pub end_lineno: usize,
    pub end_col_offset: usize,
}

pub struct Keyword {
    pub arg: Ident,
    pub value: ExprType,
    pub lineno: usize,
    pub col_offset: usize,
    pub end_lineno: usize,
    pub end_col_offset: usize,
}

pub struct Alias {
    pub name: Ident,
    pub as_name: Option<Ident>,
    pub lineno: usize,
    pub col_offset: usize,
    pub end_lineno: usize,
    pub end_col_offset: usize,
}

pub struct WithItem {
    pub context_expr: ExprType,
    pub optional_vars: Option<ExprType>,
}

pub struct MatchCase {
    pub pattern: Pattern,
    pub guard: Option<ExprType>,
    pub body: Vec<StmtSeq>,
}

pub struct TypeIgnore {
    pub lineno: usize,
    pub tag: String,
}

pub struct Const {
    pub name: String,
    pub value: usize,
}
