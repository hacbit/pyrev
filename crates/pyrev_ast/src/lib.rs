//! The internal AST for pyrev

#![forbid(unsafe_code, missing_docs)]

use bevy_reflect::Reflect;
pub use pyrev_ast_derive::*;
pub use pyrev_query_inner::QueryId;
use regex::Regex;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Expression trait is used to mark the struct as an expression
pub trait Expression {}

/// ## Import module expression
/// 
/// In python, it is like this:
/// ```python
/// import os
/// from os import path, system
/// import os as o
/// from os import path as p
/// ```
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct Import {
    /// The imported module name
    pub module: String,
    /// The module name which is imported from
    pub bk_module: Option<String>,
    /// The fragment of the module
    pub fragment: Option<String>,
    /// The alias of the module
    pub alias: Option<String>,
    /// The start line of the import expression
    pub start_line: usize,
    /// The start offset of the import expression
    pub start_offset: usize,
    /// The end offset of the import expression
    pub end_offset: usize,
}

/// ## Python Class expression
/// 
/// In python, it is like this:
/// ```python
/// class A:
///     def __init__(self):
///         pass
/// ```
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct Class {
    /// The marker of the class object
    /// 
    /// It is like `<code object A at 0x0000000000000000, file "path", line 1>`
    pub mark: String,
    /// The name of the class
    pub name: String,
    /// The members of the class
    /// 
    /// Each method or variable in the class is a member
    pub members: Vec<QueryId>,
    /// The start line of the class expression
    pub start_line: usize,
    /// The start offset of the class expression
    pub start_offset: usize,
    /// The end offset of the class expression
    pub end_offset: usize,
}

/// ## Local variable or function parameter
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct FastVariable {
    /// The variable index in the local scope
    pub index: usize,
    /// The name of the variable
    pub name: String,
    /// The annotation of the variable
    /// 
    /// It is the type hint of the variable
    pub annotation: Option<String>,
    /// The start line of the variable expression
    pub start_line: usize,
    /// The start offset of the variable expression
    pub start_offset: usize,
    /// The end offset of the variable expression
    pub end_offset: usize,
}

/// ## Python function expression
/// 
/// It also contains the `<lambda>`, `<listcomp>` and other special functions or generators
/// 
/// In python, it is like this:
/// ```python
/// def func(a: int = 1) -> int:
///     return a
/// b = lambda x: x + 1
/// c = [i for i in range(10)]
/// d = {i: i for i in range(10)}
/// ```
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct Function {
    /// The marker of the function object
    /// 
    /// It is like `<code object A at 0x0000000000000000, file "path", line 1>`
    pub mark: String,
    /// The name of the function
    pub name: String,
    /// The arguments of the function
    pub args: Vec<QueryId>,
    /// The default values of the arguments
    pub ret: Option<QueryId>,
    /// The body of the function
    pub bodys: Vec<QueryId>,
    /// The default values of the arguments
    pub defaults: Vec<String>,
    /// Mark whether the function is async
    pub is_async: bool,
    /// The start line of the function expression
    pub start_line: usize,
    /// The end line of the function expression
    pub end_line: usize,
    /// The start offset of the function expression
    pub start_offset: usize,
    /// The end offset of the function expression
    pub end_offset: usize,
}

/// ## Return expression
/// 
/// It is used in the function
/// 
/// In python, it is like this:
/// ```python
/// def func():
///     return 1
/// ```
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct Return {
    /// The value of the return expression
    pub value: Option<QueryId>,
    /// The start line of the return expression
    pub start_line: usize,
    /// The start offset of the return expression
    pub start_offset: usize,
    /// The end offset of the return expression
    pub end_offset: usize,
}

/// ## Yield expression
/// 
/// It is used in the generator function
/// 
/// In python, it is like this:
/// ```python
/// def gen():
///     yield 1
/// ```
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct Yield {
    /// The value of the yield expression
    pub value: Option<QueryId>,
    /// The start line of the yield expression
    pub start_line: usize,
    /// The start offset of the yield expression
    pub start_offset: usize,
    /// The end offset of the yield expression
    pub end_offset: usize,
}

/// ## Assign expression
/// 
/// It is used to assign the value to the variable.
/// 
/// And it also contains some self-assign operators.
/// 
/// In python, it is like this:
/// ```python
/// a = 1
/// ```
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct Assign {
    /// The target variable name
    pub target: Option<QueryId>,
    /// The value of the assignment
    pub value: Option<QueryId>,
    /// The operator of the assignment
    /// 
    /// It is like `=`, `+=`, `-=`, `*=`, `/=`, `%=`, `<<=`, `>>=`, `&=`, `|=`, `^=` and so on.
    pub operator: String,
    /// The start line of the assignment expression
    pub start_line: usize,
    /// The start offset of the assignment expression
    pub start_offset: usize,
    /// The end offset of the assignment expression
    pub end_offset: usize,
}

