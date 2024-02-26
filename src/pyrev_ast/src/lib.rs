pub use pyrev_ast_derive::*;
use regex::Regex;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Expression
where
    Self: Clone + std::fmt::Debug + PartialEq + Eq,
{
    fn build_code(&self) -> Vec<(usize, String)>;
    fn query<S, U>(&self, field_name: S) -> Option<&U>
    where
        S: AsRef<str>,
        U: ?Sized;
}

#[derive(Expression, Clone, Debug, PartialEq, Eq)]
pub struct Function {
    pub mark: String,
    pub name: String,
    pub args: Vec<String>,
    pub args_annotation: Vec<String>,
    pub start_line: usize,
    pub end_line: usize,
    pub bodys: Vec<ExpressionEnum>,
}

#[derive(Expression, Clone, Debug, PartialEq, Eq)]
pub struct Assign {
    pub name: String,
    pub values: Box<ExpressionEnum>,
}

#[derive(Expression, Clone, Debug, PartialEq, Eq)]
pub struct BinaryOperation {
    pub left: Box<ExpressionEnum>,
    pub right: Box<ExpressionEnum>,
    pub operator: String,
}

#[derive(Expression, Clone, Debug, PartialEq, Eq)]
pub struct Call {
    pub func: Box<ExpressionEnum>,
    pub args: Vec<ExpressionEnum>,
}

#[derive(Expression, Clone, Debug, PartialEq, Eq)]
pub struct Container {
    pub values: Vec<ExpressionEnum>,
    pub container_type: ContainerType,
}

// String的Expression封装
#[derive(Expression, Clone, Debug, PartialEq, Eq)]
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
    // ...
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContainerType {
    List,
    Tuple,
    Set,
    Dict,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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