use super::prelude::*;
use pyrev_ast::*;
use regex::Regex;

use std::cmp::Ordering;

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
            let instruction = opcode_instructions
                .get(offset)
                .ok_or("[Parse] No instruction")?;

            let opcode = instruction.opcode();
            match opcode {
                Opcode::LoadConst | Opcode::LoadName | Opcode::LoadGlobal => {
                    exprs_stack.push(ExpressionEnum::BaseValue(BaseValue {
                        value: instruction
                            .argval
                            .as_ref()
                            .ok_or(format!(
                                "[Load] No argval, deviation is {}",
                                instruction.offset
                            ))?
                            .trim_start_matches("NULL + ")
                            .to_string(),
                        start_offset: instruction.offset,
                        end_offset: instruction.offset,
                        ..Default::default()
                    }));
                }
                Opcode::LoadFast => {
                    let name = instruction
                        .argval
                        .as_ref()
                        .ok_or(format!(
                            "[LoadFast] No argval, deviation is {}",
                            instruction.offset
                        ))?
                        .clone();

                    exprs_stack.push(ExpressionEnum::BaseValue(BaseValue {
                        value: name,
                        start_offset: instruction.offset,
                        end_offset: instruction.offset,
                        ..Default::default()
                    }));
                }
                Opcode::LoadAttr => {
                    let parent = exprs_stack.pop().ok_or(format!(
                        "[LoadAttr] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    let attr = instruction.argval.as_ref().ok_or(format!(
                        "[LoadAttr] No argval, deviation is {}",
                        instruction.offset
                    ))?;
                    exprs_stack.push(ExpressionEnum::Attribute(Attribute {
                        parent: Box::new(parent),
                        attr: Box::new(ExpressionEnum::BaseValue(BaseValue {
                            value: attr.clone(),
                            ..Default::default()
                        })),
                        start_offset: instruction.offset,
                        end_offset: instruction.offset,
                        ..Default::default()
                    }));
                }
                Opcode::LoadMethod => {
                    let parent = exprs_stack.pop().ok_or(format!(
                        "[LoadMethod] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    let method = instruction.argval.as_ref().ok_or(format!(
                        "[LoadMethod] No argval, deviation is {}",
                        instruction.offset
                    ))?;
                    exprs_stack.push(ExpressionEnum::Attribute(Attribute {
                        parent: Box::new(parent),
                        attr: Box::new(ExpressionEnum::BaseValue(BaseValue {
                            value: method.clone(),
                            ..Default::default()
                        })),
                        start_offset: instruction.offset,
                        end_offset: instruction.offset,
                        ..Default::default()
                    }));
                }
                Opcode::StoreName | Opcode::StoreGlobal => {
                    let name = instruction
                        .argval
                        .as_ref()
                        .ok_or(format!(
                            "[Store] No argval, deviation is {}",
                            instruction.offset
                        ))?
                        .clone();

                    let value = exprs_stack.pop().ok_or(format!(
                        "[Store] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;

                    match value {
                        ExpressionEnum::Function(function) => {
                            if function.name.as_str() == "<lambda>"
                                || function.name.as_str() == "<genexpr>"
                                || function.name.as_str() == "<listcomp>"
                            {
                                exprs_stack.push(ExpressionEnum::Assign(Assign {
                                    target: Box::new(ExpressionEnum::BaseValue(BaseValue {
                                        value: name,
                                        ..Default::default()
                                    })),
                                    values: Box::new(ExpressionEnum::Function(function)),
                                    operator: "=".to_string(),
                                    ..Default::default()
                                }));
                            } else {
                                exprs_stack.push(ExpressionEnum::Function(function));
                            }
                        }
                        ExpressionEnum::Import(import) => {
                            if import.bk_module.is_none() {
                                //没from
                                if import.module == name {
                                    //没from，没as
                                    exprs_stack.push(ExpressionEnum::Import(Import {
                                        module: import.module,
                                        bk_module: None,
                                        alias: None,
                                        fragment: None,
                                        start_line: import.start_line,
                                        start_offset: import.start_offset,
                                        end_offset: import.end_offset,
                                    }))
                                } else {
                                    //没from，有as
                                    exprs_stack.push(ExpressionEnum::Import(Import {
                                        module: import.module,
                                        bk_module: None,
                                        alias: Some(name),
                                        fragment: None,
                                        start_line: import.start_line,
                                        start_offset: import.start_offset,
                                        end_offset: import.end_offset,
                                    }))
                                }
                            } else {
                                //有from

                                if import.fragment.as_ref().ok_or(format!(
                                    "[StoreName-import-HaveFrom], deviation is {}",
                                    instruction.offset
                                ))? == &name
                                {
                                    //有from，无as
                                    exprs_stack.push(ExpressionEnum::Import(Import {
                                        module: import.module,
                                        bk_module: Some(
                                            import.bk_module.expect("") + &name.clone() + ", ",
                                        ),
                                        alias: None,
                                        fragment: None,
                                        start_line: import.start_line,
                                        start_offset: import.start_offset,
                                        end_offset: import.end_offset,
                                    }))
                                } else {
                                    //有from，有as
                                    exprs_stack.push(ExpressionEnum::Import(Import {
                                        module: import.module,
                                        bk_module: Some(
                                            import.bk_module.expect("")
                                                + import.fragment.as_ref().ok_or(format!(
                                                    "[StoreName-import-HaveFromAs], deviation is {}",
                                                    instruction.offset
                                                ))?
                                                + " as "
                                                + &name.clone()
                                                + ", ",
                                        ),
                                        alias: None,
                                        fragment: None,
                                        start_line: import.start_line,
                                        start_offset: import.start_offset,
                                        end_offset: import.end_offset,
                                    }))
                                }
                            }
                        }
                        _ => {
                            exprs_stack.push(ExpressionEnum::Assign(Assign {
                                target: Box::new(ExpressionEnum::BaseValue(BaseValue {
                                    value: name,
                                    ..Default::default()
                                })),
                                values: Box::new(value),
                                operator: "=".to_string(),
                                start_offset: instruction.offset,
                                end_offset: instruction.offset,
                                ..Default::default()
                            }));
                        }
                    }
                }
                Opcode::StoreFast => {
                    let name = instruction
                        .argval
                        .as_ref()
                        .ok_or("[StoreFast] No argval")?
                        .clone();
                    let value = exprs_stack.pop().ok_or(format!(
                        "[StoreFast] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;

                    match value {
                        ExpressionEnum::Function(_) => {
                            exprs_stack.push(value);
                        }
                        _ => exprs_stack.push(ExpressionEnum::Assign(Assign {
                            target: Box::new(ExpressionEnum::BaseValue(BaseValue {
                                value: name,
                                ..Default::default()
                            })),
                            values: Box::new(value),
                            operator: "=".to_string(),
                            start_line: instruction.starts_line.unwrap_or_default(),
                            start_offset: instruction.offset,
                            end_offset: instruction.offset,
                        })),
                    }
                }
                Opcode::StoreAttr => {
                    let parent = exprs_stack.pop().ok_or(format!(
                        "[StoreAttr] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    let attr = instruction.argval.as_ref().ok_or(format!(
                        "[StoreAttr] No argval, deviation is {}",
                        instruction.offset
                    ))?;
                    let value = exprs_stack.pop().ok_or(format!(
                        "[StoreAttr] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    let target = ExpressionEnum::Attribute(Attribute {
                        parent: Box::new(parent),
                        attr: Box::new(ExpressionEnum::BaseValue(BaseValue {
                            value: attr.clone(),
                            ..Default::default()
                        })),
                        start_offset: instruction.offset,
                        end_offset: instruction.offset,
                        ..Default::default()
                    });
                    // if the value is a binary operation and it's a self-assign operation like +=, -=, etc.
                    // and the target is the same as the left of the binary operation
                    // then we can ignore the target and just return the binary operation
                    if let ExpressionEnum::BinaryOperation(BinaryOperation {
                        left, operator, ..
                    }) = value.clone()
                    {
                        if let ExpressionEnum::Attribute(Attribute { parent, attr, .. }) = *left {
                            // expect an attribute expression
                            let target = target.unwrap_attribute();
                            if parent == target.parent
                                && attr == target.attr
                                && operator.ends_with('=')
                            {
                                // it will continue soon
                                // so move (instead clone) the value to the exprs_stack
                                exprs_stack.push(value);
                                offset += 1;
                                continue;
                            }
                        }
                    }
                    exprs_stack.push(ExpressionEnum::Assign(Assign {
                        target: Box::new(target),
                        values: Box::new(value),
                        operator: "=".to_string(),
                        start_line: instruction.starts_line.unwrap_or_default(),
                        start_offset: instruction.offset,
                        end_offset: instruction.offset,
                    }));
                    //dbg!(&exprs_stack);
                }
                Opcode::LoadBuildClass => {
                    let mark = opcode_instructions
                        .get(offset + 1)
                        .cloned()
                        .ok_or(format!(
                            "[LoadBuildClass] No mark, deviation is {}",
                            instruction.offset
                        ))?
                        .argval
                        .ok_or(format!(
                            "[LoadBuildClass] No argval, deviation is {}",
                            instruction.offset
                        ))?;
                    let class = Class::new(mark)?;
                    exprs_stack.push(ExpressionEnum::Class(class));

                    // skip build class
                    loop {
                        offset += 1;
                        if let Some(next_instruction) = opcode_instructions.get(offset) {
                            if next_instruction.starts_line.unwrap()
                                != instruction.starts_line.unwrap()
                            {
                                break;
                            }
                        }
                    }
                    offset -= 1;
                }
                Opcode::FormatValue => {
                    let format_value = exprs_stack.pop().ok_or(format!(
                        "[FormatValue] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    exprs_stack.push(ExpressionEnum::FormatValue(FormatValue {
                        value: Box::new(format_value),
                        start_offset: instruction.offset,
                        end_offset: instruction.offset,
                        ..Default::default()
                    }));
                }
                Opcode::BuildString => {
                    let size = instruction.arg.ok_or(format!(
                        "[BuildString] No arg, deviation is {}",
                        instruction.offset
                    ))?;
                    let mut format_string = Vec::with_capacity(size);
                    for _ in 0..size {
                        format_string.push(exprs_stack.pop().ok_or(format!(
                            "[BuildString] Stack is empty, deviation is {}",
                            instruction.offset
                        ))?);
                    }
                    format_string.reverse();
                    exprs_stack.push(ExpressionEnum::Format(Format {
                        format_values: format_string,
                        start_offset: instruction.offset,
                        end_offset: instruction.offset,
                        ..Default::default()
                    }));
                }
                Opcode::BuildTuple => {
                    let size = instruction.arg.ok_or(format!(
                        "[BuildTuple] No arg, deviation is {}",
                        instruction.offset
                    ))?;
                    let mut tuple = Vec::with_capacity(size);
                    for _ in 0..size {
                        tuple.push(exprs_stack.pop().ok_or(format!(
                            "[BuildTuple] Stack is empty, deviation is {}",
                            instruction.offset
                        ))?);
                    }
                    tuple.reverse();
                    exprs_stack.push(ExpressionEnum::Container(Container {
                        values: tuple,
                        container_type: ContainerType::Tuple,
                        start_offset: instruction.offset,
                        end_offset: instruction.offset,
                        ..Default::default()
                    }));
                }
                Opcode::BuildList => {
                    let size = instruction.arg.ok_or(format!(
                        "[BuildList] No arg, deviation is {}",
                        instruction.offset
                    ))?;
                    if size == 0 {
                        exprs_stack.push(ExpressionEnum::Container(Container {
                            values: vec![],
                            container_type: ContainerType::List,
                            start_offset: instruction.offset,
                            end_offset: instruction.offset,
                            ..Default::default()
                        }));
                    } else {
                        let mut list = Vec::with_capacity(size);
                        for _ in 0..size {
                            list.push(exprs_stack.pop().ok_or(format!(
                                "[BuildList] Stack is empty, deviation is {}",
                                instruction.offset
                            ))?);
                        }
                        list.reverse();
                        exprs_stack.push(ExpressionEnum::Container(Container {
                            values: list,
                            container_type: ContainerType::List,
                            start_offset: instruction.offset,
                            end_offset: instruction.offset,
                            ..Default::default()
                        }));
                    }
                }
                Opcode::ListExtend => {
                    let size = instruction.arg.ok_or(format!(
                        "[ListExtend] No arg, deviation is {}",
                        instruction.offset
                    ))?;
                    let mut extend = Vec::with_capacity(size);
                    for _ in 0..size {
                        extend.push(exprs_stack.pop().ok_or(format!(
                            "[ListExtend] Stack is empty, deviation is {}",
                            instruction.offset
                        ))?);
                    }
                    extend.reverse();
                    let list = exprs_stack.pop().ok_or(format!(
                        "[ListExtend] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    if let ExpressionEnum::Container(Container {
                        values: mut list,
                        container_type: ContainerType::List,
                        ..
                    }) = list
                    {
                        extend.iter_mut().for_each(|x| {
                            if let ExpressionEnum::BaseValue(value) = x {
                                // deprecated trim_start_matches('\'').trim_end_matches('\'')
                                // because it may trim more than one matches
                                // e.g. "((1, 2))" -> "1, 2"
                                value.value = Regex::new(r"\((.*)\)")
                                    .unwrap()
                                    .captures(&value.value)
                                    .unwrap()
                                    .get(1)
                                    .unwrap()
                                    .as_str()
                                    .to_string();
                            }
                        });
                        list.append(&mut extend);
                        exprs_stack.push(ExpressionEnum::Container(Container {
                            values: list,
                            container_type: ContainerType::List,
                            start_offset: instruction.offset,
                            end_offset: instruction.offset,
                            ..Default::default()
                        }));
                    } else {
                        return Err("[ListExtend] Invalid list".into());
                    }
                }
                Opcode::BuildSet => {
                    let size = instruction.arg.ok_or(format!(
                        "[BuildSet] No arg, deviation is {}",
                        instruction.offset
                    ))?;
                    let mut set = Vec::with_capacity(size);
                    for _ in 0..size {
                        set.push(exprs_stack.pop().ok_or(format!(
                            "[BuildSet] Stack is empty, deviation is {}",
                            instruction.offset
                        ))?);
                    }
                    set.reverse();
                    exprs_stack.push(ExpressionEnum::Container(Container {
                        values: set,
                        container_type: ContainerType::Set,
                        start_offset: instruction.offset,
                        end_offset: instruction.offset,
                        ..Default::default()
                    }));
                }
                Opcode::BuildMap => {
                    let size = instruction.arg.ok_or(format!(
                        "[BuildMap] No arg, deviation is {}",
                        instruction.offset
                    ))?;
                    if size == 0 {
                        exprs_stack.push(ExpressionEnum::Container(Container {
                            values: vec![],
                            container_type: ContainerType::Dict,
                            start_offset: instruction.offset,
                            end_offset: instruction.offset,
                            ..Default::default()
                        }));
                    } else {
                        let mut map = Vec::with_capacity(size * 2);
                        for _ in 0..size {
                            let value = exprs_stack.pop().ok_or(format!(
                                "[BuildMap] Stack is empty, deviation is {}",
                                instruction.offset
                            ))?;
                            let key = exprs_stack.pop().ok_or(format!(
                                "[BuildMap] Stack is empty, deviation is {}",
                                instruction.offset
                            ))?;
                            map.push(value);
                            map.push(key);
                        }
                        map.reverse();
                        exprs_stack.push(ExpressionEnum::Container(Container {
                            values: map,
                            container_type: ContainerType::Dict,
                            start_offset: instruction.offset,
                            end_offset: instruction.offset,
                            ..Default::default()
                        }));
                    }
                }
                Opcode::BuildConstKeyMap => {
                    let size = instruction.arg.ok_or(format!(
                        "[BuildConstKeyMap] No arg, deviation is {}",
                        instruction.offset
                    ))?;
                    let keys = exprs_stack.pop().ok_or(format!(
                        "[BuildConstKeyMap] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    let mut values = Vec::with_capacity(size);
                    for _ in 0..size {
                        values.push(exprs_stack.pop().ok_or(format!(
                            "[BuildConstKeyMap] Stack is empty, deviation is {}",
                            instruction.offset
                        ))?);
                    }
                    values.reverse();
                    let mut map = Vec::with_capacity(size * 2);
                    if let ExpressionEnum::BaseValue(BaseValue { value: key, .. }) = keys {
                        for (k, v) in key
                            .trim_start_matches('(')
                            .trim_end_matches(')')
                            .split(", ")
                            .zip(values)
                        {
                            map.push(ExpressionEnum::BaseValue(BaseValue {
                                value: k.to_string(),
                                ..Default::default()
                            }));
                            map.push(v);
                        }
                    }
                    exprs_stack.push(ExpressionEnum::Container(Container {
                        values: map,
                        container_type: ContainerType::Dict,
                        ..Default::default()
                    }))
                }
                Opcode::BuildSlice => {
                    let size = instruction.arg.ok_or(format!(
                        "[BuildSlice] No arg, deviation is {}",
                        instruction.offset
                    ))?;
                    let mut slice = Vec::with_capacity(size);
                    for _ in 0..size {
                        slice.push(exprs_stack.pop().ok_or(format!(
                            "[BuildSlice] Stack is empty, deviation is {}",
                            instruction.offset
                        ))?);
                    }
                    slice.reverse();
                    let origin = exprs_stack.pop().ok_or(format!(
                        "[BuildSlice] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    exprs_stack.push(ExpressionEnum::Slice(Slice {
                        origin: Box::new(origin),
                        slice,
                        start_offset: instruction.offset,
                        end_offset: instruction.offset,
                        ..Default::default()
                    }));
                }
                Opcode::MakeFunction => {
                    let mark = exprs_stack.pop().ok_or(format!(
                        "[MakeFunction] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    let mut function = Function::from(mark)?;
                    if let Some(argval) = instruction.argval.as_ref() {
                        if argval.contains("annotations") {
                            let values = exprs_stack.pop().ok_or(format!(
                                "[MakeFunction] Stack is empty, deviation is {}",
                                instruction.offset
                            ))?;
                            if let ExpressionEnum::Container(container) = values {
                                #[cfg(debug_assertions)]
                                {
                                    assert_eq!(container.container_type, ContainerType::Tuple);
                                }

                                for (idx, exprs) in container.values.chunks(2).enumerate() {
                                    if exprs.iter().all(|e| e.is_base_value()) {
                                        let arg = FastVariable {
                                            index: idx,
                                            name: exprs
                                                .first()
                                                .unwrap()
                                                .unwrap_base_value()
                                                .value
                                                .trim_start_matches('\'')
                                                .trim_end_matches('\'')
                                                .to_string(),
                                            annotation: Some(
                                                exprs
                                                    .get(1)
                                                    .as_ref()
                                                    .unwrap()
                                                    .unwrap_base_value()
                                                    .value,
                                            ),
                                            ..Default::default()
                                        };
                                        function.args.push(arg);
                                    }
                                }
                            }
                        }
                        if argval.contains("defaults") {
                            let defaults = exprs_stack.pop().ok_or(format!(
                                "[MakeFunction] Stack is empty, deviation is {}",
                                instruction.offset
                            ))?;
                            if let ExpressionEnum::BaseValue(BaseValue { value, .. }) = defaults {
                                let defaults = value
                                    .trim_start_matches('(')
                                    .trim_end_matches(')')
                                    .trim_end_matches(',')
                                    .split(", ")
                                    .map(|x| x.to_string())
                                    .collect::<Vec<_>>();
                                function.defaults = defaults;
                            }
                        }
                    }
                    function.start_offset = instruction.offset;
                    function.end_offset = instruction.offset;
                    exprs_stack.push(ExpressionEnum::Function(function));
                }
                // BinaryOperation
                Opcode::BinaryOp | Opcode::CompareOp => {
                    let right = exprs_stack.pop().ok_or(format!(
                        "[BinaryOp] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    let left = exprs_stack.pop().ok_or(format!(
                        "[BinaryOp] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    exprs_stack.push(ExpressionEnum::BinaryOperation(BinaryOperation {
                        left: Box::new(left),
                        right: Box::new(right),
                        operator: instruction
                            .argval
                            .as_ref()
                            .ok_or("[BinaryOp] No argval")?
                            .clone(),
                        start_offset: instruction.offset,
                        end_offset: instruction.offset,
                        ..Default::default()
                    }))
                }
                // BinaryOperation
                Opcode::IsOp => {
                    let right = exprs_stack.pop().ok_or(format!(
                        "[IsOp] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    let left = exprs_stack.pop().ok_or(format!(
                        "[IsOp] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    let operator = match instruction.arg.as_ref() {
                        Some(0) => "is",
                        Some(1) => "is not",
                        _ => return Err("[IsOp] No arg or Invalid arg".into()),
                    };
                    exprs_stack.push(ExpressionEnum::BinaryOperation(BinaryOperation {
                        left: Box::new(left),
                        right: Box::new(right),
                        operator: operator.to_string(),
                        start_offset: instruction.offset,
                        end_offset: instruction.offset,
                        ..Default::default()
                    }))
                }
                // BinaryOperation
                Opcode::ContainsOp => {
                    let right = exprs_stack.pop().ok_or(format!(
                        "[ContainsOp] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    let left = exprs_stack.pop().ok_or(format!(
                        "[ContainsOp] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    let operator = match instruction.arg.as_ref() {
                        Some(0) => "in",
                        Some(1) => "not in",
                        _ => return Err("[ContainsOp] No arg or Invalid arg".into()),
                    };
                    exprs_stack.push(ExpressionEnum::BinaryOperation(BinaryOperation {
                        left: Box::new(left),
                        right: Box::new(right),
                        operator: operator.to_string(),
                        start_offset: instruction.offset,
                        end_offset: instruction.offset,
                        ..Default::default()
                    }))
                }
                Opcode::BinarySubscr => {
                    let index = exprs_stack.pop().ok_or(format!(
                        "[BinarySubscr] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    let target = exprs_stack.pop().ok_or(format!(
                        "[BinarySubscr] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    exprs_stack.push(ExpressionEnum::Subscr(Subscr {
                        target: Box::new(target),
                        index: Box::new(index),
                        start_offset: instruction.offset,
                        end_offset: instruction.offset,
                        ..Default::default()
                    }));
                }
                Opcode::UnaryInvert => {
                    let target = exprs_stack.pop().ok_or(format!(
                        "[UnaryInvert] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    exprs_stack.push(ExpressionEnum::UnaryOperation(UnaryOperation {
                        target: Box::new(target),
                        unary_type: UnaryType::Invert,
                        start_offset: instruction.offset,
                        end_offset: instruction.offset,
                        ..Default::default()
                    }))
                }
                Opcode::UnaryNegative => {
                    let target = exprs_stack.pop().ok_or(format!(
                        "[UnaryNegative] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    exprs_stack.push(ExpressionEnum::UnaryOperation(UnaryOperation {
                        target: Box::new(target),
                        unary_type: UnaryType::Negative,
                        start_offset: instruction.offset,
                        end_offset: instruction.offset,
                        ..Default::default()
                    }))
                }
                Opcode::UnaryNot => {
                    let target = exprs_stack.pop().ok_or(format!(
                        "[UnaryNot] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    exprs_stack.push(ExpressionEnum::UnaryOperation(UnaryOperation {
                        target: Box::new(target),
                        unary_type: UnaryType::Not,
                        start_offset: instruction.offset,
                        end_offset: instruction.offset,
                        ..Default::default()
                    }))
                }
                Opcode::Call => {
                    #[cfg(debug_assertions)]
                    {
                        //dbg!(&exprs_stack);
                    }

                    let count = instruction.arg.ok_or(format!(
                        "[Call] No arg, deviation is {}",
                        instruction.offset
                    ))?;
                    if count == 0 {
                        let last = exprs_stack.pop().ok_or(format!(
                            "[Call] Stack is empty, deviation is {}",
                            instruction.offset
                        ))?;
                        if let ExpressionEnum::BaseValue(base_value) = &last {
                            if base_value.value.contains(' ') {
                                // not a function call
                                exprs_stack.push(last);
                            } else {
                                exprs_stack.push(ExpressionEnum::Call(Call {
                                    func: Box::new(last),
                                    args: vec![],
                                    start_offset: instruction.offset,
                                    end_offset: instruction.offset,
                                    ..Default::default()
                                }))
                            }
                        } else {
                            exprs_stack.push(ExpressionEnum::Call(Call {
                                func: Box::new(last),
                                args: vec![],
                                start_offset: instruction.offset,
                                end_offset: instruction.offset,
                                ..Default::default()
                            }));
                        }
                        offset += 1;
                        continue;
                    }
                    let mut args = Vec::with_capacity(count);
                    for _ in 0..count {
                        args.push(exprs_stack.pop().ok_or(format!(
                            "[Call] Stack is empty, deviation is {}",
                            instruction.offset
                        ))?);
                    }
                    args.reverse();
                    match exprs_stack.pop() {
                        Some(ExpressionEnum::BaseValue(function_name)) => {
                            //dbg!(&function_name);
                            let function_name = function_name.value.trim_start_matches("NULL + ");
                            exprs_stack.push(ExpressionEnum::Call(Call {
                                func: Box::new(ExpressionEnum::BaseValue(BaseValue {
                                    value: function_name.to_string(),
                                    ..Default::default()
                                })),
                                args,
                                start_line: instruction.starts_line.unwrap_or_default(),
                                start_offset: instruction.offset,
                                end_offset: instruction.offset,
                            }))
                        }
                        Some(function) => exprs_stack.push(ExpressionEnum::Call(Call {
                            func: Box::new(function),
                            args,
                            start_line: instruction.starts_line.unwrap_or_default(),
                            start_offset: instruction.offset,
                            end_offset: instruction.offset,
                        })),
                        None => return Err("[Call] Stack is empty".into()),
                    }
                }
                Opcode::ReturnValue => {
                    //dbg!(&exprs_stack);
                    let value = exprs_stack.pop().ok_or(format!(
                        "[ReturnValue] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    exprs_stack.push(ExpressionEnum::Return(Return {
                        value: Box::new(value),
                        start_line: instruction.starts_line.unwrap_or_default(),
                        start_offset: instruction.offset,
                        end_offset: instruction.offset,
                    }));
                }
                Opcode::YieldValue => {
                    let value = exprs_stack.pop().ok_or(format!(
                        "[YieldValue] Stack is empty, deviation is {}",
                        instruction.offset,
                    ))?;
                    exprs_stack.push(ExpressionEnum::Yield(Yield {
                        value: Box::new(value),
                        start_line: instruction.starts_line.unwrap_or_default(),
                        start_offset: instruction.offset,
                        end_offset: instruction.offset,
                    }))
                }
                Opcode::ImportFrom => {
                    let value = exprs_stack.pop().ok_or(format!(
                        "[ImportFrom] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    if let ExpressionEnum::Import(import) = value {
                        if import.bk_module.is_none() {
                            //没from
                            import
                                .with_mut_unchecked()
                                .patch_by(|mut i| i.fragment = None)
                                .unwrap();
                            exprs_stack.push(ExpressionEnum::Import(import))
                        } else {
                            //有from
                            let new_fragment = Some(
                                instruction
                                    .argval
                                    .as_ref()
                                    .ok_or(format!(
                                        "[ImportName] No argval, deviation is {}",
                                        instruction.offset
                                    ))?
                                    .clone(),
                            );
                            import
                                .with_mut_unchecked()
                                .patch_by(|mut i| i.fragment = new_fragment)
                                .unwrap();
                            exprs_stack.push(ExpressionEnum::Import(import))
                        }
                    }
                }
                Opcode::ImportName => {
                    let module = exprs_stack.pop().ok_or(format!(
                        "[ImportName] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;

                    // remove the "0"
                    // when import a module, it will load a const "0" first
                    if let Some(ExpressionEnum::BaseValue(base_value)) = exprs_stack.last() {
                        if base_value.value == "0" {
                            exprs_stack.pop();
                        }
                    }

                    if !module.build()?.join("").is_empty() {
                        //需要from
                        let origin_len = module.build()?.join("").len();
                        if origin_len                  //有“ * ”
                        - module
                            .build()?
                            .join("")
                            .replace('*', "")
                            .len()
                            == 1
                        {
                            exprs_stack.push(ExpressionEnum::Import(Import {
                                module: instruction
                                    .argval
                                    .as_ref()
                                    .ok_or(format!(
                                        "[ImportName] No argval, deviation is {}",
                                        instruction.offset
                                    ))?
                                    .clone(),
                                bk_module: Some('*'.to_string()),
                                fragment: None,
                                alias: None,
                                start_line: instruction.starts_line.unwrap_or_default(),
                                start_offset: instruction.offset,
                                end_offset: instruction.offset,
                            }))
                        } else {
                            //没 “ * ”
                            exprs_stack.push(ExpressionEnum::Import(Import {
                                module: instruction
                                    .argval
                                    .as_ref()
                                    .ok_or(format!(
                                        "[ImportName] No argval, deviation is {}",
                                        instruction.offset
                                    ))?
                                    .clone(),
                                bk_module: Some("".to_string()),
                                fragment: None,
                                alias: None,
                                start_line: instruction.starts_line.unwrap_or_default(),
                                start_offset: instruction.offset,
                                end_offset: instruction.offset,
                            }))
                        }
                    } else {
                        //不需要from
                        exprs_stack.push(ExpressionEnum::Import(Import {
                            module: instruction
                                .argval
                                .as_ref()
                                .ok_or(format!(
                                    "[ImportName] No argval, deviation is {}",
                                    instruction.offset
                                ))?
                                .clone(),
                            bk_module: None,
                            fragment: None,
                            alias: None,
                            start_line: instruction.starts_line.unwrap_or_default(),
                            start_offset: instruction.offset,
                            end_offset: instruction.offset,
                        }))
                    }
                }
                Opcode::PopJumpIfTrue => {
                    if let Some(next_instruction) = opcode_instructions.get(offset + 1) {
                        // if next instruction is LoadAssertionError, then it's an assert
                        if next_instruction.opcode() == Opcode::LoadAssertionError {
                            offset += 1;
                            continue;
                        }
                    }

                    let test = exprs_stack.pop().ok_or(format!(
                        "[PopJumpIfTrue] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    let test = ExpressionEnum::UnaryOperation(UnaryOperation {
                        target: Box::new(test),
                        unary_type: UnaryType::Not,
                        ..Default::default()
                    });

                    exprs_stack.push(test);

                    // change now opcode to PopJumpIfFalse, and rollback offset
                    opcode_instructions[offset]
                        .opcode
                        .replace(Opcode::PopJumpIfFalse);

                    offset -= 1;
                }
                Opcode::PopJumpIfFalse => {
                    if let Some(next_instruction) = opcode_instructions.get(offset + 1) {
                        // if next instruction is LoadAssertionError, then it's an assert
                        if next_instruction.opcode() == Opcode::LoadAssertionError {
                            offset += 1;
                            continue;
                        }
                    }

                    let test = exprs_stack.pop().ok_or(format!(
                        "[PopJumpIfFalse] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    // if test false, then jump to target
                    // now offset + 1 to target is this branch block
                    let jump_target = instruction
                        .argval
                        .as_ref()
                        .ok_or(format!(
                            "[PopJumpIfFalse] No argval, deviation is {}",
                            instruction.offset
                        ))?
                        .trim_start_matches("to ")
                        .parse::<usize>()?;

                    // judge whether the if-expr have multiple test
                    // if the jump_target points to the previous opcode is PopJumpIfFalse or similar
                    // then it may have multiple test
                    // and then need to use 'or' / 'and' to combine them
                    // and update the jump target to the end of the block
                    let jump_target_idx = opcode_instructions
                        .iter()
                        .position(|x| x.offset == jump_target)
                        .ok_or(format!(
                            "[PopJumpIfFalse] No jump target {}, deviation is {}",
                            jump_target, instruction.offset
                        ))?;
                    let prev_instruction =
                        opcode_instructions.get(jump_target_idx - 1).ok_or(format!(
                            "[PopJumpIfFalse] No prev instruction, deviation is {}",
                            instruction.offset
                        ))?;
                    let pop_jump = [
                        Opcode::PopJumpIfFalse,
                        Opcode::PopJumpIfTrue,
                        Opcode::PopJumpIfNone,
                        Opcode::PopJumpIfNotNone,
                    ];
                    let (test, block_end_idx) = if pop_jump.contains(&prev_instruction.opcode()) {
                        // if prev opcode is contained in pop_jump
                        // it may have multiple test
                        // and then we need to get all test
                        //
                        // position the last test
                        // iterate from this, get the jump target each PopJump opcode
                        // and set the iterate end when the jump target previous opcode is not PopJump opcode
                        // that means the last test (position is the offset which is the PopJump opcode, not jump target)
                        let mut _idx = offset + 1;
                        let mut last_test_idx = offset;
                        let mut block_end_idx = jump_target_idx;
                        let mut exprs_and_jumps = vec![(test.clone(), jump_target, offset)];
                        while let Some(prev_instruction) = opcode_instructions.get(_idx) {
                            if _idx >= block_end_idx {
                                break;
                            }

                            if pop_jump.contains(&prev_instruction.opcode()) {
                                let this_test_instructions =
                                    &opcode_instructions[last_test_idx + 1.._idx];

                                let mut this_test_expr = Self::parse(this_test_instructions)?.bodys;
                                assert!(this_test_expr.len() == 1);
                                let mut this_test_expr = this_test_expr.pop().unwrap();
                                let this_jump_target = prev_instruction
                                    .argval
                                    .as_ref()
                                    .ok_or(format!(
                                        "[PopJumpIfFalse] No argval, deviation is {}",
                                        instruction.offset
                                    ))?
                                    .trim_start_matches("to ")
                                    .parse::<usize>()?;

                                this_test_expr.set_offset(
                                    opcode_instructions[exprs_and_jumps.last().unwrap().2 + 1]
                                        .offset,
                                    opcode_instructions[_idx].offset,
                                );
                                exprs_and_jumps.push((this_test_expr, this_jump_target, _idx));

                                last_test_idx = _idx;
                                block_end_idx = opcode_instructions
                                    .iter()
                                    .position(|x| x.offset == this_jump_target)
                                    .ok_or(format!(
                                        "[PopJumpIfFalse] No jump target, deviation is {}",
                                        instruction.offset
                                    ))?;
                            }

                            _idx += 1;
                        }

                        // dbg!(&exprs_and_jumps);

                        let block_start_idx = exprs_and_jumps.last().unwrap().2 + 1;
                        // println!("block_start_idx: {}, block_end_idx: {}", block_start_idx, block_end_idx);
                        #[cfg(debug_assertions)]
                        {
                            assert!(
                                block_start_idx <= block_end_idx,
                                "block_start_idx: {}, offset: {}",
                                block_start_idx,
                                offset
                            );
                        }

                        offset = block_start_idx - 1;
                        // combine all test
                        let combined_test = exprs_and_jumps.first().unwrap().0.clone();

                        pub fn combine(
                            combined_test: ExpressionEnum,
                            opcode_instructions: &[OpcodeInstruction],
                            exprs_and_jumps: &[(ExpressionEnum, usize, usize)],
                            arr_start_idx: usize,
                            arr_end_idx: usize,
                            block_start_idx: usize,
                            block_end_idx: usize,
                        ) -> ExpressionEnum {
                            // this idx is the index of the exprs_and_jumps (not the opcode_instructions)
                            let this_idx = arr_start_idx;
                            let this_jump_target = exprs_and_jumps[this_idx].1;
                            // this _idx is the index of the exprs_and_jumps (not the opcode_instructions)
                            if let Some(_idx) = exprs_and_jumps
                                .iter()
                                .position(|x| x.0.get_offset().0 == this_jump_target)
                            {
                                // if find the jump target in the exprs_and_jumps

                                // cannot jump to the next test
                                // due to the next test may be run after this test
                                // it's unnecessary to jump
                                assert_ne!(
                                    this_idx + 1,
                                    _idx,
                                    "[PopJumpIfFalse] The next test is the jump target, which is not expected"
                                );

                                if this_idx + 2 == _idx {
                                    // if skip one test, then use 'and' to combine this test and the skip test
                                    // and use 'or' to combine the result with the next by the next test
                                    let next_test = exprs_and_jumps[this_idx + 1].0.clone();
                                    let combined_and_test =
                                        ExpressionEnum::BinaryOperation(BinaryOperation {
                                            left: Box::new(combined_test),
                                            right: Box::new(next_test),
                                            operator: "and".to_string(),
                                            start_offset: opcode_instructions[block_start_idx]
                                                .offset,
                                            end_offset: opcode_instructions[_idx].offset,
                                            ..Default::default()
                                        });

                                    // the next test of the next test
                                    let next_test = exprs_and_jumps[_idx].0.clone();
                                    ExpressionEnum::BinaryOperation(BinaryOperation {
                                        left: Box::new(combined_and_test),
                                        right: Box::new(next_test),
                                        operator: "or".to_string(),
                                        start_offset: opcode_instructions[block_start_idx].offset,
                                        end_offset: opcode_instructions[_idx].offset,
                                        ..Default::default()
                                    })
                                } else {
                                    // if skip more than one test
                                    // it means the previous test is 'and' with all of tests between this and the test pointed by _idx
                                    // and use 'or' to combine the result with the later test
                                    let first_and_test = combine(
                                        combined_test.clone(),
                                        opcode_instructions,
                                        exprs_and_jumps,
                                        this_idx,
                                        _idx - 1,
                                        block_start_idx,
                                        block_end_idx,
                                    );

                                    ExpressionEnum::BinaryOperation(BinaryOperation {
                                        left: Box::new(first_and_test),
                                        right: Box::new(combined_test),
                                        operator: "and".to_string(),
                                        start_offset: opcode_instructions[block_start_idx].offset,
                                        end_offset: opcode_instructions[_idx].offset,
                                        ..Default::default()
                                    })
                                }
                            } else if this_jump_target
                                == opcode_instructions[block_start_idx].offset
                            {
                                // if the jump target is the start of the block
                                // it means this test is the first test in the current exprs_and_jumps
                                // and is 'or' with all of later tests
                                let this_test = exprs_and_jumps[this_idx].0.clone();

                                let later_test_combined = combine(
                                    this_test.clone(),
                                    opcode_instructions,
                                    exprs_and_jumps,
                                    this_idx + 1,
                                    arr_end_idx,
                                    block_start_idx,
                                    block_end_idx,
                                );

                                ExpressionEnum::BinaryOperation(BinaryOperation {
                                    left: Box::new(this_test.clone()),
                                    right: Box::new(later_test_combined),
                                    operator: "or".to_string(),
                                    start_offset: opcode_instructions[block_start_idx].offset,
                                    end_offset: opcode_instructions[block_end_idx].offset,
                                    ..Default::default()
                                })
                            } else {
                                // jump to the end of the block
                                // it means this test is `and` with all of later tests
                                let this_test = exprs_and_jumps[this_idx].0.clone();

                                if this_idx + 1 == arr_end_idx {
                                    let next_test = exprs_and_jumps[this_idx + 1].0.clone();
                                    ExpressionEnum::BinaryOperation(BinaryOperation {
                                        left: Box::new(this_test.clone()),
                                        right: Box::new(next_test),
                                        operator: "and".to_string(),
                                        start_offset: opcode_instructions[block_start_idx].offset,
                                        end_offset: opcode_instructions[block_end_idx].offset,
                                        ..Default::default()
                                    })
                                } else {
                                    let later_test_combined = combine(
                                        this_test.clone(),
                                        opcode_instructions,
                                        exprs_and_jumps,
                                        this_idx + 1,
                                        arr_end_idx,
                                        block_start_idx,
                                        block_end_idx,
                                    );

                                    ExpressionEnum::BinaryOperation(BinaryOperation {
                                        left: Box::new(this_test),
                                        right: Box::new(later_test_combined),
                                        operator: "and".to_string(),
                                        start_offset: opcode_instructions[block_start_idx].offset,
                                        end_offset: opcode_instructions[block_end_idx].offset,
                                        ..Default::default()
                                    })
                                }
                            }
                        }

                        (
                            combine(
                                combined_test,
                                opcode_instructions,
                                &exprs_and_jumps,
                                0,
                                exprs_and_jumps.len() - 1,
                                block_start_idx,
                                block_end_idx,
                            ),
                            Some(block_end_idx),
                        )
                    } else {
                        // if the jump target previous opcode is not PopJump opcode
                        // then it's a single test
                        (test, None)
                    };

                    let mut sub_instructions = &opcode_instructions[offset + 1..];

                    let block_end_first_idx = if let Some(block_end_idx) = block_end_idx {
                        block_end_idx - offset - 1
                    } else {
                        // find <if> block end index in sub_instructions
                        // and will split sub_instructions at this index
                        let block_end_first_idx = sub_instructions
                            .iter()
                            .position(|x| x.offset == jump_target);

                        if let Some(block_end_first_idx) = block_end_first_idx {
                            block_end_first_idx
                        } else {
                            // if none, then it may be a nested if-expr
                            // sub-if mustn't jump further than parent-if
                            // so if the sub-if jump target is greater than parent-if jump target
                            // then we can use 'or' / 'and' to combine them
                            // and the jump target is the end of the parent-if block
                            //
                            // In now version, we will change the sub-if jump target to parent-if jump target
                            // In other words, the block_end_first_idx default is the end of the parent-if block
                            sub_instructions.len().checked_sub(1).unwrap_or_default()
                        }
                    };

                    // split the sub_instructions at block_end_first_idx
                    sub_instructions = &sub_instructions[..block_end_first_idx];
                    // dbg!(&sub_instructions);
                    let sub_expr = Self::parse(sub_instructions)?;
                    // if-expr of this branch
                    let mut if_expr = If {
                        test: Some(Box::new(test)),
                        body: sub_expr.bodys,
                        start_line: instruction.starts_line.unwrap_or_default(),
                        start_offset: instruction.offset,
                        end_offset: sub_instructions.last().unwrap().offset,
                        ..Default::default()
                    };

                    offset += block_end_first_idx;

                    /* dbg!(offset, opcode_instructions[offset].offset);
                    dbg!(&if_expr.body); */

                    // get jump target
                    // it is the next one of the last instruction of the block
                    let else_block_end_later = if let ExpressionEnum::Jump(this_block_jumps) =
                        if_expr.body.last().ok_or(format!(
                            "[PopJumpIfFalse] No last expr, deviation is {}",
                            instruction.offset
                        ))?
                        && !this_block_jumps.is_backward
                    {
                        this_block_jumps.target
                    } else if let ExpressionEnum::Return(_) = if_expr.body.last().unwrap() {
                        // if the last instruction is Return
                        // find the next jump or return as else block end
                        let else_block_end_later_idx = opcode_instructions
                            .iter()
                            .skip(offset + 1)
                            .position(|x| {
                                x.opcode() == Opcode::JumpForward
                                    || x.opcode() == Opcode::ReturnValue
                            })
                            .map(|x| x + offset + 1);
                        if let Some(else_block_end_later_idx) = else_block_end_later_idx {
                            opcode_instructions[else_block_end_later_idx].offset
                        } else {
                            // if not have jump target and Return in the block last
                            // it may only one branch (no elif/else)
                            // then the jump target is the end of this block
                            // build this block and continue
                            exprs_stack.push(ExpressionEnum::If(if_expr));

                            offset += 1;
                            continue;
                        }
                    } else {
                        // if not have jump target and Return in the block last
                        // it may only one branch (no elif/else)
                        // then the jump target is the end of this block
                        // build this block and continue
                        exprs_stack.push(ExpressionEnum::If(if_expr));

                        offset += 1;
                        continue;
                    };

                    // If find the jump target in the rest instructions, then it's an elif or else
                    // if not, that may be the end of parent if-expr
                    if let Some(else_block_end_idx) = opcode_instructions
                        .iter()
                        .position(|x| x.offset == else_block_end_later)
                    {
                        let remain_instructions =
                            &opcode_instructions[offset + 1..else_block_end_idx];

                        let remain_exprs = Self::parse(remain_instructions)?;
                        // dbg!(&remain_exprs.bodys);
                        let mut remain_exprs = remain_exprs.bodys.into_iter();
                        let remain_first_expr = remain_exprs.next();
                        if let Some(ExpressionEnum::If(mut remain_first_expr)) = remain_first_expr {
                            // if have unused exprs in remain_exprs, then it's an else
                            if remain_exprs.len() > 0 {
                                let else_expr = If {
                                    test: None,
                                    body: remain_exprs.collect(),
                                    start_line: remain_instructions
                                        .first()
                                        .unwrap()
                                        .starts_line
                                        .unwrap_or_default(),
                                    start_offset: remain_instructions.first().unwrap().offset,
                                    end_offset: remain_instructions.last().unwrap().offset,
                                    ..Default::default()
                                };
                                remain_first_expr.or_else =
                                    Some(Box::new(ExpressionEnum::If(else_expr)));
                            }

                            if_expr.or_else = Some(Box::new(ExpressionEnum::If(remain_first_expr)));
                        } else {
                            if_expr.or_else = Some(Box::new(ExpressionEnum::If(If {
                                test: None,
                                body: remain_first_expr.into_iter().chain(remain_exprs).collect(),
                                start_line: remain_instructions
                                    .first()
                                    .unwrap()
                                    .starts_line
                                    .unwrap_or_default(),
                                start_offset: remain_instructions.first().unwrap().offset,
                                end_offset: remain_instructions.last().unwrap().offset,
                                ..Default::default()
                            })));
                        }

                        offset = else_block_end_idx - 1;
                    }

                    // if the exprs_stack last expr is If, then it may be an elif/else
                    if let Some(ExpressionEnum::If(parent_if)) = exprs_stack.last_mut() {
                        if parent_if.or_else.is_none()
                            && parent_if.test.is_some()
                            && !parent_if.body.is_empty()
                            && (parent_if.body.last().unwrap().is_jump()
                                || parent_if.body.last().unwrap().is_return())
                        {
                            parent_if.or_else = Some(Box::new(ExpressionEnum::If(if_expr)));
                        } else {
                            // if the exprs_stack last expr is not If and body last not jump, then it's an if
                            exprs_stack.push(ExpressionEnum::If(if_expr));
                        }
                    } else {
                        // if the exprs_stack last expr is not If, then it's an if
                        exprs_stack.push(ExpressionEnum::If(if_expr));
                    }
                }
                Opcode::JumpForward => {
                    let jump_target = instruction
                        .argval
                        .as_ref()
                        .ok_or(format!(
                            "[JumpForward] No argval, deviation is {}",
                            instruction.offset
                        ))?
                        .trim_start_matches("to ")
                        .parse::<usize>()?;
                    exprs_stack.push(ExpressionEnum::Jump(Jump {
                        target: jump_target,
                        start_line: instruction.starts_line.unwrap_or_default(),
                        start_offset: instruction.offset,
                        end_offset: instruction.offset,
                        ..Default::default()
                    }));
                }
                Opcode::JumpBackward => {
                    let jump_target = instruction
                        .argval
                        .as_ref()
                        .ok_or(format!(
                            "[JumpBackward] No argval, deviation is {}",
                            instruction.offset
                        ))?
                        .trim_start_matches("to ")
                        .parse::<usize>()?;
                    exprs_stack.push(ExpressionEnum::Jump(Jump {
                        target: jump_target,
                        is_backward: true,
                        start_line: instruction.starts_line.unwrap_or_default(),
                        start_offset: instruction.offset,
                        end_offset: instruction.offset,
                    }));
                }
                Opcode::LoadAssertionError => {
                    let test = exprs_stack.pop().ok_or(format!(
                        "[LoadAssertionError] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;

                    exprs_stack.push(ExpressionEnum::Assert(Assert {
                        test: Box::new(test),
                        msg: None,
                        start_line: instruction.starts_line.unwrap_or_default(),
                        start_offset: instruction.offset,
                        end_offset: instruction.offset,
                    }))
                }
                Opcode::RaiseVarargs => {
                    let expr = exprs_stack.pop().ok_or(format!(
                        "[RaiseVarargs] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    if expr.is_base_value() {
                        let exception = expr;
                        if let Some(expr) = exprs_stack.pop() {
                            let mut has_assert = false;
                            if let Ok(assert) = expr.query_singleton::<Assert>() {
                                has_assert = true;
                                assert
                                    .with_mut_unchecked()
                                    .patch_by(|mut a| a.msg = Some(Box::new(exception.clone())))?;
                            }
                            exprs_stack.push(expr);

                            // if hasn't assert, then it just a raise
                            if !has_assert {
                                exprs_stack.push(ExpressionEnum::Raise(Raise {
                                    exception: Box::new(exception),
                                    start_line: instruction.starts_line.unwrap_or_default(),
                                    start_offset: instruction.offset,
                                    end_offset: instruction.offset,
                                }))
                            }
                        } else {
                            exprs_stack.push(ExpressionEnum::Raise(Raise {
                                exception: Box::new(exception),
                                start_line: instruction.starts_line.unwrap_or_default(),
                                start_offset: instruction.offset,
                                end_offset: instruction.offset,
                            }))
                        }
                    } else {
                        #[cfg(debug_assertions)]
                        {
                            assert!(expr.is_assert());
                        }
                        //
                    }
                }
                Opcode::CheckExcMatch => {
                    let err = exprs_stack.pop().ok_or(format!(
                        "[CheckExcMatch] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    if let Some(store_name_instruction_idx) = opcode_instructions[offset..]
                        .iter()
                        .position(|x| x.opcode() == Opcode::StoreName)
                    {
                        let store_name_instruction =
                            &opcode_instructions[offset + store_name_instruction_idx];
                        let alias = store_name_instruction.argval.as_ref().ok_or(format!(
                            "[CheckExcMatch] No argval, deviation is {}",
                            instruction.offset
                        ))?;

                        offset += store_name_instruction_idx;

                        exprs_stack.push(ExpressionEnum::Except(Except {
                            exception: Box::new(ExpressionEnum::Alias(Alias {
                                target: Box::new(err),
                                alias: Box::new(ExpressionEnum::BaseValue(BaseValue {
                                    value: alias.clone(),
                                    ..Default::default()
                                })),
                                ..Default::default()
                            })),
                            body: vec![],
                            start_line: instruction.starts_line.unwrap_or_default(),
                            start_offset: instruction.offset,
                            end_offset: instruction.offset,
                        }));
                    } else {
                        exprs_stack.push(ExpressionEnum::Except(Except {
                            exception: Box::new(err),
                            body: vec![],
                            start_line: instruction.starts_line.unwrap_or_default(),
                            start_offset: instruction.offset,
                            end_offset: instruction.offset,
                        }))
                    }
                }
                Opcode::BeforeWith => {
                    let expr = exprs_stack.pop().ok_or(format!(
                        "[BeforeWith] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    let mut with = With {
                        start_line: instruction.starts_line.unwrap_or_default(),
                        start_offset: instruction.offset,
                        ..Default::default()
                    };
                    if let Some(next_instruction) = opcode_instructions.get(offset + 1) {
                        if next_instruction.opcode() == Opcode::StoreName
                            || next_instruction.opcode() == Opcode::StoreFast
                        {
                            let name = next_instruction.argval.as_ref().ok_or(format!(
                                "[BeforeWith] No argval, deviation is {}",
                                instruction.offset
                            ))?;
                            with.item = Box::new(ExpressionEnum::Alias(Alias {
                                target: Box::new(expr),
                                alias: Box::new(ExpressionEnum::BaseValue(BaseValue {
                                    value: name.clone(),
                                    ..Default::default()
                                })),
                                ..Default::default()
                            }));
                        } else {
                            with.item = Box::new(expr);
                        }
                    }
                    if let Some(next_instruction) = opcode_instructions.get(offset + 2) {
                        with.end_offset = next_instruction.offset;
                    }

                    // get with block
                    let sub_instructions = &opcode_instructions[offset + 2..];
                    #[cfg(debug_assertions)]
                    {
                        //dbg!(&sub_instructions);
                    }
                    let block_end_idxs = sub_instructions
                        .iter()
                        .enumerate()
                        .filter(|(_, x)| x.starts_line == Some(with.start_line))
                        .map(|(i, _)| i)
                        .collect::<Vec<_>>();
                    let block_end_first_idx = *block_end_idxs.first().ok_or(format!(
                        "[BeforeWith] No block end, deviation is {}",
                        instruction.offset
                    ))?;
                    let block_end_last_idx = *block_end_idxs.last().unwrap();
                    let sub_instructions =
                        &opcode_instructions[offset + 2..offset + 2 + block_end_first_idx];
                    #[cfg(debug_assertions)]
                    {
                        //dbg!(&sub_instructions);
                    }
                    let sub_expr = Self::parse(sub_instructions)?;
                    with.body = sub_expr.bodys;
                    // skip the offset to the end of with block
                    offset += 2 + block_end_last_idx;

                    exprs_stack.push(ExpressionEnum::With(with));
                }
                Opcode::BeforeAsyncWith => {
                    let expr = exprs_stack.pop().ok_or(format!(
                        "[BeforeWith] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    let mut async_with = With {
                        start_line: instruction.starts_line.unwrap_or_default(),
                        start_offset: instruction.offset,
                        is_async: true,
                        ..Default::default()
                    };

                    // find SEND instruction
                    let mut send_to = 0;
                    while let Some(next_instruction) = opcode_instructions.get(offset + 1) {
                        if next_instruction.opcode() == Opcode::Send {
                            offset += 1;
                            send_to = next_instruction
                                .argval
                                .as_ref()
                                .ok_or(format!(
                                    "[BeforeAsyncWith] No argval, deviation is {}",
                                    next_instruction.offset
                                ))?
                                .trim_start_matches("to ")
                                .parse::<usize>()?;
                            break;
                        }
                        offset += 1;
                    }

                    // skip the offset to the <send_to>
                    while let Some(next_instruction) = opcode_instructions.get(offset + 1) {
                        match next_instruction.offset.cmp(&send_to) {
                            Ordering::Equal => break,
                            Ordering::Greater => {
                                return Err(format!(
                                    "[BeforeAsyncWith] Send target not found, deviation is {}",
                                    next_instruction.offset
                                )
                                .into());
                            }
                            _ => {}
                        }
                        offset += 1;
                    }

                    // check has alias
                    if let Some(next_instruction) = opcode_instructions.get(offset + 1) {
                        if next_instruction.opcode() == Opcode::StoreName
                            || next_instruction.opcode() == Opcode::StoreFast
                        {
                            let name = next_instruction.argval.as_ref().ok_or(format!(
                                "[BeforeAsyncWith] No argval, deviation is {}",
                                instruction.offset
                            ))?;
                            async_with.item = Box::new(ExpressionEnum::Alias(Alias {
                                target: Box::new(expr),
                                alias: Box::new(ExpressionEnum::BaseValue(BaseValue {
                                    value: name.clone(),
                                    ..Default::default()
                                })),
                                ..Default::default()
                            }));
                        } else {
                            async_with.item = Box::new(expr);
                        }
                    }
                    if let Some(next_instruction) = opcode_instructions.get(offset + 2) {
                        async_with.end_offset = next_instruction.offset;
                    }

                    // get <with> block
                    let sub_instructions = &opcode_instructions[offset + 2..];
                    let block_end_idxs = sub_instructions
                        .iter()
                        .enumerate()
                        .filter(|(_, x)| x.starts_line == Some(async_with.start_line))
                        .map(|(i, _)| i)
                        .collect::<Vec<_>>();
                    let block_end_first_idx = *block_end_idxs.first().ok_or(format!(
                        "[BeforeAsyncWith] No block end, deviation is {}",
                        instruction.offset
                    ))?;
                    let block_end_last_idx = *block_end_idxs.last().unwrap();
                    let sub_instructions =
                        &opcode_instructions[offset + 2..offset + 2 + block_end_first_idx];
                    #[cfg(debug_assertions)]
                    {
                        //dbg!(&sub_instructions);
                    }
                    let sub_expr = Self::parse(sub_instructions)?;
                    async_with.body = sub_expr.bodys;
                    // skip the offset to the end of with block
                    offset += 2 + block_end_last_idx;

                    exprs_stack.push(ExpressionEnum::With(async_with));
                }
                Opcode::ForIter => {
                    let iter = exprs_stack.pop().ok_or(format!(
                        "[ForIter] Stack is empty, deviation is {}",
                        instruction.offset,
                    ))?;
                    let jump_target = instruction
                        .argval
                        .as_ref()
                        .ok_or(format!(
                            "[ForIter] No argval, deviation is {}",
                            instruction.offset
                        ))?
                        .split("to ")
                        .last()
                        .ok_or(format!(
                            "[ForIter] Invalid argval, expect \"to /* usize */\", but got {:?}",
                            instruction.argval.as_ref().unwrap()
                        ))?
                        .parse::<usize>()?;
                    if let Some(next_instruction) = opcode_instructions.get(offset + 1) {
                        if next_instruction.opcode() == Opcode::StoreName
                            || next_instruction.opcode() == Opcode::StoreFast
                        {
                            exprs_stack.push(ExpressionEnum::For(For {
                                iterator: Box::new(iter),
                                items: Box::new(ExpressionEnum::BaseValue(BaseValue {
                                    value: next_instruction
                                        .argval
                                        .as_ref()
                                        .ok_or(format!(
                                            "[ForIter] No argval, deviation is {}",
                                            next_instruction.offset
                                        ))?
                                        .clone(),
                                    ..Default::default()
                                })),
                                from: instruction.offset,
                                to: jump_target,
                                start_offset: instruction.offset,
                                end_offset: instruction.offset,
                                ..Default::default()
                            }));
                            offset += 1;
                        } else {
                            exprs_stack.push(ExpressionEnum::For(For {
                                iterator: Box::new(iter),
                                from: instruction.offset,
                                to: jump_target,
                                start_offset: instruction.offset,
                                end_offset: instruction.offset,
                                ..Default::default()
                            }))
                        }
                    } else {
                        return Err(format!(
                            "[ForIter] No next instruction, deviation is {}",
                            instruction.offset,
                        )
                        .into());
                    }
                }
                Opcode::GetAiter => {
                    let aiter = exprs_stack.pop().ok_or(format!(
                        "[GetAiter] Stack is empty, deviation is {}",
                        instruction.offset,
                    ))?;

                    // find SEND instruction
                    let mut send_to = 0;
                    while let Some(next_instruction) = opcode_instructions.get(offset + 1) {
                        if next_instruction.opcode() == Opcode::Send {
                            offset += 1;
                            send_to = next_instruction
                                .argval
                                .as_ref()
                                .ok_or(format!(
                                    "[GetAiter] No argval, deviation is {}",
                                    next_instruction.offset
                                ))?
                                .trim_start_matches("to ")
                                .parse::<usize>()?;
                            break;
                        }
                        offset += 1;
                    }

                    // skip the offset to the <send_to>
                    while let Some(next_instruction) = opcode_instructions.get(offset + 1) {
                        match next_instruction.offset.cmp(&send_to) {
                            Ordering::Equal => break,
                            Ordering::Greater => {
                                return Err(format!(
                                    "[GetAiter] Send target not found, deviation is {}",
                                    next_instruction.offset
                                )
                                .into())
                            }
                            _ => {}
                        }
                        offset += 1;
                    }

                    if let Some(next_instruction) = opcode_instructions.get(offset + 1) {
                        if next_instruction.opcode() == Opcode::StoreName
                            || next_instruction.opcode() == Opcode::StoreFast
                        {
                            exprs_stack.push(ExpressionEnum::For(For {
                                iterator: Box::new(aiter),
                                items: Box::new(ExpressionEnum::BaseValue(BaseValue {
                                    value: next_instruction
                                        .argval
                                        .as_ref()
                                        .ok_or(format!(
                                            "[GetAiter] No argval, deviation is {}",
                                            next_instruction.offset
                                        ))?
                                        .clone(),
                                    ..Default::default()
                                })),
                                from: instruction.offset,
                                is_async: true,
                                start_line: instruction.starts_line.unwrap_or_default(),
                                start_offset: instruction.offset,
                                end_offset: instruction.offset,
                                ..Default::default()
                            }));
                            offset += 1;
                        } else {
                            exprs_stack.push(ExpressionEnum::For(For {
                                iterator: Box::new(aiter),
                                from: instruction.offset,
                                is_async: true,
                                start_line: instruction.starts_line.unwrap_or_default(),
                                start_offset: instruction.offset,
                                end_offset: instruction.offset,
                                ..Default::default()
                            }))
                        }
                    } else {
                        return Err(format!(
                            "[GetAiter] No next instruction, deviation is {}",
                            instruction.offset,
                        )
                        .into());
                    }
                }
                Opcode::EndAsyncFor => {
                    let mut async_for_block = vec![];
                    while let Some(expr) = exprs_stack.pop() {
                        if let ExpressionEnum::For(async_for) = expr {
                            async_for_block.reverse();
                            async_for.with_mut_unchecked().patch_by(|mut f| {
                                f.body = async_for_block;
                            })?;
                            exprs_stack.push(ExpressionEnum::For(async_for));
                            break;
                        } else {
                            async_for_block.push(expr);
                        }
                    }
                }
                /* Opcode::ReturnGenerator => {
                    // mark this block is async
                    traceback.mark_async();
                } */
                Opcode::GetAwaitable => {
                    let awaitable_expr = exprs_stack.pop().ok_or(format!(
                        "[GetAwaitable] Stack is empty, deviation is {}",
                        instruction.offset
                    ))?;
                    exprs_stack.push(ExpressionEnum::Await(Await {
                        awaitable_expr: Box::new(awaitable_expr),
                        start_line: instruction.starts_line.unwrap_or_default(),
                        start_offset: instruction.offset,
                        end_offset: instruction.offset,
                    }));

                    // find SEND instruction
                    let mut send_to = 0;
                    while let Some(next_instruction) = opcode_instructions.get(offset + 1) {
                        if next_instruction.opcode() == Opcode::Send {
                            offset += 1;
                            send_to = next_instruction
                                .argval
                                .as_ref()
                                .ok_or(format!(
                                    "[GetAwaitable] No argval, deviation is {}",
                                    next_instruction.offset
                                ))?
                                .trim_start_matches("to ")
                                .parse::<usize>()?;
                            break;
                        }
                        offset += 1;
                    }

                    // skip the offset to the <send_to>
                    while let Some(next_instruction) = opcode_instructions.get(offset + 1) {
                        match next_instruction.offset.cmp(&send_to) {
                            Ordering::Equal => break,
                            Ordering::Greater => {
                                return Err(format!(
                                    "[GetAwaitable] Send target not found, deviation is {}",
                                    next_instruction.offset
                                )
                                .into())
                            }
                            _ => {}
                        }
                        offset += 1;
                    }
                }
                Opcode::UnpackSequence => {
                    let mut count = instruction.arg.ok_or(format!(
                        "[UnpackSequence] No arg, deviation is {}",
                        instruction.offset
                    ))?;
                    let mut sequence = Vec::with_capacity(count);
                    loop {
                        if count == 0 {
                            break;
                        }
                        let next_instruction =
                            opcode_instructions.get(offset + 1).ok_or(format!(
                                "[UnpackSequence] No next instruction, diviation is {}",
                                instruction.offset
                            ))?;
                        match next_instruction.opcode() {
                            Opcode::StoreName | Opcode::StoreFast => {
                                let name = next_instruction
                                    .argval
                                    .as_ref()
                                    .ok_or(format!(
                                        "[UnpackSequence] No argval, diviation is {}",
                                        next_instruction.offset
                                    ))?
                                    .clone();
                                if let Some(ExpressionEnum::Container(sub_seq)) = sequence.last() {
                                    if sub_seq.values.len() < sub_seq.values.capacity() {
                                        sub_seq.with_mut_unchecked().patch_by(|mut container| {
                                            container.values.push(ExpressionEnum::BaseValue(
                                                BaseValue {
                                                    value: name,
                                                    start_offset: next_instruction.offset,
                                                    end_offset: next_instruction.offset,
                                                    ..Default::default()
                                                },
                                            ))
                                        })?;
                                    }
                                    if sub_seq.values.len() == sub_seq.values.capacity() {
                                        count -= 1;
                                    }
                                } else {
                                    sequence.push(ExpressionEnum::BaseValue(BaseValue {
                                        value: name,
                                        start_offset: next_instruction.offset,
                                        end_offset: next_instruction.offset,
                                        ..Default::default()
                                    }));
                                    count -= 1;
                                }
                            }
                            Opcode::UnpackSequence => {
                                let sub_count = next_instruction.arg.ok_or(format!(
                                    "[UnpackSequence] No arg, diviation is {}",
                                    next_instruction.offset
                                ))?;
                                let sub_sequence = Vec::with_capacity(sub_count);
                                sequence.push(ExpressionEnum::Container(Container {
                                    values: sub_sequence,
                                    start_offset: next_instruction.offset,
                                    end_offset: next_instruction.offset,
                                    ..Default::default()
                                }))
                            }
                            _ => {
                                return Err(format!(
                                "[UnpackSequence] Expect StoreName, StoreFast or UnpackSequence, but got {:?}",
                                next_instruction.opcode()
                            )
                                .into())
                            }
                        }
                        offset += 1;
                    }
                    if let Some(ExpressionEnum::For(for_expr)) = exprs_stack.last() {
                        for_expr.with_mut_unchecked().patch_by(|mut f| {
                            f.items = Box::new(ExpressionEnum::Container(Container {
                                values: sequence,
                                start_offset: instruction.offset,
                                end_offset: instruction.offset,
                                ..Default::default()
                            }))
                        })?;
                    } else {
                        return Err(format!(
                            "[UnpackSequence] Expect <For> expr, but got {:?}",
                            exprs_stack.last()
                        )
                        .into());
                    }
                }
                Opcode::Copy => {
                    let count = instruction.arg.ok_or(format!(
                        "[Copy] No arg, deviation is {}",
                        instruction.offset
                    ))?;

                    if count <= exprs_stack.len() {
                        let copy_exprs = exprs_stack
                            .iter()
                            .skip(exprs_stack.len() - count)
                            .cloned()
                            .collect::<Vec<_>>();
                        exprs_stack.extend(copy_exprs);
                    } else {
                        return Err(format!(
                            "[Copy] Stack is empty, deviation is {}",
                            instruction.offset
                        )
                        .into());
                    }
                }
                Opcode::Swap => {
                    let count = instruction.arg.ok_or(format!(
                        "[Swap] No arg, deviation is {}",
                        instruction.offset
                    ))?;
                    if count <= exprs_stack.len() {
                        let mut swap_exprs = exprs_stack
                            .iter()
                            .skip(exprs_stack.len() - count)
                            .cloned()
                            .collect::<Vec<_>>();
                        swap_exprs.reverse();
                        exprs_stack.truncate(exprs_stack.len() - count);
                        exprs_stack.extend(swap_exprs);
                    } else {
                        return Err(format!(
                            "[Swap] Stack is empty, deviation is {}",
                            instruction.offset
                        )
                        .into());
                    }
                }
                _ => {}
            }

            offset += 1;
        }

        Ok(Box::new(Self::from(exprs_stack)))
    }
}

pub fn get_trace(opcode_instructions: &[OpcodeInstruction]) -> Result<TraceBack> {
    let mut traceback = TraceBack::default();

    for instruction in opcode_instructions {
        match instruction.opcode() {
            Opcode::StoreFast => {
                let arg = instruction.arg.as_ref().ok_or(format!(
                    "[Trace] No arg, deviation is {}",
                    instruction.offset
                ))?;
                let name = instruction.argval.as_ref().ok_or(format!(
                    "[Trace] No argval, deviation is {}",
                    instruction.offset
                ))?;

                if let Some(local) = traceback.get_mut_local(arg) {
                    if !local.is_store {
                        // do not store fast before, it's an arguement for function
                        local.is_arg = true;
                    } else {
                        // store fast after load fast, not arguement for function
                        local.is_arg = false;
                    }
                } else {
                    // store fast before load fast, not arguement for function
                    traceback.insert_local(
                        *arg,
                        Local {
                            name: name.clone(),
                            is_store: true,
                            is_arg: false,
                        },
                    );
                }
            }
            Opcode::LoadFast => {
                let arg = instruction.arg.as_ref().ok_or(format!(
                    "[Trace] No arg, deviation is {}",
                    instruction.offset
                ))?;
                let name = instruction.argval.as_ref().ok_or(format!(
                    "[Trace] No argval, deviation is {}",
                    instruction.offset
                ))?;
                if let Some(local) = traceback.get_mut_local(arg) {
                    // load fast after store fast, not arguement for function
                    if local.is_store {
                        local.is_arg = false;
                    } else {
                        // is arg because its not store before
                        local.is_arg = true;
                    }
                } else {
                    // if load fast before store fast, it's an arguement for function
                    traceback.insert_local(
                        *arg,
                        Local {
                            name: name.clone(),
                            is_store: false,
                            is_arg: true,
                        },
                    );
                }
            }
            Opcode::ReturnGenerator => {
                // mark this block is async
                traceback.mark_async();
            }
            Opcode::PopJumpIfFalse | Opcode::PopJumpIfTrue => {
                let jump_target = instruction
                    .argval
                    .as_ref()
                    .ok_or(format!(
                        "[Trace] No argval, deviation is {}",
                        instruction.offset
                    ))?
                    .trim_start_matches("to ")
                    .parse::<usize>()?;
                traceback.insert_jump(instruction.offset, jump_target);
            }
            _ => {}
        }
    }

    Ok(traceback)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expr() {
        let instructions = [
            OpcodeInstruction {
                opcode: Opcode::LoadConst.into(),
                opname: "LOAD_CONST".into(),
                arg: Some(0),
                argval: Some("'a'".into()),
                offset: 2,
                starts_line: Some(1),
                is_jump_target: false,
                positions: vec![],
            },
            OpcodeInstruction {
                opcode: Opcode::LoadName.into(),
                opname: "LOAD_NAME".into(),
                arg: Some(0),
                argval: Some("int".into()),
                offset: 4,
                starts_line: Some(1),
                is_jump_target: false,
                positions: vec![],
            },
            OpcodeInstruction {
                opcode: Opcode::LoadConst.into(),
                opname: "LOAD_CONST".into(),
                arg: Some(1),
                argval: Some("'return'".into()),
                offset: 6,
                starts_line: Some(1),
                is_jump_target: false,
                positions: vec![],
            },
            OpcodeInstruction {
                opcode: Opcode::LoadName.into(),
                opname: "LOAD_NAME".into(),
                arg: Some(0),
                argval: Some("int".into()),
                offset: 8,
                starts_line: Some(1),
                is_jump_target: false,
                positions: vec![],
            },
            OpcodeInstruction {
                opcode: Opcode::BuildTuple.into(),
                opname: "BUILD_TUPLE".into(),
                arg: Some(4),
                argval: None,
                offset: 10,
                starts_line: Some(1),
                is_jump_target: false,
                positions: vec![],
            },
            OpcodeInstruction {
                opcode: Opcode::LoadConst.into(),
                opname: "LOAD_CONST".into(),
                arg: Some(2),
                argval: Some(
                    "<code object test at 0x00000279922BDB80, file \"test/def.py\", line 1>".into(),
                ),
                offset: 12,
                starts_line: Some(1),
                is_jump_target: false,
                positions: vec![],
            },
            OpcodeInstruction {
                opcode: Opcode::MakeFunction.into(),
                opname: "MAKE_FUNCTION".into(),
                arg: Some(4),
                argval: Some("annotations".into()),
                offset: 14,
                starts_line: Some(1),
                is_jump_target: false,
                positions: vec![],
            },
            OpcodeInstruction {
                opcode: Opcode::StoreName.into(),
                opname: "STORE_NAME".into(),
                arg: Some(1),
                argval: Some("test".into()),
                offset: 16,
                starts_line: Some(1),
                is_jump_target: false,
                positions: vec![],
            },
        ];

        assert_eq!(
            Expr::parse(&instructions).unwrap(),
            Box::new(Expr {
                bodys: vec![ExpressionEnum::Function(Function {
                    mark: "<code object test at 0x00000279922BDB80, file \"test/def.py\", line 1>"
                        .into(),
                    name: "test".into(),
                    args: [
                        FastVariable {
                            index: 0,
                            name: "a".into(),
                            annotation: Some("int".into()),
                            ..Default::default()
                        },
                        FastVariable {
                            index: 1,
                            name: "return".into(),
                            annotation: Some("int".into()),
                            ..Default::default()
                        },
                    ]
                    .into(),
                    defaults: vec![],
                    start_line: 1,
                    end_line: 1,
                    bodys: vec![],
                    start_offset: 14,
                    end_offset: 14,
                    ..Default::default()
                },),]
                .into()
            })
        )
    }

    #[test]
    fn test_query() {
        let expr = Box::new(Expr {
            bodys: vec![ExpressionEnum::Assign(Assign {
                target: Box::new(ExpressionEnum::BaseValue(BaseValue {
                    value: "test".into(),
                    ..Default::default()
                })),
                values: Box::new(ExpressionEnum::Function(Function {
                    mark: "<code object test at 0x00000279922BDB80, file \"test/def.py\", line 1>"
                        .into(),
                    name: "test".into(),
                    args: vec![],
                    defaults: vec![],
                    start_line: 1,
                    end_line: 1,
                    bodys: vec![],
                    ..Default::default()
                })),
                operator: "=".into(),
                ..Default::default()
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
                    value: "test".into(),
                    ..Default::default()
                })),
                values: Box::new(ExpressionEnum::Function(Function {
                    mark: "<code object test at 0x00000279922BDB80, file \"test/def.py\", line 1>"
                        .into(),
                    name: "test".into(),
                    args: vec![],
                    defaults: vec![],
                    start_line: 1,
                    end_line: 1,
                    bodys: vec![],
                    ..Default::default()
                },)),
                operator: "=".into(),
                ..Default::default()
            }]
        );
        assert_eq!(
            function_query,
            vec![&Function {
                mark: "<code object test at 0x00000279922BDB80, file \"test/def.py\", line 1>"
                    .into(),
                name: "test".into(),
                args: vec![],
                defaults: vec![],
                start_line: 1,
                end_line: 1,
                bodys: vec![],
                ..Default::default()
            }]
        );
    }

    #[test]
    fn test_any() {
        let expr = Assign {
            target: Box::new(ExpressionEnum::BaseValue(BaseValue {
                value: "a".to_string(),
                ..Default::default()
            })),
            values: Box::new(ExpressionEnum::BaseValue(BaseValue {
                value: "1".to_string(),
                ..Default::default()
            })),
            operator: "=".to_string(),
            ..Default::default()
        };
        let any = expr.try_query::<Assign>();

        //dbg!(any);
        //assert!(false);
        assert_eq!(
            any,
            Some(&Assign {
                target: Box::new(ExpressionEnum::BaseValue(BaseValue {
                    value: "a".to_string(),
                    ..Default::default()
                })),
                values: Box::new(ExpressionEnum::BaseValue(BaseValue {
                    value: "1".to_string(),
                    ..Default::default()
                })),
                operator: "=".to_string(),
                ..Default::default()
            })
        )
    }
}