/// ## Alias expression
/// 
/// It very like the [`Assign`], but is only used `as` operator.
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct Alias {
    /// The variable which will be aliased
    pub target: Option<QueryId>,
    /// The alias name
    pub alias: Option<QueryId>,
    /// The start line of the alias expression
    pub start_line: usize,
    /// The start offset of the alias expression
    pub start_offset: usize,
    /// The end offset of the alias expression
    pub end_offset: usize,
}

/// ## Try expression
/// 
/// It is used to catch the exception in the block.
/// 
/// In python, it is like this:
/// ```python
/// try:
///     pass
/// except Exception as e:
///     pass
/// finally:
///     pass
/// ```
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct Try {
    /// The body of the try block
    pub body: Vec<QueryId>,
    /// The except block
    pub except: Vec<QueryId>,
    /// The finally block
    pub finally: Option<QueryId>,
    /// The start line of the try expression
    pub start_line: usize,
    /// The start offset of the try expression
    pub start_offset: usize,
    /// The end offset of the try expression
    pub end_offset: usize,
}

/// ## Except expression
/// 
/// It is used to catch the exception in the block.
/// 
/// Learn more from [`Try`]
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct Except {
    /// The exception type
    pub exception: Option<QueryId>,
    /// The body of the except block
    pub body: Vec<QueryId>,
    /// The start line of the except expression
    pub start_line: usize,
    /// The start offset of the except expression
    pub start_offset: usize,
    /// The end offset of the except expression
    pub end_offset: usize,
}

/// ## Finally expression
/// 
/// It is used to execute the block after the try block.
/// 
/// Learn more from [`Try`]
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct Finally {
    /// The body of the finally block
    pub body: Vec<QueryId>,
    /// The start line of the finally expression
    pub start_line: usize,
    /// The start offset of the finally expression
    pub start_offset: usize,
    /// The end offset of the finally expression
    pub end_offset: usize,
}

/// ## Assertion expression
/// 
/// It is used to assert the expression is true.
/// 
/// In python, it is like this:
/// ```python
/// assert a == 1, "a should be 1"
/// assert a > 0
/// ```
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct Assert {
    /// The test expression. It should be true.
    pub test: Option<QueryId>,
    /// The message of the assertion
    /// 
    /// If the assertion is failed, the message will be printed.
    /// 
    /// Learn more from [`Raise`]
    pub msg: Option<QueryId>,
    /// The start line of the assertion expression
    pub start_line: usize,
    /// The start offset of the assertion expression
    pub start_offset: usize,
    /// The end offset of the assertion expression
    pub end_offset: usize,
}

/// ## Raise expression
/// 
/// It will actively throw an exception.
/// 
/// In python, it is like this:
/// ```python
/// raise Exception("error")
/// ```
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct Raise {
    /// The exception expression
    pub exception: Option<QueryId>,
    /// The start line of the raise expression
    pub start_line: usize,
    /// The start offset of the raise expression
    pub start_offset: usize,
    /// The end offset of the raise expression
    pub end_offset: usize,
}

/// ## Format value expression
/// 
/// It is used to format the value.
/// 
/// In python, it is like this:
/// ```python
/// f"{a}"
/// ```
/// 
/// It will format the value of `a` to the string.
/// 
/// Learn more from [`Format`]
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct FormatValue {
    /// The value of the format expression
    pub value: Option<QueryId>,
    /// The start line of the format value expression
    pub start_line: usize,
    /// The start offset of the format value expression
    pub start_offset: usize,
    /// The end offset of the format value expression
    pub end_offset: usize,
}

/// ## Format expression
/// 
/// It contains the format, and some format values.
/// 
/// The first format value is the format string.
/// 
/// And the other format values are the values which will be formatted.
/// Learn more from [`FormatValue`]
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct Format {
    /// The format string and format values
    pub format_values: Vec<QueryId>,
    /// The start line of the format expression
    pub start_line: usize,
    /// The start offset of the format expression
    pub start_offset: usize,
    /// The end offset of the format expression
    pub end_offset: usize,
}

/// ## Binary operation expression
/// 
/// It is used to operate the two values.
/// 
/// It contains this operators:
/// - `+`, `-`, `*`, `/`, `//`, `%`, `**`, `<<`, etc.
/// 
/// In python, it is like this:
/// ```python
/// a + b
/// ```
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct BinaryOperation {
    /// The left value of the binary operation
    pub left: Option<QueryId>,
    /// The right value of the binary operation
    pub right: Option<QueryId>,
    /// The operator of the binary operation
    pub operator: String,
    /// The start line of the binary operation expression
    pub start_line: usize,
    /// The start offset of the binary operation expression
    pub start_offset: usize,
    /// The end offset of the binary operation expression
    pub end_offset: usize,
}

/// ## Subscript expression
/// 
/// It is used to get the value from the container.
/// 
/// In python, it is like this:
/// ```python
/// a[0]
/// ```
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct Subscr {
    /// The index of the subscript
    pub index: Option<QueryId>,
    /// The container
    pub target: Option<QueryId>,
    /// The start line of the subscript expression
    pub start_line: usize,
    /// The start offset of the subscript expression
    pub start_offset: usize,
    /// The end offset of the subscript expression
    pub end_offset: usize,
}

