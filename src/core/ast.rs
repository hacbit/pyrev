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
        let mut offset = 0;
        loop {
            if offset == opcode_instructions.len() {
                break;
            }
            let instruction = opcode_instructions.get(offset).ok_or("[Parse] No instruction")?;
            #[cfg(debug_assertions)]
            {
                dbg!(&instruction);
            }

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
                Opcode::LoadMethod => {
                    let parent = exprs_stack.pop().ok_or("[LoadMethod] Stack is empty")?;
                    let method = instruction
                        .argval
                        .as_ref()
                        .ok_or("[LoadMethod] No argval")?;
                    exprs_stack.push(ExpressionEnum::Attribute(Attribute {
                        parent: Box::new(parent),
                        attr: Box::new(ExpressionEnum::BaseValue(BaseValue {
                            value: method.clone(),
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
                    tuple.reverse();
                    exprs_stack.push(ExpressionEnum::Container(Container {
                        values: tuple,
                        container_type: ContainerType::Tuple,
                    }));
                }
                Opcode::BuildList => {
                    let size = instruction.arg.ok_or("[BuildList] No arg")?;
                    if size == 0 {
                        exprs_stack.push(ExpressionEnum::Container(Container {
                            values: vec![],
                            container_type: ContainerType::List,
                        }));
                    } else {
                        let mut list = Vec::with_capacity(size);
                        for _ in 0..size {
                            list.push(exprs_stack.pop().ok_or("[BuildList] Stack is empty")?);
                        }
                        list.reverse();
                        exprs_stack.push(ExpressionEnum::Container(Container {
                            values: list,
                            container_type: ContainerType::List,
                        }));
                    }
                }
                Opcode::ListExtend => {
                    let size = instruction.arg.ok_or("[ListExtend] No arg")?;
                    let mut extend = Vec::with_capacity(size);
                    for _ in 0..size {
                        extend.push(exprs_stack.pop().ok_or("[ListExtend] Stack is empty")?);
                    }
                    extend.reverse();
                    let list = exprs_stack.pop().ok_or("[ListExtend] Stack is empty")?;
                    if let ExpressionEnum::Container(Container {
                        values: mut list,
                        container_type: ContainerType::List,
                    }) = list
                    {
                        extend.iter_mut().for_each(|x| {
                            if let ExpressionEnum::BaseValue(value) = x {
                                value.value = value
                                    .value
                                    .trim_start_matches('(')
                                    .trim_end_matches(')')
                                    .to_string();
                            }
                        });
                        list.append(&mut extend);
                        exprs_stack.push(ExpressionEnum::Container(Container {
                            values: list,
                            container_type: ContainerType::List,
                        }));
                    } else {
                        return Err("[ListExtend] Invalid list".into());
                    }
                }
                Opcode::BuildSet => {
                    let size = instruction.arg.ok_or("[BuildSet] No arg")?;
                    let mut set = Vec::with_capacity(size);
                    for _ in 0..size {
                        set.push(exprs_stack.pop().ok_or("[BuildSet] Stack is empty")?);
                    }
                    set.reverse();
                    exprs_stack.push(ExpressionEnum::Container(Container {
                        values: set,
                        container_type: ContainerType::Set,
                    }));
                }
                Opcode::BuildMap => {
                    let size = instruction.arg.ok_or("[BuildMap] No arg")?;
                    if size == 0 {
                        exprs_stack.push(ExpressionEnum::Container(Container {
                            values: vec![],
                            container_type: ContainerType::Dict,
                        }));
                    } else {
                        let mut map = Vec::with_capacity(size * 2);
                        for _ in 0..size {
                            let value = exprs_stack.pop().ok_or("[BuildMap] Stack is empty")?;
                            let key = exprs_stack.pop().ok_or("[BuildMap] Stack is empty")?;
                            map.push(value);
                            map.push(key);
                        }
                        map.reverse();
                        exprs_stack.push(ExpressionEnum::Container(Container {
                            values: map,
                            container_type: ContainerType::Dict,
                        }));
                    }
                }
                Opcode::BuildConstKeyMap => {
                    let size = instruction.arg.ok_or("[BuildConstKeyMap] No arg")?;
                    let keys = exprs_stack
                        .pop()
                        .ok_or("[BuildConstKeyMap] Stack is empty")?;
                    let mut values = Vec::with_capacity(size);
                    for _ in 0..size {
                        values.push(
                            exprs_stack
                                .pop()
                                .ok_or("[BuildConstKeyMap] Stack is empty")?,
                        );
                    }
                    values.reverse();
                    let mut map = Vec::with_capacity(size * 2);
                    if let ExpressionEnum::BaseValue(BaseValue { value: key }) = keys {
                        for (k, v) in key
                            .trim_start_matches('(')
                            .trim_end_matches(')')
                            .split(", ")
                            .zip(values)
                        {
                            map.push(ExpressionEnum::BaseValue(BaseValue {
                                value: k.to_string(),
                            }));
                            map.push(v);
                        }
                    }
                    exprs_stack.push(ExpressionEnum::Container(Container {
                        values: map,
                        container_type: ContainerType::Dict,
                    }))
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
                            #[cfg(debug_assertions)]
                            {
                                assert_eq!(container.container_type, ContainerType::Tuple);
                            }
                            for (i, value) in container.values.iter().enumerate() {
                                if let ExpressionEnum::BaseValue(base_value) = value {
                                    // 比如 ('a', int, 'return', int)
                                    // 需要把单引号去掉
                                    if i % 2 == 1 {
                                        function.args_annotation.push(base_value.value.clone());
                                    } else {
                                        function.args.push(
                                            base_value
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
                Opcode::UnaryInvert => {
                    let target = exprs_stack.pop().ok_or("[UnaryInvert] Stack is empty")?;
                    exprs_stack.push(ExpressionEnum::UnaryOperation(UnaryOperation {
                        target: Box::new(target),
                        unary_type: UnaryType::Invert,
                    }))
                }
                Opcode::UnaryNegative => {
                    let target = exprs_stack.pop().ok_or("[UnaryNegative] Stack is empty")?;
                    exprs_stack.push(ExpressionEnum::UnaryOperation(UnaryOperation {
                        target: Box::new(target),
                        unary_type: UnaryType::Negative,
                    }))
                }
                Opcode::UnaryNot => {
                    let target = exprs_stack.pop().ok_or("[UnaryNot] Stack is empty")?;
                    exprs_stack.push(ExpressionEnum::UnaryOperation(UnaryOperation {
                        target: Box::new(target),
                        unary_type: UnaryType::Not,
                    }))
                }
                Opcode::Call => {
                    let count = instruction.arg.ok_or("[Call] No arg")?;
                    if count == 0 {
                        offset += 1;
                        continue;
                    }
                    let mut args = Vec::with_capacity(count);
                    for _ in 0..count {
                        args.push(exprs_stack.pop().ok_or("[Call] Stack is empty")?);
                    }
                    args.reverse();
                    match exprs_stack.pop() {
                        Some(ExpressionEnum::BaseValue(function_name)) => {
                            //dbg!(&function_name);
                            let function_name = function_name.value.trim_start_matches("NULL + ");
                            exprs_stack.push(ExpressionEnum::Call(Call {
                                func: Box::new(ExpressionEnum::BaseValue(BaseValue {
                                    value: function_name.to_string(),
                                })),
                                args,
                            }))
                        }
                        Some(function) => exprs_stack.push(ExpressionEnum::Call(Call {
                            func: Box::new(function),
                            args,
                        })),
                        None => return Err("[Call] Stack is empty".into()),
                    }
                }
                Opcode::ReturnValue => {
                    //dbg!(&exprs_stack);
                    let value = exprs_stack.pop().ok_or("[ReturnValue] Stack is empty")?;
                    exprs_stack.push(ExpressionEnum::Return(Return {
                        value: Box::new(value),
                    }));
                }
                Opcode::ImportName => {
                    let module = instruction
                        .argval
                        .as_ref()
                        .ok_or("[ImportName] No argval")?;

                    /* exprs_stack.push(ExpressionEnum::Import(Import {
                        module: module.clone(),
                        ..
                    })) */
                }
                Opcode::PopJumpIfTrue => {
                    if let Some(next_instruction) = opcode_instructions.get(offset + 1) {
                        // if next instruction is LoadAssertionError, then it's an assert
                        if next_instruction.opcode == Opcode::LoadAssertionError {
                            offset += 1;
                            continue;
                        }
                    }

                    let test = exprs_stack.pop().ok_or("[PopJumpIfTrue] Stack is empty")?;
                    let test = ExpressionEnum::UnaryOperation(UnaryOperation {
                        target: Box::new(test),
                        unary_type: UnaryType::Not,
                    });
                    let jump_target = instruction
                        .argval
                        .as_ref()
                        .ok_or("[PopJumpIfTrue] No argval")?
                        .trim_start_matches("to ")
                        .parse::<usize>()?;
                    exprs_stack.push(ExpressionEnum::If(If {
                        test: Box::new(test),
                        body: vec![],
                        or_else: vec![ExpressionEnum::Jump(Jump {
                            target: jump_target,
                            body: vec![],
                        })],
                    }));
                }
                Opcode::PopJumpIfFalse => {
                    if let Some(next_instruction) = opcode_instructions.get(offset + 1) {
                        // if next instruction is LoadAssertionError, then it's an assert
                        if next_instruction.opcode == Opcode::LoadAssertionError {
                            offset += 1;
                            continue;
                        }
                    }

                    let test = exprs_stack.pop().ok_or("[PopJumpIfFalse] Stack is empty")?;
                    let jump_target = instruction
                        .argval
                        .as_ref()
                        .ok_or("[PopJumpIfFalse] No argval")?
                        .trim_start_matches("to ")
                        .parse::<usize>()?;
                    exprs_stack.push(ExpressionEnum::If(If {
                        test: Box::new(test),
                        body: vec![],
                        or_else: vec![ExpressionEnum::Jump(Jump {
                            target: jump_target,
                            body: vec![],
                        })],
                    }));
                }
                Opcode::LoadAssertionError => {
                    let test = exprs_stack
                        .pop()
                        .ok_or("[LoadAssertionError] Stack is empty")?;

                    exprs_stack.push(ExpressionEnum::Assert(Assert {
                        test: Box::new(test),
                        msg: None,
                    }))
                }
                Opcode::RaiseVarargs => {
                    let exception = exprs_stack.pop().ok_or("[RaiseVarargs] Stack is empty")?;
                    if let Some(expr) = exprs_stack.pop() {
                        if let Ok(assert) = expr.query_singleton::<Assert>() {
                            assert
                                .with_mut()
                                .patch_by(|a| a.msg = Some(Box::new(exception)))?;
                        }
                        exprs_stack.push(expr);
                    } else {
                        exprs_stack.push(ExpressionEnum::Raise(Raise {
                            exception: Box::new(exception),
                        }))
                    }

                    #[cfg(debug_assertions)]
                    {
                        dbg!(&exprs_stack);
                    }
                }

                _ => {}
            }

            offset += 1;
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
