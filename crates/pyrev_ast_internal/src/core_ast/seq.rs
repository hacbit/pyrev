//! Python expression sequence.

use super::{
    expr::Expr, expr_types::{
        Alias, Arg, Arguments, Comprehension, ExceptHandler, Keyword, MatchCase, TypeIgnore,
        WithItem,
    }, module::Mod, pattern::Pattern, stmt::Stmt, type_param::TypeParam, Ident
};

#[macro_export]
macro_rules! seq_head {
    (
        $(
            $vis:vis struct $id:ident {
                $( $_v:vis $field : ident:$ty:ty ),*$(,)?
            }
        )*
    ) => {
        $(
            $vis struct $id {
                pub size: usize,
                pub elements: Vec<Vec<()>>,
                $( $_v $field:$ty ),*
            }
        )*
    };
}

seq_head! {
    pub struct IdentSeq {
        pub typed_elements: Ident,
    }

    pub struct IntSeq {
        pub typed_elements: i32,
    }

    pub struct ModSeq {
        pub typed_elements: [Mod; 1],
    }

    pub struct StmtSeq {
        pub typed_elements: [Stmt; 1],
    }

    pub struct ExprSeq {
        pub typed_elements: [Expr; 1],
    }

    pub struct ComprehensionSeq {
        pub typed_elements: [Comprehension; 1],
    }

    pub struct ExceptHandlerSeq {
        pub typed_elements: [ExceptHandler; 1],
    }

    pub struct ArgumentsSeq {
        pub typed_elements: [Arguments; 1],
    }

    pub struct ArgSeq {
        pub typed_elements: [Arg; 1],
    }

    pub struct KeywordSeq {
        pub typed_elements: [Keyword; 1],
    }

    pub struct AliasSeq {
        pub typed_elements: [Alias; 1],
    }

    pub struct WithItemSeq {
        pub typed_elements: [WithItem; 1],
    }

    pub struct MatchCaseSeq {
        pub typed_elements: [MatchCase; 1],
    }

    pub struct PatternSeq {
        pub typed_elements: [Pattern; 1],
    }

    pub struct TypeIgnoreSeq {
        pub typed_elements: [TypeIgnore; 1],
    }

    pub struct TypeParamSeq {
        pub typed_elements: [TypeParam; 1],
    }
}
