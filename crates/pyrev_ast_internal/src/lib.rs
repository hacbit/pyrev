//! Python AST internal representation.

#![forbid(unsafe_code)]
// #![deny(missing_docs)]

pub mod core_ast;

use std::marker::PhantomData;

#[allow(missing_docs, non_upper_case_globals)]
pub const PyCF_ONLY_AST: usize = 1024;
#[allow(missing_docs, non_upper_case_globals)]
pub const PyCF_TYPE_COMMENTS: usize = 4096;
#[allow(missing_docs, non_upper_case_globals)]
pub const PyCF_ALLOW_TOP_LEVEL_AWAIT: usize = 8192;
#[allow(missing_docs, non_upper_case_globals)]
pub const PyCF_OPTIMIZED: usize = 33792;

struct Ast {
    match_args: PhantomData<()>,
}
