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
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct Import {
    pub module: String,
    pub bk_module: Option<String>,
    pub fragment: Option<String>,
    pub alias: Option<String>,
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// 类
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct Class {
    pub mark: String,
    pub name: String,
    pub members: Vec<ExpressionEnum>,
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// 局部变量
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct FastVariable {
    pub index: usize,
    pub name: String,
    pub annotation: Option<String>,
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// 函数
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct Function {
    pub mark: String,
    pub name: String,
    pub args: Vec<FastVariable>,
    pub bodys: Vec<ExpressionEnum>,
    pub defaults: Vec<String>,
    pub is_async: bool,
    pub start_line: usize,
    pub end_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// 返回
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct Return {
    pub value: Box<ExpressionEnum>,
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct Yield {
    pub value: Box<ExpressionEnum>,
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// 赋值
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct Assign {
    pub target: Box<ExpressionEnum>,
    pub values: Box<ExpressionEnum>,
    pub operator: String,
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// Alias, like Assign but only for `as`
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct Alias {
    pub target: Box<ExpressionEnum>,
    pub alias: Box<ExpressionEnum>,
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// Try
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct Try {
    pub body: Vec<ExpressionEnum>,
    /// this is the exception which will be caught
    pub except: Vec<ExpressionEnum>,
    pub finally: Box<ExpressionEnum>,
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// Except
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct Except {
    pub exception: Box<ExpressionEnum>,
    pub body: Vec<ExpressionEnum>,
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// finally
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct Finally {
    pub body: Vec<ExpressionEnum>,
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// 断言
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct Assert {
    pub test: Box<ExpressionEnum>,
    pub msg: Option<Box<ExpressionEnum>>,
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// 抛出异常
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct Raise {
    pub exception: Box<ExpressionEnum>,
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct FormatValue {
    pub value: Box<ExpressionEnum>,
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// 格式化字符串
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct Format {
    pub format_values: Vec<ExpressionEnum>,
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// 二元操作
/// 包括 +, -, *, /, <<, %, ==, >, is, in等
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct BinaryOperation {
    pub left: Box<ExpressionEnum>,
    pub right: Box<ExpressionEnum>,
    pub operator: String,
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// 下标
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct Subscr {
    pub index: Box<ExpressionEnum>,
    pub target: Box<ExpressionEnum>,
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// 一元操作
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct UnaryOperation {
    pub target: Box<ExpressionEnum>,
    pub unary_type: UnaryType,
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}
/// 函数调用
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct Call {
    pub func: Box<ExpressionEnum>,
    pub args: Vec<ExpressionEnum>,
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct With {
    pub item: Box<ExpressionEnum>,
    pub body: Vec<ExpressionEnum>,
    pub is_async: bool,
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// For循环
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct For {
    pub iterator: Box<ExpressionEnum>,
    pub items: Box<ExpressionEnum>,
    pub body: Vec<ExpressionEnum>,
    pub from: usize,
    pub to: usize,
    pub is_async: bool,
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// If expression
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct If {
    pub test: Option<Box<ExpressionEnum>>,
    pub body: Vec<ExpressionEnum>,
    pub or_else: Option<Box<ExpressionEnum>>,
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// Jump
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct Jump {
    pub target: usize,
    pub body: Vec<ExpressionEnum>,
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// Await
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct Await {
    pub awaitable_expr: Box<ExpressionEnum>,
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// 容器(包括list, tuple, set, dict等)
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct Container {
    pub values: Vec<ExpressionEnum>,
    pub container_type: ContainerType,
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// 属性
/// 例如: a.b
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct Attribute {
    pub parent: Box<ExpressionEnum>,
    pub attr: Box<ExpressionEnum>,
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// 切片
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct Slice {
    pub origin: Box<ExpressionEnum>,
    pub slice: Vec<ExpressionEnum>,
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// String的Expression封装
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct BaseValue {
    pub value: String,
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// Deprecated
/* /// None
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
pub struct NoneValue {
    pub start_line: usize,
    pub start_offset: usize,
    pub end_offset: usize,
} */

/// 为上面的表达式提供一个封装
/// 用来实现不同Expression的嵌套
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Is, Unwrap, GetOffset)]
pub enum ExpressionEnum {
    /// NoneValue is deprecated
    // NoneValue(NoneValue),
    Import(Import),
    Class(Class),
    FastVariable(FastVariable),
    Function(Function),
    Return(Return),
    Yield(Yield),
    Assign(Assign),
    Alias(Alias),
    Try(Try),
    Except(Except),
    Finally(Finally),
    Assert(Assert),
    Raise(Raise),
    BaseValue(BaseValue),
    FormatValue(FormatValue),
    Format(Format),
    BinaryOperation(BinaryOperation),
    Subscr(Subscr),
    UnaryOperation(UnaryOperation),
    Call(Call),
    With(With),
    For(For),
    If(If),
    Await(Await),
    Jump(Jump),
    Container(Container),
    Slice(Slice),
    Attribute(Attribute),
    // ...
}

impl Default for ExpressionEnum {
    fn default() -> Self {
        Self::BaseValue(Default::default())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum UnaryType {
    Negative,
    Invert,
    Not,
    #[default]
    Positive,
}

impl Query for UnaryType {
    fn query<T: 'static>(&self) -> Vec<&T> {
        vec![]
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum ContainerType {
    List,
    #[default]
    Tuple,
    Set,
    Dict,
}

impl Query for ContainerType {
    fn query<T: 'static>(&self) -> Vec<&T> {
        vec![]
    }
}

/// `Vec<ExpressionEnum>`的封装
#[derive(Clone, Debug, PartialEq, Eq, Query, Expression)]
pub struct Expr {
    pub bodys: Vec<ExpressionEnum>,
}

impl Class {
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

    pub fn from(expr: ExpressionEnum) -> Result<Self> {
        if let ExpressionEnum::BaseValue(value) = expr {
            Self::new(value.value)
        } else {
            Err(format!("Expect BaseValue, got {:?}", expr).into())
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

    pub fn from(bodys: Vec<ExpressionEnum>) -> Self {
        Self { bodys }
    }

    pub fn add_expression(&mut self, expr: ExpressionEnum) {
        self.bodys.push(expr);
    }

    pub fn extend(&mut self, expr: Expr) {
        self.bodys.extend(expr.bodys);
    }

    pub fn iter(&self) -> impl Iterator<Item = ExpressionEnum> {
        self.bodys.clone().into_iter()
    }
}

/// 递归遍历表达式树, 生成代码
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
                    let expr_code = expr.build()?;
                    for line in expr_code.iter() {
                        code.push(format!("    {}", line));
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
