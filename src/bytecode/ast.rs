use pyrev_ast::*;

use super::opcode::{Opcode, OpcodeInstruction};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait ExprParser {
    fn parse(opcode_instructions: &Vec<OpcodeInstruction>) -> Result<Box<Self>>;
}

impl ExprParser for Expr {
    fn parse(opcode_instructions: &Vec<OpcodeInstruction>) -> Result<Box<Self>> {
        let start_line = opcode_instructions
            .first()
            .ok_or("Not have first")?
            .starts_line
            .ok_or("Not have starts_line")?;
        let mut stack = Vec::new();
        let mut exprs = Vec::<ExpressionEnum>::new();
        for instruction in opcode_instructions {
            match instruction.opcode {
                Opcode::None => {}
                Opcode::Nop => {}
                Opcode::LoadConst | Opcode::LoadName | Opcode::LoadFast | Opcode::LoadGlobal => {
                    stack.push(instruction.argval.as_ref().ok_or("No argval")?.clone());
                }
                Opcode::StoreName | Opcode::StoreFast | Opcode::StoreGlobal => {
                    let name = instruction.argval.as_ref().ok_or("No argval")?.clone();
                    let value = stack.pop().ok_or("Stack is empty")?;
                    // 判断exprs是否包含Function

                    exprs.push(ExpressionEnum::Assign(Assign {
                        name,
                        values: Box::new(ExpressionEnum::BaseValue(BaseValue { value })),
                    }));
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
                    exprs.push(ExpressionEnum::Function(function));
                }
                Opcode::BinaryOp => {
                    let right = stack.pop().ok_or("Stack is empty")?;
                    let left = stack.pop().ok_or("Stack is empty")?;
                    stack.push(format!(
                        "{} {} {}",
                        left,
                        instruction.argval.as_ref().ok_or("No argval")?,
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

        Ok(Box::new(Self { bodys: exprs }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_function() {
        let msg = r#"Disassembly of <code object foo at 0x00000246F5EA0D50, file "test/def.py", line 3>:"#;
        let function = Function::new(msg).unwrap();

        assert_eq!(function.mark, msg,);
        assert_eq!(function.name, "foo");
        assert_eq!(function.start_line, 3);
        assert_eq!(function.end_line, 3);
        assert_eq!(function.args, Vec::<String>::new());
        assert_eq!(function.args_annotation, Vec::<String>::new());
        //assert!(false);
    }
}