/// ## Unary operation expression
/// 
/// It is used to operate the single value.
/// 
/// It contains this operators:
/// - `+`, `-`, `~`, `not`
/// 
/// In python, it is like this:
/// ```python
/// -a
/// +a
/// ~a
/// not a
/// ```
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct UnaryOperation {
    /// The target value of the unary operation
    pub target: Option<QueryId>,
    /// The type of the unary operation
    pub unary_type: UnaryType,
    /// The start line of the unary operation expression
    pub start_line: usize,
    /// The start offset of the unary operation expression
    pub start_offset: usize,
    /// The end offset of the unary operation expression
    pub end_offset: usize,
}

/// ## Call expression
/// 
/// It is used to call the function.
/// 
/// In python, it is like this:
/// ```python
/// a()
/// print("hello")
/// ```
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct Call {
    /// The function name or expression
    /// 
    /// It can be a variable, a function or a lambda.
    pub func: Option<QueryId>,
    /// The arguments of the call expression
    pub args: Vec<QueryId>,
    /// The start line of the call expression
    pub start_line: usize,
    /// The start offset of the call expression
    pub start_offset: usize,
    /// The end offset of the call expression
    pub end_offset: usize,
}

/// ## With expression
/// 
/// It is used to open the context manager.
/// 
/// In python, it is like this:
/// ```python
/// with open("file.txt") as f:
///     pass
/// ```
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct With {
    /// The item of the with expression
    /// 
    /// It is the context manager.
    pub item: Option<QueryId>,
    /// The body of the with expression
    pub body: Vec<QueryId>,
    /// Mark whether the with expression is async
    pub is_async: bool,
    /// The start line of the with expression
    pub start_line: usize,
    /// The start offset of the with expression
    pub start_offset: usize,
    /// The end offset of the with expression
    pub end_offset: usize,
}

/// ## For expression
/// 
/// It is used to iterate the container.
/// 
/// In python, it is like this:
/// ```python
/// for i in range(10):
///     pass
/// ```
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct For {
    /// The iterator of the for expression
    pub iterator: Option<QueryId>,
    /// The items of the for expression
    pub items: Option<QueryId>,
    /// The body of the for expression
    pub body: Vec<QueryId>,
    /// The start line of the for expression
    pub from: usize,
    /// The end line of the for expression
    pub to: usize,
    /// Mark whether the for expression is async
    pub is_async: bool,
    /// The start line of the for expression
    pub start_line: usize,
    /// The start offset of the for expression
    pub start_offset: usize,
    /// The end offset of the for expression
    pub end_offset: usize,
}

/// ## If expression
/// 
/// It is used to judge the condition.
/// 
/// In python, it is like this:
/// ```python
/// if a == 1:
///     pass
/// elif a == 2:
///     pass
/// else:
///     pass
/// ```
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct If {
    /// The test condition expression of the if expression
    pub test: Option<QueryId>,
    /// The body of the if expression
    pub body: Vec<QueryId>,
    /// The elif / else expression
    /// 
    /// It connects the next branch of the if expression.
    pub or_else: Option<QueryId>,
    /// The start line of the if expression
    pub start_line: usize,
    /// The start offset of the if expression
    pub start_offset: usize,
    /// The end offset of the if expression
    pub end_offset: usize,
}

/// ## Jump expression
/// 
/// It records the jump target of the jump expression.
/// 
/// And it contains some jump operations likes `break`, `continue`, etc.
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct Jump {
    /// The target of the jump expression
    pub target: usize,
    /// Mark whether the jump expression is backward
    pub is_backward: bool,
    /// The start line of the jump expression
    pub start_line: usize,
    /// The start offset of the jump expression
    pub start_offset: usize,
    /// The end offset of the jump expression
    pub end_offset: usize,
}

/// ## Await expression
/// 
/// It is used to wait the awaitable object.
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct Await {
    /// The awaitable expression
    pub awaitable_expr: Option<QueryId>,
    /// The start line of the await expression
    pub start_line: usize,
    /// The start offset of the await expression
    pub start_offset: usize,
    /// The end offset of the await expression
    pub end_offset: usize,
}

/// ## Container expression
/// 
/// It is used to contain the values.
/// 
/// It contains the list, tuple, set, dict, etc.
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct Container {
    /// The values of the container
    pub values: Vec<QueryId>,
    /// The type of the container
    pub container_type: ContainerType,
    /// The start line of the container expression
    pub start_line: usize,
    /// The start offset of the container expression
    pub start_offset: usize,
    /// The end offset of the container expression
    pub end_offset: usize,
}

