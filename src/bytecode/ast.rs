use std::default;

use regex::Regex;

use super::{
    opcode::{Opcode, OpcodeInstruction},
    parse_opcode::CodeObject,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Function {
    pub mark: String,
    pub name: String,
    pub args: Vec<String>,
    pub args_annotation: Vec<String>,
    pub start_line: usize,
    pub end_line: usize,
    pub child: Vec<Expr>,
}

#[derive(Debug)]
pub struct Assign {
    pub name: String,
    pub value: Expr,
}

#[derive(Debug)]
pub struct Expr {
    pub expr_type: ExprType,
    pub values: Vec<String>,
    pub child: Vec<Expr>,
    pub func: Option<Function>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum ExprType {
    #[default]
    Expr,
    Function,
    Assign,
    // ...
}

impl Function {
    fn new<S: AsRef<str>>(object_mark: S) -> Result<Self> {
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
            child: Vec::new(),
        })
    }

    fn build_code(&self) -> Vec<(usize, String)> {
        let mut code = Vec::new();
        if self.name == "<lambda>" {
            code.push((
                self.start_line,
                format!("lambda {}: ...", self.args.join(", ")),
            ))
        } else if self.name == "<listcomp>" {
            code.push((
                self.start_line,
                format!(
                    "[{} for {} in {}]",
                    self.args[0], self.args[1], self.args[2]
                ),
            ))
        } else if self.name == "<dictcomp>" {
            code.push((
                self.start_line,
                format!(
                    "{{{}: {} for {} in {}}}",
                    self.args[0], self.args[1], self.args[2], self.args[3]
                ),
            ))
        } else if self.name == "<setcomp>" {
            code.push((
                self.start_line,
                format!(
                    "{{{} for {} in {}}}",
                    self.args[0], self.args[1], self.args[2]
                ),
            ))
        } else {
            code.push((
                self.start_line,
                format!("def {}({}):", self.name, self.args.join(", ")),
            ))
        }
        code
    }
}

impl Expr {
    fn new(opcode_instructions: Vec<OpcodeInstruction>) -> Result<Self> {
        let start_line = opcode_instructions
            .first()
            .ok_or("Not have first")?
            .starts_line
            .ok_or("Not have starts_line")?;
        let mut stack = Vec::new();
        let mut child = Vec::new();
        let mut values = Vec::new();
        let mut expr_type = ExprType::default();
        let mut func: Option<Function> = None;
        for instruction in opcode_instructions {
            match instruction.opcode {
                Opcode::None => {}
                Opcode::Nop => {}
                Opcode::LoadConst | Opcode::LoadName | Opcode::LoadFast | Opcode::LoadGlobal => {
                    stack.push(instruction.argval.ok_or("No argval")?.clone());
                }
                Opcode::StoreName => {
                    let name = instruction.argval.ok_or("No argval")?.clone();
                    let value = stack.pop().ok_or("Stack is empty")?;
                    if let Some(f) = func.as_ref() {
                        if f.name == name {
                            continue;
                        }
                    }
                }
                Opcode::BuildTuple => {
                    let size = instruction.arg.ok_or("No arg")?;
                    let mut tuple = Vec::with_capacity(size);
                    for _ in 0..size {
                        tuple.push(stack.pop().ok_or("Stack is empty")?);
                    }
                    stack.push(format!("({})", tuple.join(", ")));
                }
                Opcode::MakeFunction => {
                    let mark = stack.pop().ok_or("Stack is empty")?;
                    let mut function = Function::new(mark)?;
                    if instruction.argval == Some("annotations".to_string()) {
                        let values = stack
                            .pop()
                            .ok_or("Stack is empty")?
                            .split(", ")
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>();
                        // 比如 ('a', int, 'result', int)
                        for i in (0..instruction.arg.ok_or("No arg")?).step_by(2) {
                            function.args.push(
                                values
                                    .get(i)
                                    .ok_or("No arg")?
                                    .trim_start_matches('\'')
                                    .trim_end_matches('\'')
                                    .to_string(),
                            );
                            function
                                .args_annotation
                                .push(values.get(i + 1).ok_or("No arg annotation")?.to_string());
                        }
                    }
                    expr_type = ExprType::Function;
                    func = Some(function);
                }
                Opcode::BinaryOp => {
                    let right = stack.pop().ok_or("Stack is empty")?;
                    let left = stack.pop().ok_or("Stack is empty")?;
                    stack.push(format!(
                        "{} {} {}",
                        left,
                        instruction.argval.ok_or("No argval")?,
                        right
                    ));
                }
                Opcode::Call => {
                    let count = instruction.arg.ok_or("No arg")?;
                    let mut args = Vec::with_capacity(count);
                    for _ in 0..count {
                        args.push(stack.pop().ok_or("Stack is empty")?);
                    }
                    let func_name = stack
                        .pop()
                        .ok_or("Stack is empty")?
                        .trim_start_matches("NULL + ")
                        .to_string();
                    stack.push(format!("{}({})", func_name, args.join(", ")));
                }
                Opcode::ReturnValue => {
                    let value = stack.pop().ok_or("Stack is empty")?;
                    if value != "None" {
                        stack.push(format!("return {}", value));
                    }
                }
                _ => {}
            }
        }
        Ok(Self {
            expr_type,
            values,
            child,
            func,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_function() {
        let msg = r#"Disassembly of <code object foo at 0x00000246F5EA0D50, file "test/def.py", line 3>:"#;
        let function = Function::new(msg).unwrap();
        dbg!(function);
        assert!(false);
    }
}
