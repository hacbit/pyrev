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
        let mut exprs_stack = Vec::<ExpressionEnum>::new();
        for instruction in opcode_instructions {
            match instruction.opcode {
                Opcode::None => {}
                Opcode::Nop => {}
                Opcode::LoadConst | Opcode::LoadName | Opcode::LoadFast | Opcode::LoadGlobal => {
                    exprs_stack.push(ExpressionEnum::BaseValue(BaseValue {
                        value: instruction.argval.as_ref().ok_or("[Load] No argval")?.clone(),
                    }));
                }
                Opcode::StoreName | Opcode::StoreFast | Opcode::StoreGlobal => {
                    let name = instruction.argval.as_ref().ok_or("[Store] No argval")?.clone();
                    let value = exprs_stack.pop().ok_or("[Store] Stack is empty")?;
                    // 判断exprs是否包含Function

                    exprs_stack.push(ExpressionEnum::Assign(Assign {
                        name,
                        values: Box::new(value),
                    }));
                }
                Opcode::BuildTuple => {
                    let size = instruction.arg.ok_or("[BuildTuple] No arg")?;
                    let mut tuple = Vec::with_capacity(size);
                    for _ in 0..size {
                        tuple.push(exprs_stack.pop().ok_or("[BuildTuple] Stack is empty")?);
                    }
                    exprs_stack.push(ExpressionEnum::Container(Container {
                        values: tuple,
                        container_type: ContainerType::Tuple,
                    }));
                }
                Opcode::MakeFunction => {
                    let mark = exprs_stack.pop().ok_or("[MakeFunction] Stack is empty")?;
                    let mut function = Function::from(mark)?;
                    if instruction.argval == Some("annotations".to_string()) {
                        let values = exprs_stack
                            .pop()
                            .ok_or("[MakeFunction] Stack is empty")?;
                        if let ExpressionEnum::Container(container) = values {
                            assert_eq!(container.container_type, ContainerType::Tuple);
                            for (i, value) in container.values.iter().rev().enumerate() {
                                if let ExpressionEnum::BaseValue(value) = value {
                                    if i % 2 == 1 {
                                        function.args_annotation.push(value.value.clone());
                                    } else {
                                        function.args.push(value.value.trim_start_matches('\'').trim_end_matches('\'').to_string());
                                    }
                                }
                            }
                        }
                    }
                    exprs_stack.push(ExpressionEnum::Function(function));
                }
                Opcode::BinaryOp => {
                    let right = exprs_stack.pop().ok_or("[BinaryOp] Stack is empty")?;
                    let left = exprs_stack.pop().ok_or("[BinaryOp] Stack is empty")?;
                    exprs_stack.push(ExpressionEnum::BinaryOperation(BinaryOperation {
                        left: Box::new(left),
                        right: Box::new(right),
                        operator: instruction.argval.as_ref().ok_or("[BinaryOp] No argval")?.clone(),
                    }))
                }
                Opcode::Call => {
                    let count = instruction.arg.ok_or("[Call] No arg")?;
                    let mut args = Vec::with_capacity(count);
                    for _ in 0..count {
                        args.push(exprs_stack.pop().ok_or("[Call] Stack is empty")?);
                    }
                    if let ExpressionEnum::BaseValue(function_name) = exprs_stack.pop().ok_or("[Call] Stack is empty")? {
                        let function_name = function_name.value.trim_start_matches("NULL + ");
                        exprs_stack.push(ExpressionEnum::Call(Call {
                            func: Box::new(ExpressionEnum::BaseValue(BaseValue {
                                value: function_name.to_string(),
                            })),
                            args,
                        }))
                    }
                }
                Opcode::ReturnValue => {
                    let value = exprs_stack.pop();
                    if let Some(value) = value {
                        if let ExpressionEnum::BaseValue(value) = value {
                            if value.value != "None" {
                                exprs_stack.push(ExpressionEnum::BaseValue(BaseValue {
                                    value: format!("return {}", value.value),
                                }))
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(Box::new(Self { bodys: exprs_stack }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::parse_opcode::OpcodeParser;
    use std::fs;

    #[test]
    fn test_parse_function() {
        let msg = r#"<code object foo at 0x00000246F5EA0D50, file "test/def.py", line 3>"#;
        let function = Function::new(msg).unwrap();

        assert_eq!(function.mark, msg,);
        assert_eq!(function.name, "foo");
        assert_eq!(function.start_line, 3);
        assert_eq!(function.end_line, 3);
        assert_eq!(function.args, Vec::<String>::new());
        assert_eq!(function.args_annotation, Vec::<String>::new());
        //dbg!(function);
        //assert!(false);
    }

    #[test]
    fn test_parse_expr() {
        let input = fs::read_to_string("test/def.txt").unwrap();
        let opcode_instructions = input.parse().unwrap();
        let a = opcode_instructions.get("<main>").unwrap();
        let a = a.get(&1).unwrap();
        //dbg!(&a);
        assert_eq!(
            Expr::parse(&a).unwrap(),
            Box::new(Expr {
                bodys: [
                    ExpressionEnum::Assign(
                        Assign {
                            name: "test".into(),
                            values: Box::new(ExpressionEnum::Function(
                                Function {
                                    mark: "<code object test at 0x00000279922BDB80, file \"test/def.py\", line 1>".into(),
                                    name: "test".into(),
                                    args: [
                                        "a".into(),
                                        "return".into(),
                                    ].into(),
                                    args_annotation: [
                                        "int".into(),
                                        "int".into(),
                                    ].into(),
                                    start_line: 1,
                                    end_line: 1,
                                    bodys: vec![],
                                },
                            )),
                        },
                    ),
                ].into(),
            })
        )
    }
}
