#![feature(concat_idents)]

mod query;
mod querymutable;

pub use pyrev_ast_derive::*;
pub use query::*;
pub use querymutable::*;
use regex::Regex;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Expression trait is used to mark the struct as an expression
pub trait Expression {}

/// 导入
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query)]
pub struct Import {
    pub module: String,
    pub bk_module: Option<String>,
    pub fragment: Option<String>,
    pub alias: Option<String>,
}

#[derive(Expression, Clone, Debug, PartialEq, Eq, Query)]
pub struct FastVariable {
    pub index: usize,
    pub name: String,
    pub annotation: Option<String>,
}

/// 函数
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query)]
pub struct Function {
    pub mark: String,
    pub name: String,
    pub args: Vec<FastVariable>,
    pub start_line: usize,
    pub end_line: usize,
    pub bodys: Vec<ExpressionEnum>,
}

/// 返回
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query)]
pub struct Return {
    pub value: Box<ExpressionEnum>,
}

/// 赋值
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query)]
pub struct Assign {
    pub target: Box<ExpressionEnum>,
    pub values: Box<ExpressionEnum>,
    pub operator: String,
}

/// Alias, like Assign but only for `as`
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query)]
pub struct Alias {
    pub target: Box<ExpressionEnum>,
    pub alias: Box<ExpressionEnum>,
}

/// Try
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query)]
pub struct Try {
    pub body: Vec<ExpressionEnum>,
    /// this is the exception which will be caught
    pub except: Vec<ExpressionEnum>,
    pub finally: Box<ExpressionEnum>,
}

/// Except
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query)]
pub struct Except {
    pub exception: Box<ExpressionEnum>,
    pub body: Vec<ExpressionEnum>,
}

/// finally
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query)]
pub struct Finally {
    pub body: Vec<ExpressionEnum>,
}

/// 断言
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query)]
pub struct Assert {
    pub test: Box<ExpressionEnum>,
    pub msg: Option<Box<ExpressionEnum>>,
}

/// 抛出异常
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query)]
pub struct Raise {
    pub exception: Box<ExpressionEnum>,
}

/// 二元操作
/// 包括 +, -, *, /, <<, %, ==, >, is, in等
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query)]
pub struct BinaryOperation {
    pub left: Box<ExpressionEnum>,
    pub right: Box<ExpressionEnum>,
    pub operator: String,
}

#[derive(Expression, Clone, Debug, PartialEq, Eq, Query)]
pub struct UnaryOperation {
    pub target: Box<ExpressionEnum>,
    pub unary_type: UnaryType,
}
/// 函数调用
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query)]
pub struct Call {
    pub func: Box<ExpressionEnum>,
    pub args: Vec<ExpressionEnum>,
}

/// If expression
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query)]
pub struct If {
    pub test: Box<ExpressionEnum>,
    pub body: Vec<ExpressionEnum>,
    pub or_else: Vec<ExpressionEnum>,
}

/// Jump
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query)]
pub struct Jump {
    pub target: usize,
    pub body: Vec<ExpressionEnum>,
}

/// 容器(包括list, tuple, set, dict等)
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query)]
pub struct Container {
    pub values: Vec<ExpressionEnum>,
    pub container_type: ContainerType,
}

/// 属性
/// 例如: a.b
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query)]
pub struct Attribute {
    pub parent: Box<ExpressionEnum>,
    pub attr: Box<ExpressionEnum>,
}

/// 切片
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query)]
pub struct Slice {
    pub origin: Box<ExpressionEnum>,
    pub slice: Vec<ExpressionEnum>,
}

/// String的Expression封装
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query)]
pub struct BaseValue {
    pub value: String,
}

