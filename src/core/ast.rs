use super::opcode::{Opcode, OpcodeInstruction};
use pyrev_ast::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait ExprParser {
    fn parse(opcode_instructions: &[OpcodeInstruction]) -> Result<Box<Self>>;
}

impl ExprParser for Expr {
    /// 用于解析一段字节码指令为AST
    fn parse(opcode_instructions: &[OpcodeInstruction]) -> Result<Box<Self>> {
        let mut exprs_stack = Vec::<ExpressionEnum>::new();
        for instruction in opcode_instructions {
            match instruction.opcode {
                Opcode::LoadConst | Opcode::LoadName | Opcode::LoadFast | Opcode::LoadGlobal => {
                    exprs_stack.push(ExpressionEnum::BaseValue(BaseValue {
                        value: instruction
                            .argval
                            .as_ref()
                            .ok_or("[Load] No argval")?
                            .clone(),
                    }));
                }
                Opcode::LoadAttr => {
                    let parent = exprs_stack.pop().ok_or("[LoadAttr] Stack is empty")?;
                    let attr = instruction.argval.as_ref().ok_or("[LoadAttr] No argval")?;
                    exprs_stack.push(ExpressionEnum::Attribute(Attribute {
                        parent: Box::new(parent),
                        attr: Box::new(ExpressionEnum::BaseValue(BaseValue {
                            value: attr.clone(),
                        })),
                    }));
                }
                Opcode::StoreName | Opcode::StoreFast | Opcode::StoreGlobal => {
                    let name = instruction
                        .argval
                        .as_ref()
                        .ok_or("[Store] No argval")?
                        .clone();
                    let value = exprs_stack.pop().ok_or("[Store] Stack is empty")?;

                    match value {
                        ExpressionEnum::Function(_) => {
                            exprs_stack.push(value);
                        }
                        _ => {
                            exprs_stack.push(ExpressionEnum::Assign(Assign {
                                target: Box::new(ExpressionEnum::BaseValue(BaseValue {
                                    value: name,
                                })),
                                values: Box::new(value),
                                operator: "=".to_string(),
                            }));
                        }
                    }
                }
                Opcode::StoreAttr => {
                    let parent = exprs_stack.pop().ok_or("[StoreAttr] Stack is empty")?;
                    let attr = instruction.argval.as_ref().ok_or("[StoreAttr] No argval")?;
                    let value = exprs_stack.pop().ok_or("[StoreAttr] Stack is empty")?;
                    exprs_stack.push(ExpressionEnum::Assign(Assign {
                        target: Box::new(ExpressionEnum::Attribute(Attribute {
                            parent: Box::new(parent),
                            attr: Box::new(ExpressionEnum::BaseValue(BaseValue {
                                value: attr.clone(),
                            })),
                        })),
                        values: Box::new(value),
                        operator: "=".to_string(),
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
                Opcode::BuildSlice => {
                    let size = instruction.arg.ok_or("[BuildSlice] No arg")?;
                    let mut slice = Vec::with_capacity(size);
                    for _ in 0..size {
                        slice.push(exprs_stack.pop().ok_or("[BuildSlice] Stack is empty")?);
                    }
                    slice.reverse();
                    let origin = exprs_stack.pop().ok_or("[BuildSlice] Stack is empty")?;
                    exprs_stack.push(ExpressionEnum::Slice(Slice {
                        origin: Box::new(origin),
                        slice,
                    }));
                }
                Opcode::MakeFunction => {
                    let mark = exprs_stack.pop().ok_or("[MakeFunction] Stack is empty")?;
                    let mut function = Function::from(mark)?;
                    if instruction.argval == Some("annotations".to_string()) {
                        let values = exprs_stack.pop().ok_or("[MakeFunction] Stack is empty")?;
                        if let ExpressionEnum::Container(container) = values {
                            assert_eq!(container.container_type, ContainerType::Tuple);
                            for (i, value) in container.values.iter().rev().enumerate() {
                                if let ExpressionEnum::BaseValue(value) = value {
                                    // 比如 ('a', int, 'return', int)
                                    // 需要把单引号去掉
                                    if i % 2 == 1 {
                                        function.args_annotation.push(value.value.clone());
                                    } else {
                                        function.args.push(
                                            value
                                                .value
                                                .trim_start_matches('\'')
                                                .trim_end_matches('\'')
                                                .to_string(),
                                        );
                                    }
                                }
                            }
                        }
                    }
                    exprs_stack.push(ExpressionEnum::Function(function));
                }
                // BinaryOperation
                Opcode::BinaryOp | Opcode::CompareOp => {
                    let right = exprs_stack.pop().ok_or("[BinaryOp] Stack is empty")?;
                    let left = exprs_stack.pop().ok_or("[BinaryOp] Stack is empty")?;
                    exprs_stack.push(ExpressionEnum::BinaryOperation(BinaryOperation {
                        left: Box::new(left),
                        right: Box::new(right),
                        operator: instruction
                            .argval
                            .as_ref()
                            .ok_or("[BinaryOp] No argval")?
                            .clone(),
                    }))
                }
                // BinaryOperation
                Opcode::IsOp => {
                    let right = exprs_stack.pop().ok_or("[IsOp] Stack is empty")?;
                    let left = exprs_stack.pop().ok_or("[IsOp] Stack is empty")?;
                    let operator = match instruction.arg.as_ref() {
                        Some(0) => "is",
                        Some(1) => "is not",
                        _ => return Err("[IsOp] No arg or Invalid arg".into()),
                    };
                    exprs_stack.push(ExpressionEnum::BinaryOperation(BinaryOperation {
                        left: Box::new(left),
                        right: Box::new(right),
                        operator: operator.to_string(),
                    }))
                }
                // BinaryOperation
                Opcode::ContainsOp => {
                    let right = exprs_stack.pop().ok_or("[ContainsOp] Stack is empty")?;
                    let left = exprs_stack.pop().ok_or("[ContainsOp] Stack is empty")?;
                    let operator = match instruction.arg.as_ref() {
                        Some(0) => "in",
                        Some(1) => "not in",
                        _ => return Err("[ContainsOp] No arg or Invalid arg".into()),
                    };
                    exprs_stack.push(ExpressionEnum::BinaryOperation(BinaryOperation {
                        left: Box::new(left),
                        right: Box::new(right),
                        operator: operator.to_string(),
                    }))
                }
                Opcode::Call => {
                    let count = instruction.arg.ok_or("[Call] No arg")?;
                    let mut args = Vec::with_capacity(count);
                    for _ in 0..count {
                        args.push(exprs_stack.pop().ok_or("[Call] Stack is empty")?);
                    }
                    if let ExpressionEnum::BaseValue(function_name) =
                        exprs_stack.pop().ok_or("[Call] Stack is empty")?
                    {
                        //dbg!(&function_name);
                        let function_name = function_name.value.trim_start_matches("NULL + ");
                        exprs_stack.push(ExpressionEnum::Call(Call {
                            func: Box::new(ExpressionEnum::BaseValue(BaseValue {
                                value: function_name.to_string(),
                            })),
                            args,
                        }))
                    }
                    //dbg!(&exprs_stack);
                }
                Opcode::ReturnValue => {
                    //dbg!(&exprs_stack);
                    let value = exprs_stack.pop().ok_or("[ReturnValue] Stack is empty")?;
                    exprs_stack.push(ExpressionEnum::Return(Return {
                        value: Box::new(value),
                    }));
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

    #[test]
    fn test_parse_function() {
        let msg = r#"<code object foo at 0x00000246F5EA0D50, file "test/def.py", line 3>"#;
        let function = Function::new(msg).unwrap();

        assert_eq!(
            function,
            Function {
                mark: "<code object foo at 0x00000246F5EA0D50, file \"test/def.py\", line 3>"
                    .into(),
                name: "foo".into(),
                args: vec![],
                args_annotation: vec![],
                start_line: 3,
                end_line: 3,
                bodys: vec![],
            }
        )
        //dbg!(function);
        //assert!(false);
    }

    #[test]
    fn test_parse_expr() {
        let instructions = [
            OpcodeInstruction {
                opcode: Opcode::LoadConst,
                opname: "LOAD_CONST".into(),
                arg: Some(0),
                argval: Some("'a'".into()),
                offset: 2,
                starts_line: Some(1),
                is_jump_target: false,
                positions: vec![],
            },
            OpcodeInstruction {
                opcode: Opcode::LoadName,
                opname: "LOAD_NAME".into(),
                arg: Some(0),
                argval: Some("int".into()),
                offset: 4,
                starts_line: None,
                is_jump_target: false,
                positions: vec![],
            },
            OpcodeInstruction {
                opcode: Opcode::LoadConst,
                opname: "LOAD_CONST".into(),
                arg: Some(1),
                argval: Some("'return'".into()),
                offset: 6,
                starts_line: None,
                is_jump_target: false,
                positions: vec![],
            },
            OpcodeInstruction {
                opcode: Opcode::LoadName,
                opname: "LOAD_NAME".into(),
                arg: Some(0),
                argval: Some("int".into()),
                offset: 8,
                starts_line: None,
                is_jump_target: false,
                positions: vec![],
            },
            OpcodeInstruction {
                opcode: Opcode::BuildTuple,
                opname: "BUILD_TUPLE".into(),
                arg: Some(4),
                argval: None,
                offset: 10,
                starts_line: None,
                is_jump_target: false,
                positions: vec![],
            },
            OpcodeInstruction {
                opcode: Opcode::LoadConst,
                opname: "LOAD_CONST".into(),
                arg: Some(2),
                argval: Some(
                    "<code object test at 0x00000279922BDB80, file \"test/def.py\", line 1>".into(),
                ),
                offset: 12,
                starts_line: None,
                is_jump_target: false,
                positions: vec![],
            },
            OpcodeInstruction {
                opcode: Opcode::MakeFunction,
                opname: "MAKE_FUNCTION".into(),
                arg: Some(4),
                argval: Some("annotations".into()),
                offset: 14,
                starts_line: None,
                is_jump_target: false,
                positions: vec![],
            },
            OpcodeInstruction {
                opcode: Opcode::StoreName,
                opname: "STORE_NAME".into(),
                arg: Some(1),
                argval: Some("test".into()),
                offset: 16,
                starts_line: None,
                is_jump_target: false,
                positions: vec![],
            },
        ];

        assert_eq!(
            Expr::parse(&instructions).unwrap(),
            Box::new(Expr {
                bodys: [ExpressionEnum::Function(Function {
                    mark: "<code object test at 0x00000279922BDB80, file \"test/def.py\", line 1>"
                        .into(),
                    name: "test".into(),
                    args: ["a".into(), "return".into(),].into(),
                    args_annotation: ["int".into(), "int".into(),].into(),
                    start_line: 1,
                    end_line: 1,
                    bodys: vec![],
                },),]
                .into(),
            })
        )
    }

    #[test]
    fn test_build_from_ast() {
        let input = Box::new(Expr {
            bodys: [
                ExpressionEnum::Function(Function {
                    mark: "<code object test at 0x00000279922BDB80, file \"test/def.py\", line 1>"
                        .into(),
                    name: "test".into(),
                    args: ["a".into(), "return".into()].into(),
                    args_annotation: ["int".into(), "int".into()].into(),
                    start_line: 1,
                    end_line: 1,
                    bodys: vec![],
                }),
                ExpressionEnum::Call(Call {
                    func: Box::new(ExpressionEnum::BaseValue(BaseValue {
                        value: "print".into(),
                    })),
                    args: vec![ExpressionEnum::BaseValue(BaseValue {
                        value: "test".into(),
                    })],
                }),
            ]
            .into(),
        });
        let mut res = Vec::new();
        for expr in input.bodys.iter() {
            res.append(&mut expr.build().unwrap());
        }
        //dbg!(res);
        assert_eq!(
            res,
            vec!["def test(a: int) -> int:", "    pass", "print(test)",]
        )
    }

    #[test]
    fn test_query() {
        let expr = Box::new(Expr {
            bodys: [ExpressionEnum::Assign(Assign {
                target: Box::new(ExpressionEnum::BaseValue(BaseValue {
                    value: "test".into(),
                })),
                values: Box::new(ExpressionEnum::Function(Function {
                    mark: "<code object test at 0x00000279922BDB80, file \"test/def.py\", line 1>"
                        .into(),
                    name: "test".into(),
                    args: ["a".into(), "return".into()].into(),
                    args_annotation: ["int".into(), "int".into()].into(),
                    start_line: 1,
                    end_line: 1,
                    bodys: vec![],
                })),
                operator: "=".into(),
            })]
            .into(),
        });
        let assign_query = expr.query::<Assign>();
        let function_query = expr.query::<Function>();
        //dbg!(query);
        //assert!(false);
        assert_eq!(
            assign_query,
            vec![&Assign {
                target: Box::new(ExpressionEnum::BaseValue(BaseValue {
                    value: "test".into()
                })),
                values: Box::new(ExpressionEnum::Function(Function {
                    mark: "<code object test at 0x00000279922BDB80, file \"test/def.py\", line 1>"
                        .into(),
                    name: "test".into(),
                    args: ["a".into(), "return".into(),].into(),
                    args_annotation: ["int".into(), "int".into(),].into(),
                    start_line: 1,
                    end_line: 1,
                    bodys: vec![],
                },)),
                operator: "=".into(),
            }]
        );
        assert_eq!(
            function_query,
            vec![&Function {
                mark: "<code object test at 0x00000279922BDB80, file \"test/def.py\", line 1>"
                    .into(),
                name: "test".into(),
                args: ["a".into(), "return".into(),].into(),
                args_annotation: ["int".into(), "int".into(),].into(),
                start_line: 1,
                end_line: 1,
                bodys: vec![],
            }]
        );
    }

    #[test]
    fn test_any() {
        let expr = Assign {
            target: Box::new(ExpressionEnum::BaseValue(BaseValue {
                value: "a".to_string(),
            })),
            values: Box::new(ExpressionEnum::BaseValue(BaseValue {
                value: "1".to_string(),
            })),
            operator: "=".to_string(),
        };
        let any = expr.try_query::<Assign>();

        //dbg!(any);
        //assert!(false);
        assert_eq!(
            any,
            Some(&Assign {
                target: Box::new(ExpressionEnum::BaseValue(BaseValue {
                    value: "a".to_string(),
                })),
                values: Box::new(ExpressionEnum::BaseValue(BaseValue {
                    value: "1".to_string(),
                })),
                operator: "=".to_string(),
            })
        )
    }
}
