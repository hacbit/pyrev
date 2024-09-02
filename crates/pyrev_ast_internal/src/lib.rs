//! Python AST internal representation.

#![forbid(unsafe_code)]
// #![deny(missing_docs)]

pub mod core_ast;

#[allow(missing_docs, non_upper_case_globals)]
pub const PyCF_ONLY_AST: usize = 1024;
#[allow(missing_docs, non_upper_case_globals)]
pub const PyCF_TYPE_COMMENTS: usize = 4096;
#[allow(missing_docs, non_upper_case_globals)]
pub const PyCF_ALLOW_TOP_LEVEL_AWAIT: usize = 8192;
#[allow(missing_docs, non_upper_case_globals)]
pub const PyCF_OPTIMIZED: usize = 33792;

#[derive(Default)]
pub struct Unparser {
    pub source: Vec<String>,
    pub type_ignores: Vec<String>,
    pub indent: usize,
    pub avoid_backslashes: bool,
    pub in_try_star: bool,
}

impl Unparser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_avoid_backslashes(avoid_backslashes: bool) -> Self {
        Self {
            avoid_backslashes,
            ..Self::default()
        }
    }

    /// Write a single line of text.
    #[inline(always)]
    pub fn write_single(&mut self, text: String) {
        self.source.push(text);
    }

    /// Write a single line of text.
    #[inline(always)]
    pub fn write_single_str(&mut self, text: &str) {
        self.write_single(text.to_string());
    }

    /// Write a piece of text.
    #[inline(always)]
    pub fn write(&mut self, texts: &[String]) {
        self.source.extend_from_slice(texts);
    }

    /// Write a piece of text.
    #[inline(always)]
    pub fn write_strs(&mut self, texts: &[&str]) {
        self.source.extend(texts.iter().map(|s| s.to_string()));
    }

    /// Add a newline if it is not already present.
    #[inline(always)]
    pub fn maybe_newline(&mut self) {
        if self
            .source
            .last()
            .map_or(false, |line| !line.ends_with('\n'))
        {
            self.write_single_str("\n");
        }
    }

    /// Indent a piece of text and append it, according to the current indentation level.
    #[inline(always)]
    pub fn fill(&mut self, text: &str) {
        self.maybe_newline();
        self.write_single(" ".repeat(self.indent) + text);
    }
}

impl Unparser {
    /// Output a source code string.
    pub fn visit(&mut self, node: &core_ast::Node) {
        match node {
            &core_ast::Node::Expr(ref expr) => self.visit_expr(expr),
            &core_ast::Node::Stmt(ref stmt) => self.visit_stmt(stmt),
            &core_ast::Node::Seq(ref seq) => self.visit_seq(seq),
            &core_ast::Node::Mod(ref module) => self.visit_mod(module),
            &core_ast::Node::Nodes(ref nodes) => {
                for node in nodes {
                    self.visit(node);
                }
            }
        }
    }

    pub fn visit_expr(&mut self, expr: &core_ast::Expr) {
        match &expr.value {
            &core_ast::ExprType::BoolOp(ref bool_op) => {
                self.visit_bool_op(bool_op);
            }
            &core_ast::ExprType::NamedExpr(ref named_expr) => {
                self.visit_named_expr(named_expr);
            }
            &core_ast::ExprType::BinOp(ref bin_op) => {
                self.visit_bin_op(bin_op);
            }
            &core_ast::ExprType::UnaryOp(ref unary_op) => {
                self.visit_unary_op(unary_op);
            }
            &core_ast::ExprType::Lambda(ref lambda) => {
                self.visit_lambda(lambda);
            }
            &core_ast::ExprType::IfExp(ref if_exp) => {
                self.visit_if_exp(if_exp);
            }
            &core_ast::ExprType::Dict(ref dict) => {
                self.visit_dict(dict);
            }
            &core_ast::ExprType::Set(ref set) => {
                self.visit_set(set);
            }
            &core_ast::ExprType::ListComp(ref list_comp) => {
                self.visit_list_comp(list_comp);
            }
            &core_ast::ExprType::SetComp(ref set_comp) => {
                self.visit_set_comp(set_comp);
            }
            &core_ast::ExprType::DictComp(ref dict_comp) => {
                self.visit_dict_comp(dict_comp);
            }
            &core_ast::ExprType::GeneratorExp(ref generator_exp) => {
                self.visit_generator_exp(generator_exp);
            }
            &core_ast::ExprType::Await(ref await_) => {
                self.visit_await(await_);
            }
            &core_ast::ExprType::Yield(ref yield_) => {
                self.visit_yield(yield_);
            }
            &core_ast::ExprType::YieldFrom(ref yield_from) => {
                self.visit_yield_from(yield_from);
            }
            &core_ast::ExprType::Compare(ref compare) => {
                self.visit_compare(compare);
            }
            &core_ast::ExprType::Call(ref call) => {
                self.visit_call(call);
            }
            &core_ast::ExprType::FormattedValue(ref formatted_value) => {
                self.visit_formatted_value(formatted_value);
            }
            &core_ast::ExprType::JoinedStr(ref joined_str) => {
                self.visit_joined_str(joined_str);
            }
            &core_ast::ExprType::Constant(ref constant) => {
                self.visit_constant(constant);
            }
            &core_ast::ExprType::Attribute(ref attribute) => {
                self.visit_attribute(attribute);
            }
            &core_ast::ExprType::Subscript(ref subscript) => {
                self.visit_subscript(subscript);
            }
            &core_ast::ExprType::Starred(ref starred) => {
                self.visit_starred(starred);
            }
            &core_ast::ExprType::Name(ref name) => {
                self.visit_name(name);
            }
            &core_ast::ExprType::List(ref list) => {
                self.visit_list(list);
            }
            &core_ast::ExprType::Tuple(ref tuple) => {
                self.visit_tuple(tuple);
            }
            &core_ast::ExprType::Slice(ref slice) => {
                self.visit_slice(slice);
            }
        }
    }