/// ## Attribute expression
/// 
/// It is used to get the attribute of the object.
/// 
/// In python, it is like this:
/// ```python
/// a.b
/// ```
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct Attribute {
    /// The parent object
    pub parent: Option<QueryId>,
    /// The attribute name
    pub attr: Option<QueryId>,
    /// The start line of the attribute expression
    pub start_line: usize,
    /// The start offset of the attribute expression
    pub start_offset: usize,
    /// The end offset of the attribute expression
    pub end_offset: usize,
}

/// ## Slice expression
/// 
/// It is used to get the slice of the object.
/// 
/// In python, it is like this:
/// ```python
/// a[1:2]
/// arr[::-1]
/// bbb[1:10:2]
/// ```
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct Slice {
    /// The origin object
    pub origin: Option<QueryId>,
    /// The start, stop, step of the slice expression
    pub slice: Vec<QueryId>,
    /// The start line of the slice expression
    pub start_line: usize,
    /// The start offset of the slice expression
    pub start_offset: usize,
    /// The end offset of the slice expression
    pub end_offset: usize,
}

/// ## Base value expression
/// 
/// It is the wrapper of the integer, float, string and other base type values.
/// 
/// It also contains the Identifier.
#[derive(Expression, Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub struct BaseValue {
    /// The value of the base value expression
    pub value: String,
    /// The start line of the base value expression
    pub start_line: usize,
    /// The start offset of the base value expression
    pub start_offset: usize,
    /// The end offset of the base value expression
    pub end_offset: usize,
}

/// Wrapped the expressions as an enum
/// 
/// The default is [`ExpressionEnum::BaseValue`]
#[derive(Expression, Clone, Debug, PartialEq, Eq)]
#[derive(Is, Unwrap, Offset, FromExpression, AsRef, TypeIter)]
pub enum ExpressionEnum {
    /// Import expression
    /// 
    /// Learn more from [`Import`]
    Import(Import),
    /// Class expression
    /// 
    /// Learn more from [`Class`]
    Class(Class),
    /// Local variable or function parameter
    /// 
    /// Learn more from [`FastVariable`]
    FastVariable(FastVariable),
    /// Function expression
    /// 
    /// Learn more from [`Function`]
    Function(Function),
    /// Return expression
    /// 
    /// Learn more from [`Return`]
    Return(Return),
    /// Yield expression
    /// 
    /// Learn more from [`Yield`]
    Yield(Yield),
    /// Assign expression
    /// 
    /// Learn more from [`Assign`]
    Assign(Assign),
    /// Alias expression
    /// 
    /// Learn more from [`Alias`]
    Alias(Alias),
    /// Try expression
    /// 
    /// Learn more from [`Try`]
    Try(Try),
    /// Except expression
    /// 
    /// Learn more from [`Except`]
    Except(Except),
    /// Finally expression
    /// 
    /// Learn more from [`Finally`]
    Finally(Finally),
    /// Assertion expression
    /// 
    /// Learn more from [`Assert`]
    Assert(Assert),
    /// Raise expression
    /// 
    /// Learn more from [`Raise`]
    Raise(Raise),
    /// Format value expression
    /// 
    /// Learn more from [`BaseValue`]
    BaseValue(BaseValue),
    /// Format expression
    /// 
    /// Learn more from [`FormatValue`]
    FormatValue(FormatValue),
    /// Binary operation expression
    /// 
    /// Learn more from [`Format`]
    Format(Format),
    /// Binary operation expression
    /// 
    /// Learn more from [`BinaryOperation`]
    BinaryOperation(BinaryOperation),
    /// Subscript expression
    /// 
    /// Learn more from [`Subscr`]
    Subscr(Subscr),
    /// Unary operation expression
    /// 
    /// Learn more from [`UnaryOperation`]
    UnaryOperation(UnaryOperation),
    /// Call expression
    /// 
    /// Learn more from [`Call`]
    Call(Call),
    /// With expression
    /// 
    /// Learn more from [`With`]
    With(With),
    /// For expression
    /// 
    /// Learn more from [`For`]
    For(For),
    /// If expression
    /// 
    /// Learn more from [`If`]
    If(If),
    /// Await expression
    /// 
    /// Learn more from [`Await`]
    Await(Await),
    /// Jump expression
    /// 
    /// Learn more from [`Jump`]
    Jump(Jump),
    /// Container expression
    /// 
    /// Learn more from [`Container`]
    Container(Container),
    /// Attribute expression
    /// 
    /// Learn more from [`Slice`]
    Slice(Slice),
    /// Slice expression
    /// 
    /// Learn more from [`Attribute`]
    Attribute(Attribute),
}

impl Default for ExpressionEnum {
    fn default() -> Self {
        Self::BaseValue(Default::default())
    }
}

/// Defined the unary operation type
/// 
/// The default is [`UnaryType::Positive`]
#[derive(Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub enum UnaryType {
    /// The negative operation
    /// 
    /// It is like `-a`
    Negative,
    /// The invert operation
    /// 
    /// It is like `~a`
    Invert,
    /// The not operation
    /// 
    /// It is like `not a`
    Not,
    /// The positive operation
    /// 
    /// It is like `+a`, but we always ignore it.
    #[default]
    Positive,
}

