use std::any::{Any, TypeId};

pub use pyrev_ast_derive::*;
use regex::Regex;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Expression
where
    Self: Clone + std::fmt::Debug + PartialEq + Eq,
{
    fn build_code(&self) -> Vec<(usize, String)>;
}

pub trait Queryable {
    fn as_any(&self) -> &dyn Any;
    fn try_query<T: 'static>(&self) -> Option<&T> {
        if TypeId::of::<T>() == self.as_any().type_id() {
            unsafe { Some(&*(self.as_any() as *const dyn Any as *const T)) }
        } else {
            None
        }
    }
}

impl<T: 'static> Queryable for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait Query {
    fn query<T: std::fmt::Debug + 'static>(&self) -> Vec<&T>;
}

#[derive(Expression, Clone, Debug, PartialEq, Eq, Query)]
pub struct Function {
    pub mark: String,
    pub name: String,
    pub args: Vec<String>,
    pub args_annotation: Vec<String>,
    pub start_line: usize,
    pub end_line: usize,
    pub bodys: Vec<ExpressionEnum>,
}

#[derive(Expression, Clone, Debug, PartialEq, Eq, Query)]
pub struct Assign {
    pub target: Box<ExpressionEnum>,
    pub values: Box<ExpressionEnum>,
}

#[derive(Expression, Clone, Debug, PartialEq, Eq, Query)]
pub struct BinaryOperation {
    pub left: Box<ExpressionEnum>,
    pub right: Box<ExpressionEnum>,
    pub operator: String,
}

#[derive(Expression, Clone, Debug, PartialEq, Eq, Query)]
pub struct Call {
    pub func: Box<ExpressionEnum>,
    pub args: Vec<ExpressionEnum>,
}

#[derive(Expression, Clone, Debug, PartialEq, Eq, Query)]
pub struct Container {
    pub values: Vec<ExpressionEnum>,
    pub container_type: ContainerType,
}

#[derive(Expression, Clone, Debug, PartialEq, Eq, Query)]
pub struct Attribute {
    pub parent: Box<ExpressionEnum>,
    pub attr: Box<ExpressionEnum>,
}

// String的Expression封装
#[derive(Expression, Clone, Debug, PartialEq, Eq, Query)]
pub struct BaseValue {
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpressionEnum {
    Function(Function),
    Assign(Assign),
    BaseValue(BaseValue),
    BinaryOperation(BinaryOperation),
    Call(Call),
    Container(Container),
    Attribute(Attribute),
    // ...
}

impl Query for ExpressionEnum {
    fn query<T: std::fmt::Debug + 'static>(&self) -> Vec<&T> {
        match self {
            ExpressionEnum::Function(f) => f.query(),
            ExpressionEnum::Assign(a) => a.query(),
            ExpressionEnum::BaseValue(b) => b.query(),
            ExpressionEnum::BinaryOperation(b) => b.query(),
            ExpressionEnum::Call(c) => c.query(),
            ExpressionEnum::Container(c) => c.query(),
            ExpressionEnum::Attribute(a) => a.query(),
        }
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

impl Query for String {
    fn query<U: 'static>(&self) -> Vec<&U> {
        vec![]
    }
}

impl Query for usize {
    fn query<U: 'static>(&self) -> Vec<&U> {
        vec![]
    }
}

impl<T: Query> Query for Vec<T> {
    fn query<U: std::fmt::Debug + 'static>(&self) -> Vec<&U> {
        let mut result = Vec::new();
        for item in self.iter() {
            result.extend(item.query::<U>());
        }
        result
    }
}

impl<T: Query> Query for Box<T> {
    fn query<U: std::fmt::Debug + 'static>(&self) -> Vec<&U> {
        self.as_ref().query::<U>()
    }
}

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
            args_annotation: Vec::new(),
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

impl ExpressionEnum {
    pub fn build(&self) -> Result<Vec<String>> {
        match self {
            ExpressionEnum::Function(function) => {
                let mut code = Vec::new();
                code.push(format!("def {}():", function.name));
                for expr in function.bodys.iter() {
                    let mut sub_code = expr.build()?;
                    code.append(&mut sub_code);
                }
                Ok(code)
            }
            ExpressionEnum::Assign(a) => {
                let mut code = Vec::new();
                let target_code = a.target.build()?;
                let value_code = a.values.build()?;
                if !value_code.first().unwrap().trim().starts_with("def") {
                    code.push(format!(
                        "{} = {}",
                        target_code.join(""),
                        value_code.join("")
                    ));
                } else {
                    code.push(value_code.join(""));
                }
                Ok(code)
            }
            ExpressionEnum::BaseValue(base_value) => Ok(vec![base_value.value.clone()]),
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
            ExpressionEnum::Container(container) => {
                let mut code = Vec::new();
                let mut values_code = Vec::new();
                for value in container.values.iter() {
                    let mut value_code = value.build()?;
                    values_code.append(&mut value_code);
                }
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
            ExpressionEnum::Attribute(attribute) => Ok(vec![format!(
                "{}.{}",
                attribute.parent.build()?.join(""),
                attribute.attr.build()?.join("")
            )]),
        }
    }
}