    pub fn visit_bool_op(&mut self, bool_op: &core_ast::BoolOp) {}

    pub fn visit_named_expr(&mut self, named_expr: &core_ast::NamedExpr) {}

    pub fn visit_bin_op(&mut self, bin_op: &core_ast::BinOp) {}

    pub fn visit_unary_op(&mut self, unary_op: &core_ast::UnaryOp) {}

    pub fn visit_lambda(&mut self, lambda: &core_ast::Lambda) {}

    pub fn visit_if_exp(&mut self, if_exp: &core_ast::IfExp) {}

    pub fn visit_dict(&mut self, dict: &core_ast::Dict) {}

    pub fn visit_set(&mut self, set: &core_ast::Set) {}

    pub fn visit_list_comp(&mut self, list_comp: &core_ast::ListComp) {}

    pub fn visit_set_comp(&mut self, set_comp: &core_ast::SetComp) {}

    pub fn visit_dict_comp(&mut self, dict_comp: &core_ast::DictComp) {}

    pub fn visit_generator_exp(&mut self, generator_exp: &core_ast::GeneratorExp) {}

    pub fn visit_await(&mut self, await_: &core_ast::Await) {}

    pub fn visit_yield(&mut self, yield_: &core_ast::Yield) {}

    pub fn visit_yield_from(&mut self, yield_from: &core_ast::YieldFrom) {}

    pub fn visit_compare(&mut self, compare: &core_ast::Compare) {}

    pub fn visit_call(&mut self, call: &core_ast::Call) {}

    pub fn visit_formatted_value(&mut self, formatted_value: &core_ast::FormattedValue) {}

    pub fn visit_joined_str(&mut self, joined_str: &core_ast::JoinedStr) {}

    pub fn visit_constant(&mut self, constant: &core_ast::Constant) {}

    pub fn visit_attribute(&mut self, attribute: &core_ast::Attribute) {}

    pub fn visit_subscript(&mut self, subscript: &core_ast::Subscript) {}

    pub fn visit_starred(&mut self, starred: &core_ast::Starred) {}

    pub fn visit_name(&mut self, name: &core_ast::Name) {}

    pub fn visit_list(&mut self, list: &core_ast::List) {}

    pub fn visit_tuple(&mut self, tuple: &core_ast::Tuple) {}

    pub fn visit_slice(&mut self, slice: &core_ast::Slice) {}