/// Defined the container type
/// which includes List, Tuple, Set, Dict
/// 
/// And the default is [`ContainerType::Tuple`]
#[derive(Clone, Debug, PartialEq, Eq, Default)]
#[derive(Reflect)]
pub enum ContainerType {
    /// The list container.
    /// 
    /// It is like `[1, 2, 3]`
    List,
    /// The tuple container
    /// 
    /// It is like `(1, (), "a")`
    #[default]
    Tuple,
    /// The set container, it is like HashSet in Rust.
    /// But the value can be any different types
    /// 
    /// It is like `{1, 2, "a"}`
    Set,
    /// The dictionary container, it is like HashMap in Rust.
    /// But the key and value can be any different types
    /// 
    /// It is like `{1: 2, "a": "b", (): []}`
    Dict,
}

// /// `Vec<QueryId>`的封装
// #[derive(Clone, Debug, PartialEq, Eq, Expression)]
// pub struct Expr {
//     pub bodys: Vec<QueryId>,
// }

impl Class {
    /// Create a new class expression from the code object mark
    pub fn new<S: AsRef<str>>(object_mark: S) -> Result<Self> {
        let reg = Regex::new(
            r#"(?x)<code\ object\ (?P<name>\S+)\ at[\S\ ]+\ line\ (?P<start_line>\d+)>"#,
        )?;
        let cap = reg
            .captures(object_mark.as_ref())
            .ok_or(format!("Invalid function mark: {}", object_mark.as_ref()))?;
        let name = cap.name("name").unwrap().as_str().to_string();
        let start_line = cap.name("start_line").unwrap().as_str().parse::<usize>()?;
        Ok(Self {
            mark: object_mark.as_ref().to_string(),
            name,
            members: Vec::new(),
            start_line,
            ..Default::default()
        })
    }
}

