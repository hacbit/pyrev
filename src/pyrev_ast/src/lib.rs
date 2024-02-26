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
    //fn children_expression(&self) -> Vec<&ExpressionEnum>;
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
    pub target: Box<ExpressionEnum>,
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

#[derive(Expression, Clone, Debug, PartialEq, Eq)]
pub struct Attribute {
    pub parent: Box<ExpressionEnum>,
    pub attr: Box<ExpressionEnum>,
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
    Attribute(Attribute),
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
                    code.push(format!("{} = {}", target_code.join(""), value_code.join("")));
                } else {
                    code.push(value_code.join(""));
                }
                Ok(code)
            }
            ExpressionEnum::BaseValue(base_value) => {
                Ok(vec![base_value.value.clone()])
            }
            ExpressionEnum::Call(call) => {
                let mut code = Vec::new();
                let mut func_code = call.func.build()?;
                code.append(&mut func_code);
                for arg in call.args.iter() {
                    let mut arg_code = arg.build()?;
                    code.append(&mut arg_code);
                }
                Ok(code)
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