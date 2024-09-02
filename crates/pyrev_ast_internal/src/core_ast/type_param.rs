//! Python Type Param

use super::{expr::Expr, Ident};

pub struct TypeParam {
    pub value: TypeParamType,
    pub lineno: usize,
    pub col_offset: usize,
    pub end_lineno: usize,
    pub end_col_offset: usize,
}

pub enum TypeParamType {
    TypeVar(TypeVar),
    ParamSpec(ParamSpec),
    TypeVarTuple(TypeVarTuple),
}

pub struct TypeVar {
    pub name: Ident,
    pub bound: Expr,
    pub default_value: Expr,
}

pub struct ParamSpec {
    pub name: Ident,
    pub default_value: Expr,
}

pub struct TypeVarTuple {
    pub name: Ident,
    pub default_value: Expr,
}