    pub fn visit_stmt(&mut self, stmt: &core_ast::Stmt) {
        match &stmt.value {
            &core_ast::StmtType::AnnAssign(ref ann_assign) => {
                self.visit_ann_assign(ann_assign);
            }
            &core_ast::StmtType::Assert(ref assert_) => {
                self.visit_assert(assert_);
            }
            &core_ast::StmtType::Assign(ref assign) => {
                self.visit_assign(assign);
            }
            &core_ast::StmtType::AsyncFor(ref async_for) => {
                self.visit_async_for(async_for);
            }
            &core_ast::StmtType::AsyncFunctionDef(ref async_function_def) => {
                self.visit_async_function_def(async_function_def);
            }
            &core_ast::StmtType::AsyncWith(ref async_with) => {
                self.visit_async_with(async_with);
            }
            &core_ast::StmtType::AugAssign(ref aug_assign) => {
                self.visit_aug_assign(aug_assign);
            }
            &core_ast::StmtType::Break(ref break_) => {
                self.visit_break(break_);
            }
            &core_ast::StmtType::ClassDef(ref class_def) => {
                self.visit_class_def(class_def);
            }
            &core_ast::StmtType::Continue(ref continue_) => {
                self.visit_continue(continue_);
            }
            &core_ast::StmtType::Delete(ref delete) => {
                self.visit_delete(delete);
            }
            &core_ast::StmtType::For(ref for_) => {
                self.visit_for(for_);
            }
            &core_ast::StmtType::FunctionDef(ref function_def) => {
                self.visit_function_def(function_def);
            }
            &core_ast::StmtType::Expr(ref expr) => {
                self.visit_expr(expr);
            }
            &core_ast::StmtType::Global(ref global) => {
                self.visit_global(global);
            }
            &core_ast::StmtType::If(ref if_) => {
                self.visit_if(if_);
            }
            &core_ast::StmtType::Import(ref import) => {
                self.visit_import(import);
            }
            &core_ast::StmtType::ImportFrom(ref import_from) => {
                self.visit_import_from(import_from);
            }
            &core_ast::StmtType::Match(ref match_) => {
                self.visit_match(match_);
            }
            &core_ast::StmtType::Nonlocal(ref nonlocal) => {
                self.visit_nonlocal(nonlocal);
            }
            &core_ast::StmtType::Pass(ref pass) => {
                self.visit_pass(pass);
            }
            &core_ast::StmtType::Raise(ref raise) => {
                self.visit_raise(raise);
            }
            &core_ast::StmtType::Return(ref return_) => {
                self.visit_return(return_);
            }
            &core_ast::StmtType::Try(ref try_) => {
                self.visit_try(try_);
            }
            &core_ast::StmtType::TryStar(ref try_star) => {
                self.visit_try_star(try_star);
            }
            &core_ast::StmtType::TypeAlias(ref type_alias) => {
                self.visit_type_alias(type_alias);
            }
            &core_ast::StmtType::While(ref while_) => {
                self.visit_while(while_);
            }
            &core_ast::StmtType::With(ref with) => {
                self.visit_with(with);
            }
        }
    }

    pub fn visit_ann_assign(&mut self, ann_assign: &core_ast::AnnAssign) {}

    pub fn visit_assert(&mut self, assert_: &core_ast::Assert) {}

    pub fn visit_assign(&mut self, assign: &core_ast::Assign) {}

    pub fn visit_async_for(&mut self, async_for: &core_ast::AsyncFor) {}

    pub fn visit_async_function_def(&mut self, async_function_def: &core_ast::AsyncFunctionDef) {}

    pub fn visit_async_with(&mut self, async_with: &core_ast::AsyncWith) {}

    pub fn visit_aug_assign(&mut self, aug_assign: &core_ast::AugAssign) {}

    pub fn visit_break(&mut self, break_: &core_ast::Break) {}

    pub fn visit_class_def(&mut self, class_def: &core_ast::ClassDef) {}

    pub fn visit_continue(&mut self, continue_: &core_ast::Continue) {}

    pub fn visit_delete(&mut self, delete: &core_ast::Delete) {}

    pub fn visit_for(&mut self, for_: &core_ast::For) {}

    pub fn visit_function_def(&mut self, function_def: &core_ast::FunctionDef) {}

    pub fn visit_global(&mut self, global: &core_ast::Global) {}

    pub fn visit_if(&mut self, if_: &core_ast::If) {}

    pub fn visit_import(&mut self, import: &core_ast::Import) {
        self.fill("import ");
        import.names.iter().for_each(|alias_seq| {
            self.visit_alias_seq(alias_seq);
        })
    }

    pub fn visit_import_from(&mut self, import_from: &core_ast::ImportFrom) {}

    pub fn visit_match(&mut self, match_: &core_ast::Match) {}

    pub fn visit_nonlocal(&mut self, nonlocal: &core_ast::Nonlocal) {}

    pub fn visit_pass(&mut self, pass: &core_ast::Pass) {}

    pub fn visit_raise(&mut self, raise: &core_ast::Raise) {}

    pub fn visit_return(&mut self, return_: &core_ast::Return) {}

    pub fn visit_try(&mut self, try_: &core_ast::Try) {}

    pub fn visit_try_star(&mut self, try_star: &core_ast::TryStar) {}

    pub fn visit_type_alias(&mut self, type_alias: &core_ast::TypeAlias) {}

    pub fn visit_while(&mut self, while_: &core_ast::While) {}

    pub fn visit_with(&mut self, with: &core_ast::With) {}