/// 为上面的表达式提供一个封装
/// 用来实现不同Expression的嵌套
///
/// is_xxx function example:
/// ```
/// use pyrev_ast::*;
/// let expr = ExpressionEnum::BaseValue(BaseValue { value: "None".to_string() });
/// assert!(expr.is_base_value());
/// let expr = ExpressionEnum::Assign(Assign {
///     target: Box::new(ExpressionEnum::BaseValue(BaseValue { value: "a".to_string() })),
///     values: Box::new(ExpressionEnum::BaseValue(BaseValue { value: "1".to_string() })),
///     operator: "=".to_string(),
/// });
/// assert!(expr.is_assign());
/// ```
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Common)]
pub enum ExpressionEnum {
    Import(Import),
    FastVariable(FastVariable),
    Function(Function),
    Return(Return),
    Assign(Assign),
    Alias(Alias),
    Try(Try),
    Except(Except),
    Finally(Finally),
    Assert(Assert),
    Raise(Raise),
    BaseValue(BaseValue),
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
    Call(Call),
    If(If),
    Jump(Jump),
    Container(Container),
    Slice(Slice),
    Attribute(Attribute),
    // ...
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UnaryType {
    Negative,
    Invert,
    Not,
}

impl Query for UnaryType {
    fn query<T: 'static>(&self) -> Vec<&T> {
        vec![]
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContainerType {
    List,
    Tuple,
    Set,
    Dict,
}

impl Query for ContainerType {
    fn query<T: 'static>(&self) -> Vec<&T> {
        vec![]
    }
}

/// 只是对外提供一个ExpressionEnum的封装 (单纯不想使用`Vec<ExpressionEnum>`而已 )
#[derive(Clone, Debug, PartialEq, Eq, Query)]
pub struct Expr {
    pub bodys: Vec<ExpressionEnum>,
}

impl Function {
    pub fn new<S: AsRef<str>>(object_mark: S) -> Result<Self> {
        let reg =
            Regex::new(r"(?x)code\ object\ (?P<name>\S+)\ at[\S\ ]+\ line\ (?P<start_line>\d+)>")?;
        let cap = reg.captures(object_mark.as_ref()).unwrap();
        let name = cap.name("name").unwrap().as_str().to_string();
        let start_line = cap.name("start_line").unwrap().as_str().parse::<usize>()?;
        Ok(Self {
            mark: object_mark.as_ref().to_string(),
            name,
            args: Vec::new(),
            start_line,
            end_line: start_line,
            bodys: Vec::new(),
        })
    }

    pub fn from(expr: ExpressionEnum) -> Result<Self> {
        if let ExpressionEnum::BaseValue(value) = expr {
            Self::new(value.value)
        } else {
            Err("Function name must be a string".into())
        }
    }

    pub fn args_iter(&self) -> impl Iterator<Item = (&String, &Option<String>)> {
        let mut iter = self
            .args
            .iter()
            .map(|arg| (&arg.index, &arg.name, &arg.annotation))
            .collect::<Vec<_>>();
        iter.sort_by(|i, j| i.0.cmp(j.0));
        iter.into_iter().map(|(_, name, anno)| (name, anno))
    }
}

impl Default for Expr {
    fn default() -> Self {
        Self::new()
    }
}

impl Expr {
    pub fn new() -> Self {
        Self { bodys: Vec::new() }
    }

    pub fn add_expression(&mut self, expr: ExpressionEnum) {
        self.bodys.push(expr);
    }

    pub fn extend(&mut self, expr: Expr) {
        self.bodys.extend(expr.bodys);
    }
}

/// 递归遍历表达式树, 生成代码
impl ExpressionEnum {
    pub fn build(&self) -> Result<Vec<String>> {
        match self {
            ExpressionEnum::Function(function) => {
                let mut code = Vec::new();
                let mut args_code = String::new();
                let mut ret_code = String::new();
                for (arg, anno) in function.args_iter() {
                    if arg == "return" {
                        ret_code.push_str(&format!(
                            " -> {}",
                            anno.as_ref().unwrap_or(&"None".to_string())
                        ));
                        continue;
                    }
                    if anno.is_none() {
                        args_code.push_str(&format!("{}, ", arg));
                    } else {
                        args_code.push_str(&format!("{}: {}, ", arg, anno.as_ref().unwrap()));
                    }
                }
                match function.name.as_str() {
                    "<lambda>" => {
                        code.push(format!(
                            "lambda {}: {}",
                            args_code.trim_end_matches(", "),
                            function.bodys[0]
                                .build()?
                                .join("")
                                .trim_start_matches("return "),
                        ));
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
                        code.push(format!(
                            "def {}({}){}:",
                            function.name,
                            args_code.trim_end_matches(", "),
                            ret_code
                        ));
                        for expr in function.bodys.iter() {
                            let expr_code = expr.build()?;
                            for line in expr_code.iter() {
                                code.push(format!("    {}", line));
                            }
                        }
                        if code.len() == 1 {
                            code.push("    pass".to_string());
                        }
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
            ExpressionEnum::BaseValue(base_value) => {
                if base_value.value == "None" {
                    Ok(vec!["".to_string()])
                } else if base_value.value == "0" {
                    Ok(vec![])
                } else {
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
                Ok(vec![format!(
                    "{}({})",
                    func_code,
                    args_code.join(", ").trim_end_matches(", ")
                )])
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
                },
                unary_operation.target.build()?.join("")
            )]),
            ExpressionEnum::Import(import) => {
                if import.bk_module == None {
                    //没from
                    if import.alias == None {
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
            _ => Ok(vec![]),
        }
    }
}
