//! Python match case patterns

use super::{
    expr::{Constant, Expr},
    seq::{ExprSeq, IdentSeq, PatternSeq},
    Ident,
};

pub struct Pattern {
    pub value: PatternType,
    pub lineno: usize,
    pub col_offset: usize,
    pub end_lineno: usize,
    pub end_col_offset: usize,
}

pub enum PatternType {
    MatchValue(MatchValue),
    MatchSingleton(MatchSingleton),
    MatchSequence(MatchSequence),
    MatchMapping(MatchMapping),
    MatchClass(MatchClass),
    MatchStar(MatchStar),
    MatchAs(MatchAs),
    MatchOr(MatchOr),
}

pub struct MatchValue {
    pub value: Expr,
}

pub struct MatchSingleton {
    pub value: Constant,
}

pub struct MatchSequence {
    pub patterns: Vec<PatternSeq>,
}

pub struct MatchMapping {
    pub keys: Vec<ExprSeq>,
    pub patterns: Vec<PatternSeq>,
    pub rest: Ident,
}

pub struct MatchClass {
    pub class: Expr,
    pub patterns: Vec<PatternSeq>,
    pub kwd_attrs: Vec<IdentSeq>,
    pub kwd_patterns: Vec<PatternSeq>,
}

pub struct MatchStar {
    pub name: Ident,
}

pub struct MatchAs {
    pub pattern: Box<Pattern>,
    pub name: Ident,
}

pub struct MatchOr {
    pub patterns: Vec<PatternSeq>,
}