    pub fn visit_seq(&mut self, seq: &core_ast::Seq) {
        match &seq.value {
            &core_ast::SeqType::AliasSeq(ref alias_seq) => {
                self.visit_alias_seq(alias_seq);
            }
            &core_ast::SeqType::ArgSeq(ref arg_seq) => {
                self.visit_arg_seq(arg_seq);
            }
            &core_ast::SeqType::ArgumentsSeq(ref arguments_seq) => {
                self.visit_arguments_seq(arguments_seq);
            }
            &core_ast::SeqType::ComprehensionSeq(ref comprehension_seq) => {
                self.visit_comprehension_seq(comprehension_seq);
            }
            &core_ast::SeqType::ExceptHandlerSeq(ref except_handler_seq) => {
                self.visit_except_handler_seq(except_handler_seq);
            }
            &core_ast::SeqType::ExprSeq(ref expr_seq) => {
                self.visit_expr_seq(expr_seq);
            }
            &core_ast::SeqType::IdentSeq(ref ident_seq) => {
                self.visit_ident_seq(ident_seq);
            }
            &core_ast::SeqType::IntSeq(ref int_seq) => {
                self.visit_int_seq(int_seq);
            }
            &core_ast::SeqType::KeywordSeq(ref keyword_seq) => {
                self.visit_keyword_seq(keyword_seq);
            }
            &core_ast::SeqType::MatchCaseSeq(ref match_case_seq) => {
                self.visit_match_case_seq(match_case_seq);
            }
            &core_ast::SeqType::ModSeq(ref mod_seq) => {
                self.visit_mod_seq(mod_seq);
            }
            &core_ast::SeqType::PatternSeq(ref pattern_seq) => {
                self.visit_pattern_seq(pattern_seq);
            }
            &core_ast::SeqType::StmtSeq(ref stmt_seq) => {
                self.visit_stmt_seq(stmt_seq);
            }
            &core_ast::SeqType::TypeIgnoreSeq(ref type_ignore_seq) => {
                self.visit_type_ignore_seq(type_ignore_seq);
            }
            &core_ast::SeqType::TypeParamSeq(ref type_param_seq) => {
                self.visit_type_param_seq(type_param_seq);
            }
            &core_ast::SeqType::WithItemSeq(ref with_item_seq) => {
                self.visit_with_item_seq(with_item_seq);
            }
        }
    }

    pub fn visit_alias_seq(&mut self, alias_seq: &core_ast::AliasSeq) {}

    pub fn visit_arg_seq(&mut self, arg_seq: &core_ast::ArgSeq) {}

    pub fn visit_arguments_seq(&mut self, arguments_seq: &core_ast::ArgumentsSeq) {}

    pub fn visit_comprehension_seq(&mut self, comprehension_seq: &core_ast::ComprehensionSeq) {}

    pub fn visit_except_handler_seq(&mut self, except_handler_seq: &core_ast::ExceptHandlerSeq) {}

    pub fn visit_expr_seq(&mut self, expr_seq: &core_ast::ExprSeq) {}

    pub fn visit_ident_seq(&mut self, ident_seq: &core_ast::IdentSeq) {}

    pub fn visit_int_seq(&mut self, int_seq: &core_ast::IntSeq) {}

    pub fn visit_keyword_seq(&mut self, keyword_seq: &core_ast::KeywordSeq) {}

    pub fn visit_match_case_seq(&mut self, match_case_seq: &core_ast::MatchCaseSeq) {}

    pub fn visit_mod_seq(&mut self, mod_seq: &core_ast::ModSeq) {}

    pub fn visit_pattern_seq(&mut self, pattern_seq: &core_ast::PatternSeq) {}

    pub fn visit_stmt_seq(&mut self, stmt_seq: &core_ast::StmtSeq) {}

    pub fn visit_type_ignore_seq(&mut self, type_ignore_seq: &core_ast::TypeIgnoreSeq) {}

    pub fn visit_type_param_seq(&mut self, type_param_seq: &core_ast::TypeParamSeq) {}

    pub fn visit_with_item_seq(&mut self, with_item_seq: &core_ast::WithItemSeq) {}

    pub fn visit_mod(&mut self, module: &core_ast::Mod) {
        match &module.value {
            &core_ast::ModType::Module(ref module) => {
                self.visit_module(module);
            }
            &core_ast::ModType::Interactive(ref interactive) => {
                self.visit_interactive(interactive);
            }
            &core_ast::ModType::Expression(ref expression) => {
                self.visit_expression(expression);
            }
            &core_ast::ModType::FunctionType(ref function_type) => {
                self.visit_function_type(function_type);
            }
        }
    }

    pub fn visit_module(&mut self, module: &core_ast::Module) {}

    pub fn visit_interactive(&mut self, interactive: &core_ast::Interactive) {}

    pub fn visit_expression(&mut self, expression: &core_ast::Expression) {}

    pub fn visit_function_type(&mut self, function_type: &core_ast::FunctionType) {}
}

pub fn unparse(node: &core_ast::Node) -> String {
    let mut unparser = Unparser::new();
    unparser.visit(node);
    unparser.source.join("")
}
