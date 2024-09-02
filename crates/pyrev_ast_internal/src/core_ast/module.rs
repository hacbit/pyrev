//! Python mod

use super::{expr::Expr, seq::{ExprSeq, StmtSeq, TypeIgnoreSeq}};


pub struct Mod {
    pub value: ModType,
}

pub enum ModType {
    Module(Module),
    Interactive(Interactive),
    Expression(Expression),
    FunctionType(FunctionType),
}

pub struct Module {
    pub body: Vec<StmtSeq>,
    pub type_ignores: Vec<TypeIgnoreSeq>,
}

pub struct Interactive {
    pub body: Vec<StmtSeq>,
}

pub struct Expression {
    pub body: Expr,
}

pub struct FunctionType {
    pub arg_types: Vec<ExprSeq>,
    pub returns: Expr,
}