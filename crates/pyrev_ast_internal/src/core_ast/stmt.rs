//! Python statements.

use super::{
    expr::Expr,
    expr_types::{Arguments, BinOpType},
    seq::{
        AliasSeq, ExceptHandlerSeq, ExprSeq, IdentSeq, KeywordSeq, MatchCaseSeq, StmtSeq,
        TypeParamSeq, WithItemSeq,
    },
    AstNode, Ident,
};

pub struct Stmt {
    pub value: StmtType,
    pub lineno: usize,
    pub col_offset: usize,
    pub end_lineno: usize,
    pub end_col_offset: usize,
}

impl AstNode for Stmt {}

pub enum StmtType {
    FunctionDef(FunctionDef),
    AsyncFunctionDef(AsyncFunctionDef),
    ClassDef(ClassDef),
    Return(Return),
    Delete(Delete),
    Assign(Assign),
    TypeAlias(TypeAlias),
    AugAssign(AugAssign),
    AnnAssign(AnnAssign),
    For(For),
    AsyncFor(AsyncFor),
    While(While),
    If(If),
    With(With),
    AsyncWith(AsyncWith),
    Match(Match),
    Raise(Raise),
    Try(Try),
    TryStar(TryStar),
    Assert(Assert),
    Import(Import),
    ImportFrom(ImportFrom),
    Global(Global),
    Nonlocal(Nonlocal),
    Expr(Expr),
    Pass(Pass),
    Break(Break),
    Continue(Continue),
}

pub struct FunctionDef {
    pub name: Ident,
    pub args: Arguments,
    pub body: Vec<StmtSeq>,
    pub decorator_list: Vec<ExprSeq>,
    pub returns: Expr,
    pub type_comment: String,
    pub type_params: Vec<TypeParamSeq>,
}

pub struct AsyncFunctionDef {
    pub name: Ident,
    pub args: Arguments,
    pub body: Vec<StmtSeq>,
    pub decorator_list: Vec<ExprSeq>,
    pub returns: Expr,
    pub type_comment: String,
    pub type_params: Vec<TypeParamSeq>,
}

pub struct ClassDef {
    pub name: Ident,
    pub bases: Vec<ExprSeq>,
    pub keywords: Vec<KeywordSeq>,
    pub body: Vec<StmtSeq>,
    pub decorator_list: Vec<ExprSeq>,
    pub type_params: Vec<TypeParamSeq>,
}

pub struct Return {
    pub value: Expr,
}

pub struct Delete {
    pub targets: Vec<ExprSeq>,
}

pub struct Assign {
    pub targets: Vec<ExprSeq>,
    pub value: Expr,
    pub type_comment: String,
}

pub struct TypeAlias {
    pub name: Expr,
    pub type_params: Vec<TypeParamSeq>,
    pub value: Expr,
}

pub struct AugAssign {
    pub target: Expr,
    pub op: BinOpType,
    pub value: Expr,
}

pub struct AnnAssign {
    pub target: Expr,
    pub annotation: Expr,
    pub value: Expr,
    pub simple: usize,
}

pub struct For {
    pub target: Expr,
    pub iter: Expr,
    pub body: Vec<StmtSeq>,
    pub or_else: Vec<StmtSeq>,
    pub type_comment: String,
}

pub struct AsyncFor {
    pub target: Expr,
    pub iter: Expr,
    pub body: Vec<StmtSeq>,
    pub or_else: Vec<StmtSeq>,
    pub type_comment: String,
}

pub struct While {
    pub test: Expr,
    pub body: Vec<StmtSeq>,
    pub or_else: Vec<StmtSeq>,
}

pub struct If {
    pub test: Expr,
    pub body: Vec<StmtSeq>,
    pub or_else: Vec<StmtSeq>,
}

pub struct With {
    pub items: Vec<WithItemSeq>,
    pub body: Vec<StmtSeq>,
    pub type_comment: String,
}

pub struct AsyncWith {
    pub items: Vec<WithItemSeq>,
    pub body: Vec<StmtSeq>,
    pub type_comment: String,
}

pub struct Match {
    pub subject: Expr,
    pub cases: Vec<MatchCaseSeq>,
}

pub struct Raise {
    pub except: Expr,
    pub cause: Expr,
}

pub struct Try {
    pub body: Vec<StmtSeq>,
    pub handlers: Vec<ExceptHandlerSeq>,
    pub or_else: Vec<StmtSeq>,
    pub final_body: Vec<StmtSeq>,
}

pub struct TryStar {
    pub body: Vec<StmtSeq>,
    pub handlers: Vec<ExceptHandlerSeq>,
    pub or_else: Vec<StmtSeq>,
    pub final_body: Vec<StmtSeq>,
}

pub struct Assert {
    pub test: Expr,
    pub msg: Expr,
}

pub struct Import {
    pub names: Vec<AliasSeq>,
}

pub struct ImportFrom {
    pub module: Ident,
    pub names: Vec<AliasSeq>,
    pub level: usize,
}

pub struct Global {
    pub names: Vec<IdentSeq>,
}

pub struct Nonlocal {
    pub names: Vec<IdentSeq>,
}

// deprecated
// use expr::Expr instead
/* pub struct Expr {
    pub value: expr::Expr,
} */

pub struct Pass {}

pub struct Break {}

pub struct Continue {}