impl Function {
    /// Create a new function expression from the code object mark
    pub fn new<S: AsRef<str>>(object_mark: S) -> Result<Self> {
        let reg = Regex::new(
            r#"(?x)<code\ object\ (?P<name>\S+)\ at[\S\ ]+\ line\ (?P<start_line>\d+)>"#,
        )?;
        let cap = reg
            .captures(object_mark.as_ref())
            .ok_or(format!("Invalid function mark: {}", object_mark.as_ref()))?;
        let name = cap.name("name").unwrap().as_str().to_string();
        let start_line = cap.name("start_line").unwrap().as_str().parse::<usize>()?;
        Ok(Self {
            mark: object_mark.as_ref().to_string(),
            name,
            args: Vec::new(),
            defaults: Vec::new(),
            start_line,
            end_line: start_line,
            bodys: Vec::new(),
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod test {
    use bevy_reflect::{GetPath, ReflectKind};

    use super::*;

    #[test]
    fn test_reflect() {
        let test = Import {
            module: "os".to_string(),
            bk_module: None,
            fragment: None,
            alias: None,
            start_line: 1,
            start_offset: 1,
            end_offset: 10,
        };
        
        assert_eq!(test.reflect_kind(), ReflectKind::Struct);
        assert_eq!(test.path::<String>("module"), Ok(&"os".to_string()));
    }
}

// impl Expr {
//     pub fn new() -> Self {
//         Self { bodys: Vec::new() }
//     }

//     pub fn from(bodys: Vec<QueryId>) -> Self {
//         Self { bodys }
//     }

//     pub fn add_expression(&mut self, expr: ExpressionEnum) {
//         self.bodys.push(expr);
//     }

//     pub fn extend(&mut self, expr: Expr) {
//         self.bodys.extend(expr.bodys);
//     }

//     pub fn iter(&self) -> impl Iterator<Item = ExpressionEnum> {
//         self.bodys.clone().into_iter()
//     }
// }

/*
impl ExpressionEnum {
    pub fn build(&self) -> Result<Vec<String>> {
        match self {
            ExpressionEnum::Class(class) => {
                let mut code = Vec::new();
                code.push(format!("class {}:", class.name));

                let mut class_members = class.members.iter();
                let filter_members = ["__module__", "__qualname__"];

                let mut next_expr = class_members.next();
                // expect to skip the __module__ and __qualname__ assignment
                loop {
                    if let Some(ExpressionEnum::Assign(assign)) = next_expr {
                        if let ExpressionEnum::BaseValue(name) = assign.target.as_ref() {
                            if filter_members.contains(&name.value.as_str()) {
                                next_expr = class_members.next();
                            } else {
                                break;
                            }
                        }
                    } else {
                        // unexpected
                        // but no warning
                        break;
                    }
                }

                // check whether the docstring is exist
                let mut has_doc = false;

                if let Some(ExpressionEnum::Assign(assign)) = next_expr {
                    if let ExpressionEnum::BaseValue(name) = assign.target.as_ref() {
                        if name.value == "__doc__" {
                            has_doc = true;
                            let docstring = assign.values.build()?.join("").replace("\\n", "\n");
                            let docstring = docstring.trim_matches('\'');
                            code.push("    \"\"\"".to_string());
                            for line in docstring.lines().filter(|l| !l.trim().is_empty()) {
                                code.push(line.to_string());
                            }
                            code.push("    \"\"\"".to_string());
                            code.push("".to_string());
                        }
                    }
                }
                let re = Regex::new(r"def [A-Za-z_]+\((?P<args>[\S_]*)\)")?;
                let add_self_to_no_arg_func = |line: String| {
                    if let Some(caps) = re.captures(&line) {
                        if let Some(args) = caps.name("args") {
                            if args.is_empty() {
                                line.replace("()", "(self, *args)")
                            } else {
                                line.replace(args.as_str(), &format!("{}, *args", args.as_str()))
                            }
                        } else {
                            line
                        }
                    } else {
                        line
                    }
                };

                if !has_doc {
                    if let Some(expr) = next_expr {
                        let expr_code = expr.build()?;
                        for line in expr_code {
                            code.push(format!("    {}", add_self_to_no_arg_func(line)));
                        }
                        if !code.last().unwrap().trim().is_empty() {
                            code.push("".to_string());
                        }
                    }
                    // if None
                    // it may be an empty class
                    // it doesn't report error
                    else {
                        code.push("    pass".to_string());
                    }
                }

                for expr in class_members {
                    let expr_code = expr.build()?;
                    for line in expr_code {
                        code.push(format!("    {}", add_self_to_no_arg_func(line)));
                    }
                    if !code.last().unwrap().trim().is_empty() {
                        code.push("".to_string());
                    }
                }

                Ok(code)
            }
            ExpressionEnum::Function(function) => {
                let mut code = Vec::new();
                let mut args_code = String::new();
                let mut ret_code = String::new();
                let mut defaults_iter = function.defaults.iter();
                let mut default_offset = function.args.len() - function.defaults.len();
                for (arg, anno) in function.args_iter() {
                    if arg == "return" {
                        ret_code.push_str(&format!(
                            " -> {}",
                            anno.as_ref().unwrap_or(&"None".to_string())
                        ));
                        continue;
                    }
                    if anno.is_none() {
                        args_code.push_str(arg);
                    } else {
                        args_code.push_str(&format!("{}: {}", arg, anno.as_ref().unwrap()));
                    }

                    if default_offset == 0 {
                        args_code.push_str(&format!(
                            " = {}",
                            defaults_iter.next().ok_or("No default! Iter error")?
                        ));
                    } else {
                        // 还不是有默认值的参数
                        default_offset -= 1;
                    }
                    args_code.push_str(", ")
                }
                match function.name.as_str() {
                    "<lambda>" => {
                        let lambda_args = args_code.trim_end_matches(", ");
                        let lambda_body = function
                            .bodys
                            .first()
                            .ok_or("No lambda body")?
                            .build()?
                            .join("");
                        #[cfg(debug_assertions)]
                        {
                            // dbg!(&lambda_body);
                        }
                        let lambda_body = lambda_body.trim_start_matches("return ");
                        if lambda_body.starts_with("yield") {
                            code.push(format!("lambda {}: ({})", lambda_args, lambda_body));
                        } else {
                            code.push(format!("lambda {}: {}", lambda_args, lambda_body));
                        }
                    }
                    "<listcomp>" => {
                        code.push(format!(
                            "[{} for {} in {}]",
                            function.bodys[0].build()?.join(""),
                            args_code.trim_end_matches(", "),
                            function.bodys[1].build()?.join(""),
                        ));
                    }
                    _ => {
                        #[cfg(debug_assertions)]
                        {
                            //dbg!(&args_code);
                        }
                        let first_line = if function.is_async {
                            format!(
                                "async def {}({}){}:",
                                function.name,
                                args_code.trim_end_matches(", "),
                                ret_code
                            )
                        } else {
                            format!(
                                "def {}({}){}:",
                                function.name,
                                args_code.trim_end_matches(", "),
                                ret_code
                            )
                        };
                        code.push(first_line);
                        for expr in function.bodys.iter() {
                            let expr_code = expr.build()?;
                            for line in expr_code.iter() {
                                code.push(format!("    {}", line));
                            }
                        }
                        if code.len() == 1 {
                            code.push("    pass".to_string());
                        }
                        code.push("".to_string());
                    }
                }
                Ok(code)
            }
            ExpressionEnum::FastVariable(fast_var) => {
                if fast_var.name == "None" {
                    Ok(vec!["".to_string()])
                } else if fast_var.name == "0" {
                    Ok(vec![])
                } else {
                    Ok(vec![fast_var.name.clone()])
                }
            }
            ExpressionEnum::Return(r) => {
                let value_code = r.value.build()?.join("");
                if value_code.is_empty() {
                    Ok(vec![])
                } else {
                    Ok(vec![format!("return {}", value_code)])
                }
            }
            ExpressionEnum::Yield(y) => {
                let value_code = y.value.build()?.join("");
                if value_code.is_empty() {
                    Ok(vec!["yield".to_string()])
                } else {
                    Ok(vec![format!("yield {}", value_code)])
                }
            }
            ExpressionEnum::Assign(a) => {
                let mut code = Vec::new();
                let target_code = a.target.build()?;
                let value_code = a.values.build()?;
                code.push(format!(
                    "{} {} {}",
                    target_code.join(""),
                    a.operator,
                    value_code.join("")
                ));
                Ok(code)
            }
            ExpressionEnum::Alias(alias) => {
                let target_code = alias.target.build()?.join("");
                let alias_code = alias.alias.build()?.join("");
                Ok(vec![format!("{} as {}", target_code, alias_code)])
            }
            ExpressionEnum::Try(try_expr) => {
                let mut code = Vec::new();
                for expr in try_expr.body.iter() {
                    let expr_code = expr.build()?;
                    for line in expr_code.iter() {
                        code.push(format!("    {}", line));
                    }
                }
                for expr in try_expr.except.iter() {
                    let expr_code = expr.build()?;
                    code.extend(expr_code);
                }
                code.extend(try_expr.finally.build()?);
                Ok(code)
            }
            ExpressionEnum::Except(except) => {
                let exception_code = except.exception.build()?.join("");
                let mut code = Vec::new();
                if exception_code.is_empty() {
                    code.push("except:".to_string());
                } else {
                    code.push(format!("except {}:", exception_code));
                }
                for expr in except.body.iter() {
                    let expr_code = expr.build()?;
                    for line in expr_code.iter() {
                        code.push(format!("    {}", line));
                    }
                }
                Ok(code)
            }
            ExpressionEnum::Finally(finally) => {
                let mut code = Vec::new();
                code.push("finally:".to_string());
                for expr in finally.body.iter() {
                    let expr_code = expr.build()?;
                    for line in expr_code.iter() {
                        code.push(format!("    {}", line));
                    }
                }
                Ok(code)
            }
            ExpressionEnum::Assert(assert) => {
                let test_code = assert.test.build()?.join("");
                match &assert.msg {
                    Some(msg) => {
                        let msg_code = msg.build()?.join("");
                        Ok(vec![format!("assert {}, {}", test_code, msg_code)])
                    }
                    None => Ok(vec![test_code]),
                }
            }
            ExpressionEnum::Raise(raise) => {
                let exception_code = raise.exception.build()?.join("");
                Ok(vec![format!("raise {}", exception_code)])
            }
            ExpressionEnum::Await(await_expr) => {
                let awaitable_code = await_expr.awaitable_expr.build()?.join("");
                Ok(vec![format!("await {}", awaitable_code)])
            }
            ExpressionEnum::BaseValue(base_value) => {
                if base_value.value == "None" {
                    Ok(vec!["".to_string()])
                }
                /* else if base_value.value == "0" {
                    Ok(vec![])
                } */
                else {
                    Ok(vec![base_value.value.clone()])
                }
            }
            ExpressionEnum::Call(call) => {
                let func_code = call.func.build()?.join("");
                let mut args_code = Vec::new();
                for arg in call.args.iter() {
                    let arg_code = arg.build()?;
                    args_code.push(arg_code.join(""));
                }
                if func_code.starts_with("lambda ") {
                    Ok(vec![format!(
                        "({})({})",
                        func_code,
                        args_code.join(", ").trim_end_matches(", ")
                    )])
                } else {
                    Ok(vec![format!(
                        "{}({})",
                        func_code,
                        args_code.join(", ").trim_end_matches(", ")
                    )])
                }
            }
            ExpressionEnum::FormatValue(format_value) => {
                let value_code = format_value.value.build()?.join("");
                Ok(vec![value_code])
            }
            ExpressionEnum::Format(format) => {
                let mut code = Vec::new();
                let mut format_string = String::new();
                for value in format.format_values.iter() {
                    let value_code = value.build()?;
                    if value.is_format_value() {
                        format_string.push_str(&format!("{{{}}}", value_code.join("")));
                    } else {
                        format_string.push_str(value_code.join("").trim_matches('\''));
                    }
                }
                code.push(format!("f\"{}\"", format_string.replace('"', "\\\"")));
                Ok(code)
            }
            ExpressionEnum::BinaryOperation(binary_operation) => Ok(vec![format!(
                "{} {} {}",
                binary_operation.left.build()?.join(""),
                binary_operation.operator,
                binary_operation.right.build()?.join("")
            )]),
            ExpressionEnum::UnaryOperation(unary_operation) => Ok(vec![format!(
                "{}{}",
                match unary_operation.unary_type {
                    UnaryType::Negative => "-",
                    UnaryType::Invert => "~",
                    UnaryType::Not => "not ",
                    UnaryType::Positive => unreachable!(),
                },
                unary_operation.target.build()?.join("")
            )]),
            ExpressionEnum::Import(import) => {
                if import.bk_module.is_none() {
                    //没from
                    if import.alias.is_none() {
                        //没from，没as
                        Ok(vec![format!("import {}", import.module)])
                    } else {
                        //没from，有as
                        Ok(vec![format!(
                            "import {} as {}",
                            import.module,
                            import
                                .alias
                                .as_ref()
                                .expect("[No from Have as] Alias missed")
                                .trim_end_matches(", ")
                        )])
                    }
                } else {
                    //有from

                    Ok(vec![format!(
                        "from {} import {}",
                        import.module,
                        import
                            .bk_module
                            .as_ref()
                            .expect("[Have from No as] Bk_module missed")
                            .trim_end_matches(", ")
                    )])
                }
            }
            ExpressionEnum::Container(container) => {
                let mut code = Vec::new();
                let mut values_code = Vec::new();
                for value in container.values.iter() {
                    let mut value_code = value.build()?;
                    values_code.append(&mut value_code);
                }
                values_code.iter_mut().for_each(|s| {
                    if s.is_empty() {
                        *s = "None".to_string();
                    }
                });
                match container.container_type {
                    ContainerType::List => {
                        code.push(format!("[{}]", values_code.join(", ")));
                    }
                    ContainerType::Tuple => {
                        code.push(format!("({})", values_code.join(", ")));
                    }
                    ContainerType::Set => {
                        code.push(format!("{{ {} }}", values_code.join(", ")));
                    }
                    ContainerType::Dict => {
                        let mut dict_code = Vec::new();
                        for (i, value) in values_code.iter().enumerate() {
                            if i % 2 == 0 {
                                dict_code.push(format!("{}: {}", value, values_code[i + 1]));
                            }
                        }
                        code.push(format!("{{ {} }}", dict_code.join(", ")));
                    }
                }
                Ok(code)
            }
            ExpressionEnum::Subscr(subscr) => {
                let index_code = subscr.index.build()?.join("");
                let target_code = subscr.target.build()?.join("");
                Ok(vec![format!("{}[{}]", target_code, index_code)])
            }
            ExpressionEnum::Slice(slice) => {
                let origin_code = slice.origin.build()?;
                let slice_code = slice
                    .slice
                    .iter()
                    .map(|s| Ok(s.build()?.join("")))
                    .collect::<Result<Vec<String>>>()?;
                Ok(vec![format!(
                    "{}[{}]",
                    origin_code.join(""),
                    slice_code.join(":")
                )])
            }
            ExpressionEnum::Attribute(attribute) => Ok(vec![format!(
                "{}.{}",
                attribute.parent.build()?.join(""),
                attribute.attr.build()?.join("")
            )]),
            ExpressionEnum::With(with) => {
                let item_code = with.item.build()?.join("");
                let mut code = Vec::new();
                let first_line = if with.is_async {
                    format!("async with {}:", item_code)
                } else {
                    format!("with {}:", item_code)
                };
                code.push(first_line);
                if with.body.is_empty() {
                    code.push("    pass".to_string());
                } else {
                    for expr in with.body.iter() {
                        let expr_code = expr.build()?;
                        for line in expr_code.iter() {
                            code.push(format!("    {}", line));
                        }
                    }
                }
                Ok(code)
            }
            ExpressionEnum::If(if_else) => {
                let mut code = Vec::new();
                if let Some(test) = if_else.test.as_ref() {
                    let test_code = test.build()?.join("");
                    code.push(format!("if {}:", test_code));
                } else {
                    code.push("else:".to_string());
                }

                for expr in if_else.body.iter() {
                    if let ExpressionEnum::Jump(jump) = expr {
                        if jump.is_backward {
                            code.push("    continue".to_string());
                        }
                    } else {
                        let expr_code = expr.build()?;
                        for line in expr_code.iter() {
                            code.push(format!("    {}", line));
                        }
                    }
                }

                if let Some(or_else) = if_else.or_else.as_ref() {
                    let or_else_code = or_else.build()?;
                    // dbg!(&or_else_code);
                    if or_else_code[0].starts_with("if ") {
                        // elif
                        code.push(format!("el{}", or_else_code[0]));
                        code.extend(or_else_code.into_iter().skip(1));
                    } else {
                        // starts with "else:"
                        code.extend(or_else_code);
                    }
                }

                Ok(code)
            }
            ExpressionEnum::For(for_expr) => {
                let iter_code = for_expr.iterator.build()?.join("");
                let item_code = for_expr.items.build()?.join("");
                let mut code = Vec::new();
                let first_line = if for_expr.is_async {
                    format!("async for {} in {}:", item_code, iter_code)
                } else {
                    format!("for {} in {}:", item_code, iter_code)
                };
                code.push(first_line);
                if for_expr.body.is_empty() {
                    code.push("    pass".to_string());
                } else {
                    for expr in for_expr.body.iter() {
                        let expr_code = expr.build()?;
                        for line in expr_code.iter() {
                            code.push(format!("    {}", line));
                        }
                    }
                }
                Ok(code)
            }
            _ => Ok(vec![]),
        }
    }
}
*/
