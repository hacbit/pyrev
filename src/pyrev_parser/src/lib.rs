//! Parser the opcode to AST

#![feature(let_chains)]

use pyrev_ast::*;
use pyrev_core::opcode::{Opcode, OpcodeInstruction};
use pyrev_query::Map;
use regex::Regex;

type Result<T> = std::result::Result<T, ParseError>;

/// Parse error
///
/// It contains some common errors that may occur during parsing
pub enum ParseError {
    OffsetOutOfRange,
    NoInstruction,
    /// The argument value is None
    ///
    /// The offset is the index of the instruction
    ArgvalIsNone {
        offset: usize,
    },
    IsNone(String),
    IsNot(String),
    IdsStackEmpty,
    InvalidArg { target: String },
    QueryFailed,
    QueryMutFailed,
    AddMapFailed,
    Other(String),
}

/// A helper macro to add a new expression to the map
/// and return the [`Option<QueryId>`]
///
/// You don't need to input [`..Default::default()`] in the block
/// The macro will add it automatically
///
/// # Example
/// ```ignore
/// let query_id = add_helper! {
///     map, BaseValue {
///         value: "hello".to_string(),
///     }
/// };
/// ```
#[macro_export]
macro_rules! add_helper {
    (
        $map:expr, $ty:ident {
            $($key:ident: $value:expr,)*$(,)?
        }
    ) => {
        {
            let expr = $ty {
                $($key: $value,) *
                ..Default::default()
            };
            $map.add::<$ty>(expr.into())
        }
    }
}

/// Parse the opcode instructions to AST
///
/// Each expression will be added to the map
pub fn parse(map: &mut Map, opcode_instructions: &[OpcodeInstruction]) -> Result<Vec<QueryId>> {
    let mut expr_ids = Vec::<QueryId>::new();
    let mut index = 0;
    loop {
        if index == opcode_instructions.len() {
            break;
        } else if index > opcode_instructions.len() {
            return Err(ParseError::OffsetOutOfRange);
        }

        let instruction = opcode_instructions
            .get(index)
            .ok_or(ParseError::NoInstruction)?;

        let opcode = instruction.opcode();

        match opcode {
            Opcode::LoadConst | Opcode::LoadName | Opcode::LoadGlobal => {
                let query_id = add_helper! {
                    map, BaseValue {
                        value: instruction
                            .argval
                            .as_ref()
                            .ok_or(ParseError::ArgvalIsNone { offset: instruction.offset })?
                            .trim_start_matches("NULL + ")
                            .to_string(),
                        start_offset: instruction.offset,
                        end_offset: instruction.offset + 1,
                    }
                }
                .ok_or(ParseError::AddMapFailed)?;

                expr_ids.push(query_id);
            } // end LoadConst | LoadName | LoadGlobal
            Opcode::LoadFast => {
                let name = instruction
                    .argval
                    .as_ref()
                    .ok_or(ParseError::ArgvalIsNone {
                        offset: instruction.offset,
                    })?
                    .clone();

                let query_id = add_helper! {
                    map, BaseValue {
                        value: name,
                        start_offset: instruction.offset,
                        end_offset: instruction.offset + 1,
                    }
                }
                .ok_or(ParseError::AddMapFailed)?;

                expr_ids.push(query_id);
            } // end LoadFast
            Opcode::LoadAttr => {
                let parent_id = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;
                let attr = instruction
                    .argval
                    .as_ref()
                    .ok_or(ParseError::ArgvalIsNone {
                        offset: instruction.offset,
                    })?;

                let query_id = add_helper! {
                    map, Attribute {
                        parent: Some(parent_id),
                        attr: add_helper! {
                            map, BaseValue {
                                value: attr.to_string(),
                                start_offset: instruction.offset,
                                end_offset: instruction.offset + 1,
                            }
                        },
                        start_offset: instruction.offset,
                        end_offset: instruction.offset + 1,
                    }
                }
                .ok_or(ParseError::AddMapFailed)?;

                expr_ids.push(query_id);
            } // end LoadAttr
            Opcode::LoadMethod => {
                let parent_id = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;
                let method = instruction
                    .argval
                    .as_ref()
                    .ok_or(ParseError::ArgvalIsNone {
                        offset: instruction.offset,
                    })?;
                let query_id = add_helper! {
                    map, Attribute {
                        parent: Some(parent_id),
                        attr: add_helper! {
                            map, BaseValue {
                                value: method.to_owned(),
                                start_offset: instruction.offset,
                                end_offset: instruction.offset + 1,
                            }
                        },
                        start_offset: instruction.offset,
                        end_offset: instruction.offset + 1,
                    }
                }
                .ok_or(ParseError::AddMapFailed)?;

                expr_ids.push(query_id);
            } // end LoadMethod
            Opcode::StoreName | Opcode::StoreGlobal => {
                let name = instruction
                    .argval
                    .as_ref()
                    .ok_or(ParseError::ArgvalIsNone {
                        offset: instruction.offset,
                    })?
                    .clone();

                let value_id = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;
                let value = map.get_single(value_id).ok_or(ParseError::QueryFailed)?;

                match value {
                    ExpressionEnum::Function(function) => {
                        let n = function.name.as_str();
                        if ["<lambda>", "<genexpr>", "<listcomp>"].contains(&n) {
                            let query_id = add_helper! {
                                map, Assign {
                                    target: add_helper! {
                                        map, BaseValue {
                                            value: name,
                                            start_offset: instruction.offset,
                                            end_offset: instruction.offset + 1,
                                        }
                                    },
                                    value: Some(value_id),
                                    operator: "=".to_string(),
                                }
                            }
                            .ok_or(ParseError::AddMapFailed)?;

                            expr_ids.push(query_id);
                        } else {
                            expr_ids.push(value_id);
                        }
                    } // end Function
                    ExpressionEnum::Import(import) => {
                        if import.bk_module.is_none() {
                            // hasn't `from`
                            if import.module == name {
                                // hasn't `as`
                                expr_ids.push(value_id);
                            } else {
                                // has `as`
                                // set the alias to the import
                                if let ExpressionEnum::Import(imp) = map
                                    .get_single_mut(value_id)
                                    .ok_or(ParseError::QueryMutFailed)?
                                {
                                    imp.alias = Some(name);
                                }
                            }
                        } else {
                            // has `from`
                            if import
                                .fragment
                                .as_ref()
                                .ok_or(ParseError::IsNone("Import fragment".into()))?
                                == &name
                            {
                                // hasn't `as`
                                if let ExpressionEnum::Import(imp) = map
                                    .get_single_mut(value_id)
                                    .ok_or(ParseError::QueryMutFailed)?
                                {
                                    imp.bk_module = Some(
                                        imp.bk_module.clone().unwrap_or_default() + &name + ", ",
                                    );
                                }
                            } else {
                                // has `as`
                                if let ExpressionEnum::Import(imp) = map
                                    .get_single_mut(value_id)
                                    .ok_or(ParseError::QueryMutFailed)?
                                {
                                    imp.bk_module = Some(
                                        imp.bk_module.clone().unwrap_or_default()
                                            + imp.fragment.as_ref().ok_or(ParseError::IsNone(
                                                "Import fragment".into(),
                                            ))?
                                            + " as "
                                            + &name
                                            + ", ",
                                    );
                                }
                            }
                        }

                        // didn't add a new expression
                        // so push the original id back
                        expr_ids.push(value_id);
                    } // end Import
                    _ => {
                        let query_id = add_helper! {
                            map, Assign {
                                target: add_helper! {
                                    map, BaseValue {
                                        value: name,
                                        start_offset: instruction.offset,
                                        end_offset: instruction.offset + 1,
                                    }
                                },
                                value: Some(value_id),
                                operator: "=".to_string(),
                                start_offset: instruction.offset,
                                end_offset: instruction.offset + 1,
                            }
                        }
                        .ok_or(ParseError::AddMapFailed)?;

                        expr_ids.push(query_id);
                    }
                }
            } // end StoreName | StoreGlobal
            Opcode::StoreFast => {
                let name = instruction
                    .argval
                    .as_ref()
                    .ok_or(ParseError::ArgvalIsNone {
                        offset: instruction.offset,
                    })?
                    .clone();
                let value_id = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;
                let value = map.get_single(value_id).ok_or(ParseError::QueryFailed)?;

                match value {
                    ExpressionEnum::Function(_) => {
                        expr_ids.push(value_id);
                    } // end Function
                    _ => {
                        let query_id = add_helper! {
                            map, Assign {
                                target: add_helper! {
                                    map, BaseValue {
                                        value: name,
                                        start_offset: instruction.offset,
                                        end_offset: instruction.offset + 1,
                                    }
                                },
                                value: Some(value_id),
                                operator: "=".to_string(),
                                start_line: instruction.starts_line.unwrap_or_default(),
                                start_offset: instruction.offset,
                                end_offset: instruction.offset + 1,
                            }
                        }
                        .ok_or(ParseError::AddMapFailed)?;

                        expr_ids.push(query_id);
                    }
                }
            } // end StoreFast
            Opcode::StoreAttr => {
                let parent_id = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;

                let attr = instruction
                    .argval
                    .as_ref()
                    .ok_or(ParseError::ArgvalIsNone {
                        offset: instruction.offset,
                    })?
                    .clone();
                let attr_id = add_helper! {
                    map, BaseValue {
                        value: attr,
                        start_offset: instruction.offset,
                        end_offset: instruction.offset + 1,
                    }
                }
                .ok_or(ParseError::AddMapFailed)?;

                let value_id = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;

                let target_id = add_helper! {
                    map, Attribute {
                        parent: Some(parent_id),
                        attr: Some(attr_id),
                        start_offset: instruction.offset,
                        end_offset: instruction.offset + 1,
                    }
                }
                .ok_or(ParseError::AddMapFailed)?;

                // if the value is [`BinaryOperation`] and it's a self-assign operation like `+=`, `-=`, etc.
                // and the target is the same as the left of the [`BinaryOperation`]
                // then we can ignore the target and just return the [`BinaryOperation`]
                if let ExpressionEnum::BinaryOperation(bin_op) =
                    map.get_single(value_id).ok_or(ParseError::QueryFailed)?
                {
                    if let ExpressionEnum::Attribute(attr) = map
                        .get_single(
                            bin_op
                                .left
                                .ok_or(ParseError::IsNone("BinaryOperation left".into()))?,
                        )
                        .ok_or(ParseError::QueryFailed)?
                    {
                        if attr.parent == Some(parent_id) // Some(target.parent)
                            && attr.attr == Some(attr_id)
                            && bin_op.operator.ends_with('=')
                        {
                            expr_ids.push(value_id);
                            index += 1;
                            continue;
                        }
                    }
                }

                let query_id = add_helper! {
                    map, Assign {
                        target: Some(target_id),
                        value: Some(value_id),
                        operator: '='.to_string(),
                        start_line: instruction.starts_line.unwrap_or_default(),
                        start_offset: instruction.offset,
                        end_offset: instruction.offset + 1,
                    }
                }
                .ok_or(ParseError::AddMapFailed)?;

                expr_ids.push(query_id);
            } // end StoreAttr
            Opcode::LoadBuildClass => {
                let mark = opcode_instructions
                    .get(index + 1)
                    .ok_or(ParseError::OffsetOutOfRange)?
                    .argval
                    .as_ref()
                    .ok_or(ParseError::ArgvalIsNone {
                        offset: instruction.offset,
                    })?
                    .clone();

                let class = Class::new(mark).map_err(|e| ParseError::Other(e.to_string()))?;
                let class_id = map
                    .add::<Class>(class.into())
                    .ok_or(ParseError::AddMapFailed)?;
                expr_ids.push(class_id);

                // skip build class
                loop {
                    index += 1;
                    if let Some(next_instruction) = opcode_instructions.get(index) {
                        if next_instruction.starts_line.unwrap() != instruction.starts_line.unwrap()
                        {
                            break;
                        }
                    }
                }
                index -= 1;
            } // end LoadBuildClass
            Opcode::FormatValue => {
                let format_value = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;
                let query_id = add_helper! {
                    map, FormatValue {
                        value: Some(format_value),
                        start_offset: instruction.offset,
                        end_offset: instruction.offset + 1,
                    }
                }
                .ok_or(ParseError::AddMapFailed)?;

                expr_ids.push(query_id);
            } // end FormatValue
            Opcode::BuildString => {
                let size = instruction
                    .arg
                    .ok_or(ParseError::IsNone("BuildString size".into()))?;
                let mut format_string = Vec::with_capacity(size);
                for _ in 0..size {
                    format_string.push(expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?);
                }
                format_string.reverse();

                let query_id = add_helper! {
                    map, Format {
                        format_values: format_string,
                        start_offset: instruction.offset,
                        end_offset: instruction.offset + 1,
                    }
                }
                .ok_or(ParseError::AddMapFailed)?;

                expr_ids.push(query_id);
            } // end BuildString
            Opcode::BuildTuple => {
                let size = instruction
                    .arg
                    .ok_or(ParseError::IsNone("BuildTuple size".into()))?;
                let mut tuple = Vec::with_capacity(size);
                for _ in 0..size {
                    tuple.push(expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?);
                }
                tuple.reverse();

                let query_id = add_helper! {
                    map, Container {
                        values: tuple,
                        container_type: ContainerType::Tuple,
                        start_offset: instruction.offset,
                        end_offset: instruction.offset + 1,
                    }
                }
                .ok_or(ParseError::AddMapFailed)?;

                expr_ids.push(query_id);
            } // end BuildTuple
            Opcode::BuildList => {
                let size = instruction
                    .arg
                    .ok_or(ParseError::IsNone("BuildList size".into()))?;

                if size == 0 {
                    let query_id = add_helper! {
                        map, Container {
                            values: vec![],
                            container_type: ContainerType::List,
                            start_offset: instruction.offset,
                            end_offset: instruction.offset + 1,
                        }
                    }
                    .ok_or(ParseError::AddMapFailed)?;

                    expr_ids.push(query_id);
                } else {
                    let mut list = Vec::with_capacity(size);
                    for _ in 0..size {
                        list.push(expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?);
                    }
                    list.reverse();

                    let query_id = add_helper! {
                        map, Container {
                            values: list,
                            container_type: ContainerType::List,
                            start_offset: instruction.offset,
                            end_offset: instruction.offset + 1,
                        }
                    }
                    .ok_or(ParseError::AddMapFailed)?;

                    expr_ids.push(query_id);
                }
            } // end BuildList
            Opcode::ListExtend => {
                let size = instruction
                    .arg
                    .ok_or(ParseError::IsNone("ListExtend size".into()))?;
                let mut extend = Vec::with_capacity(size);
                for _ in 0..size {
                    extend.push(expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?);
                }
                extend.reverse();

                let list_id = expr_ids.last().ok_or(ParseError::IdsStackEmpty)?;

                if let Some(container) = map
                    .get_single(*list_id)
                    .ok_or(ParseError::QueryFailed)?
                    .as_ref_container()
                    && container.container_type == ContainerType::List
                {
                    let re = Regex::new(r"\((.*)\)").unwrap();
                    for e in extend.iter() {
                        if let ExpressionEnum::BaseValue(base) =
                            map.get_single_mut(*e).ok_or(ParseError::QueryMutFailed)?
                        {
                            base.value = re
                                .captures(&base.value)
                                .ok_or(ParseError::Other("ListExtend value is not a tuple".into()))?
                                .get(1)
                                .ok_or(ParseError::OffsetOutOfRange)?
                                .as_str()
                                .to_string();
                        }
                    }

                    if let ExpressionEnum::Container(container) = map
                        .get_single_mut(*list_id)
                        .ok_or(ParseError::QueryMutFailed)?
                    {
                        container.values.extend(extend);
                    }
                } else {
                    return Err(ParseError::Other("ListExtend target is not a list".into()));
                }
            } // end ListExtend
            Opcode::BuildSet => {
                let size = instruction
                    .arg
                    .ok_or(ParseError::IsNone("BuildSet size".into()))?;
                let mut set = Vec::with_capacity(size);
                for _ in 0..size {
                    set.push(expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?);
                }
                set.reverse();

                let query_id = add_helper! {
                    map, Container {
                        values: set,
                        container_type: ContainerType::Set,
                        start_offset: instruction.offset,
                        end_offset: instruction.offset + 1,
                    }
                }
                .ok_or(ParseError::AddMapFailed)?;

                expr_ids.push(query_id);
            } // end BuildSet
            Opcode::BuildMap => {
                let size = instruction
                    .arg
                    .ok_or(ParseError::IsNone("BuildMap size".into()))?;
                if size == 0 {
                    let query_id = add_helper! {
                        map, Container {
                            values: vec![],
                            container_type: ContainerType::Dict,
                            start_offset: instruction.offset,
                            end_offset: instruction.offset + 1,
                        }
                    }
                    .ok_or(ParseError::AddMapFailed)?;

                    expr_ids.push(query_id);
                } else {
                    let mut map_values = Vec::with_capacity(size * 2);
                    for _ in 0..size {
                        let value = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;
                        let key = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;
                        map_values.push(value);
                        map_values.push(key);
                    }
                    map_values.reverse();

                    let query_id = add_helper! {
                        map, Container {
                            values: map_values,
                            container_type: ContainerType::Dict,
                            start_offset: instruction.offset,
                            end_offset: instruction.offset + 1,
                        }
                    }
                    .ok_or(ParseError::AddMapFailed)?;

                    expr_ids.push(query_id);
                }
            } // end BuildMap
            Opcode::BuildConstKeyMap => {
                let size = instruction
                    .arg
                    .ok_or(ParseError::IsNone("BuildConstKeyMap size".into()))?;
                let keys_id = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;
                let mut value_ids = Vec::with_capacity(size);
                for _ in 0..size {
                    value_ids.push(expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?);
                }
                value_ids.reverse();

                let mut map_values = Vec::with_capacity(size * 2);
                if let ExpressionEnum::BaseValue(BaseValue { value: key, .. }) =
                    map.get_single(keys_id).ok_or(ParseError::QueryFailed)?
                {
                    for (k, v) in key
                        .clone()
                        .trim_start_matches('(')
                        .trim_end_matches(')')
                        .split(", ")
                        .zip(value_ids.iter())
                    {
                        let query_id = add_helper! {
                            map, BaseValue {
                                value: k.to_string(),
                                start_offset: instruction.offset,
                                end_offset: instruction.offset + 1,
                            }
                        }
                        .ok_or(ParseError::AddMapFailed)?;
                        map_values.push(query_id);
                        map_values.push(*v);
                    }
                }

                let query_id = add_helper! {
                    map, Container {
                        values: map_values,
                        container_type: ContainerType::Dict,
                        start_offset: instruction.offset,
                        end_offset: instruction.offset + 1,
                    }
                }
                .ok_or(ParseError::AddMapFailed)?;

                expr_ids.push(query_id);
            } // end BuildConstKeyMap
            Opcode::BuildSlice => {
                let size = instruction
                    .arg
                    .ok_or(ParseError::IsNone("BuildSlice size".into()))?;
                let mut slice = Vec::with_capacity(size);
                for _ in 0..size {
                    slice.push(expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?);
                }
                slice.reverse();

                let origin_id = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;

                let query_id = add_helper! {
                    map, Slice {
                        origin: Some(origin_id),
                        slice: slice,
                        start_offset: instruction.offset,
                        end_offset: instruction.offset + 1,
                    }
                }
                .ok_or(ParseError::AddMapFailed)?;

                expr_ids.push(query_id);
            } // end BuildSlice
            Opcode::MakeFunction => {
                let mark_id = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;
                let mut function = Function::new(
                    &map.get_single(mark_id)
                        .ok_or(ParseError::QueryFailed)?
                        .as_ref_base_value()
                        .ok_or(ParseError::IsNot("BaseValue".into()))?
                        .value,
                )
                .map_err(|e| ParseError::Other(e.to_string()))?;
                if let Some(argval) = instruction.argval.as_ref() {
                    if argval.contains("annotations") {
                        let values_id = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;
                        if let ExpressionEnum::Container(container) =
                            map.get_single(values_id).ok_or(ParseError::QueryFailed)?
                        {
                            debug_assert_eq!(container.container_type, ContainerType::Tuple);

                            for (idx, ids) in container.values.clone().chunks(2).enumerate() {
                                if ids.iter().all(|e| e.is::<BaseValue>()) {
                                    let arg_id = add_helper! {
                                        map, FastVariable {
                                            index: idx,
                                            name: map.get_single(ids[0]).ok_or(ParseError::QueryFailed)?.as_ref_base_value().unwrap().value.trim_start_matches('\'').trim_end_matches('\'').to_string(),
                                            annotation: Some(map.get_single(ids[1]).ok_or(ParseError::QueryFailed)?.as_ref_base_value().unwrap().value.clone()),
                                        }
                                    }.ok_or(ParseError::AddMapFailed)?;

                                    function.args.push(arg_id);
                                }
                            } // end for
                        }
                    } // end if argval contains annotations

                    if argval.contains("defaults") {
                        let defaults_id = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;
                        if let ExpressionEnum::BaseValue(base) =
                            map.get_single(defaults_id).ok_or(ParseError::QueryFailed)?
                        {
                            let defaults = base
                                .value
                                .trim_start_matches('(')
                                .trim_end_matches(')')
                                .trim_end_matches(',')
                                .split(", ")
                                .map(|x| x.to_string())
                                .collect::<Vec<_>>();

                            function.defaults = defaults;
                        }
                    } // end if argval contains defaults
                }

                function.start_offset = instruction.offset;
                function.end_offset = instruction.offset + 1;

                let query_id = map
                    .add::<Function>(function.into())
                    .ok_or(ParseError::AddMapFailed)?;
                expr_ids.push(query_id);
            } // end MakeFunction
            Opcode::BinaryOp | Opcode::CompareOp => {
                let right = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;
                let left = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;

                let query_id = add_helper! {
                    map, BinaryOperation {
                        left: Some(left),
                        right: Some(right),
                        operator: instruction
                            .argval
                            .as_ref()
                            .ok_or(ParseError::ArgvalIsNone { offset: instruction.offset })?
                            .to_string(),
                        start_offset: instruction.offset,
                        end_offset: instruction.offset + 1,
                    }
                }.ok_or(ParseError::AddMapFailed)?;

                expr_ids.push(query_id);
            } // end BinaryOp | CompareOp
            Opcode::IsOp => {
                let right = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;
                let left = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;
                let operator = match instruction.arg.as_ref() {
                    Some(0) => "is",
                    Some(1) => "is not",
                    _ => return Err(ParseError::InvalidArg { target: "IsOp operator".into() }),
                };
                
                let query_id = add_helper! {
                    map, BinaryOperation {
                        left: Some(left),
                        right: Some(right),
                        operator: operator.to_string(),
                        start_offset: instruction.offset,
                        end_offset: instruction.offset + 1,
                    }
                }.ok_or(ParseError::AddMapFailed)?;

                expr_ids.push(query_id);
            } // end IsOp
            Opcode::ContainsOp => {
                let right = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;
                let left = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;
                let operator = match instruction.arg.as_ref() {
                    Some(0) => "in",
                    Some(1) => "not in",
                    _ => return Err(ParseError::InvalidArg { target: "ContainsOp operator".into() }),
                };
                
                let query_id = add_helper! {
                    map, BinaryOperation {
                        left: Some(left),
                        right: Some(right),
                        operator: operator.to_string(),
                        start_offset: instruction.offset,
                        end_offset: instruction.offset + 1,
                    }
                }.ok_or(ParseError::AddMapFailed)?;

                expr_ids.push(query_id);
            } // end ContainsOp
            Opcode::BinarySubscr => {
                let scr_id = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;
                let target_id = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;

                let query_id = add_helper! {
                    map, Subscr {
                        target: Some(target_id),
                        index: Some(scr_id),
                        start_offset: instruction.offset,
                        end_offset: instruction.offset + 1,
                    }
                }.ok_or(ParseError::AddMapFailed)?;

                expr_ids.push(query_id);
            } // end BinarySubscr
            Opcode::UnaryInvert => {
                let target_id = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;

                let query_id = add_helper! {
                    map, UnaryOperation {
                        target: Some(target_id),
                        unary_type: UnaryType::Invert,
                        start_offset: instruction.offset,
                        end_offset: instruction.offset + 1,
                    }
                }.ok_or(ParseError::AddMapFailed)?;

                expr_ids.push(query_id);
            } // end UnaryInvert
            Opcode::UnaryNegative => {
                let target_id = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;

                let query_id = add_helper! {
                    map, UnaryOperation {
                        target: Some(target_id),
                        unary_type: UnaryType::Negative,
                        start_offset: instruction.offset,
                        end_offset: instruction.offset + 1,
                    }
                }.ok_or(ParseError::AddMapFailed)?;

                expr_ids.push(query_id);
            } // end UnaryNegative
            Opcode::UnaryNot => {
                let target_id = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;

                let query_id = add_helper! {
                    map, UnaryOperation {
                        target: Some(target_id),
                        unary_type: UnaryType::Not,
                        start_offset: instruction.offset,
                        end_offset: instruction.offset + 1,
                    }
                }.ok_or(ParseError::AddMapFailed)?;

                expr_ids.push(query_id);
            } // end UnaryNot
            Opcode::Call => {
                let count = instruction.arg.ok_or(ParseError::IsNone("Call count".into()))?;

                if count == 0 {
                    let last_id = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;
                    if let ExpressionEnum::BaseValue(base) = map.get_single(last_id).ok_or(ParseError::QueryFailed)? {
                        if base.value.contains(' ') {
                            // not a function call
                            expr_ids.push(last_id);
                        } else {
                            let query_id = add_helper! {
                                map, Call {
                                    func: Some(last_id),
                                    args: vec![],
                                    start_offset: instruction.offset,
                                    end_offset: instruction.offset + 1,
                                }
                            }.ok_or(ParseError::AddMapFailed)?;

                            expr_ids.push(query_id);
                        }
                    } else {
                        let query_id = add_helper! {
                            map, Call {
                                func: Some(last_id),
                                args: vec![],
                                start_offset: instruction.offset,
                                end_offset: instruction.offset + 1,
                            }
                        }.ok_or(ParseError::AddMapFailed)?;

                        expr_ids.push(query_id);
                    }

                    index += 1;
                    continue;
                }

                let mut args = Vec::with_capacity(count);
                for _ in 0..count {
                    args.push(expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?);
                }
                args.reverse();

                let last_id = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;
                match map.get_single(last_id).ok_or(ParseError::QueryFailed)? {
                    ExpressionEnum::BaseValue(function_name) => {
                        let function_name = function_name.value.trim_start_matches("NULL + ");

                        let query_id = add_helper! {
                            map, Call {
                                func: add_helper! {
                                    map, BaseValue {
                                        value: function_name.to_string(),
                                        start_offset: instruction.offset,
                                        end_offset: instruction.offset + 1,
                                    }
                                },
                                args: args,
                                start_line: instruction.starts_line.unwrap_or_default(),
                                start_offset: instruction.offset,
                                end_offset: instruction.offset + 1,
                            }
                        }.ok_or(ParseError::AddMapFailed)?;

                        expr_ids.push(query_id);
                    } // end BaseValue
                    _ => {
                        let query_id = add_helper! {
                            map, Call {
                                func: Some(last_id),
                                args: args,
                                start_offset: instruction.offset,
                                end_offset: instruction.offset + 1,
                            }
                        }.ok_or(ParseError::AddMapFailed)?;

                        expr_ids.push(query_id);
                    }
                }
            } // end Call
            Opcode::ReturnValue => {
                let value_id = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;

                let query_id = add_helper! {
                    map, Return {
                        value: Some(value_id),
                        start_line: instruction.starts_line.unwrap_or_default(),
                        start_offset: instruction.offset,
                        end_offset: instruction.offset + 1,
                    }
                }.ok_or(ParseError::AddMapFailed)?;

                expr_ids.push(query_id);
            } // end ReturnValue
            Opcode::YieldValue => {
                let value = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;

                let query_id = add_helper! {
                    map, Yield {
                        value: Some(value),
                        start_line: instruction.starts_line.unwrap_or_default(),
                        start_offset: instruction.offset,
                        end_offset: instruction.offset + 1,
                    }
                }.ok_or(ParseError::AddMapFailed)?;

                expr_ids.push(query_id);
            } // end YieldValue
            Opcode::ImportFrom => {
                let value_id = expr_ids.last().ok_or(ParseError::IdsStackEmpty)?;

                if let ExpressionEnum::Import(import) = map.get_single_mut(*value_id).ok_or(ParseError::QueryFailed)? {
                    if import.bk_module.is_none() {
                        // hasn't `from`
                        import.fragment = None;
                    } else {
                        // has `from`
                        let new_fragment = instruction
                            .argval
                            .as_ref()
                            .ok_or(ParseError::ArgvalIsNone {
                                offset: instruction.offset,
                            })?
                            .clone();
                        import.fragment = Some(new_fragment);
                    }
                }
            } // end ImportFrom
            Opcode::ImportName => {
                let module_id = expr_ids.pop().ok_or(ParseError::IdsStackEmpty)?;
                let last_id = expr_ids.last().ok_or(ParseError::IdsStackEmpty)?;

                // remove the '0'
                if let Some(ExpressionEnum::BaseValue(base)) = map.get_single(*last_id) {
                    if base.value == "0" {
                        expr_ids.pop();
                    }
                }

                todo!()
            } // end ImportName
            Opcode::PopJumpIfTrue => {

            } // end PopJumpIfTrue
            Opcode::PopJumpIfFalse => {

            } // end PopJumpIfFalse
            Opcode::JumpForward => {

            } // end JumpForward
            Opcode::JumpBackward => {

            } // end JumpBackward
            Opcode::LoadAssertionError => {

            } // end LoadAssertionError
            Opcode::RaiseVarargs => {

            } // end RaiseVarargs
            Opcode::CheckExcMatch => {

            } // end CheckExcMatch
            Opcode::BeforeWith => {

            } // end BeforeWith
            Opcode::BeforeAsyncWith => {

            } // end BeforeAsyncWith
            Opcode::ForIter => {

            } // end ForIter
            Opcode::GetAiter => {

            } // end GetAiter
            Opcode::EndAsyncFor => {

            } // end EndAsyncFor
            Opcode::GetAwaitable => {

            } // end GetAwaitable
            Opcode::UnpackSequence => {

            } // end UnpackSequence
            Opcode::Copy => {

            } // end Copy
            Opcode::Swap => {

            } // end Swap
            _ => {}
        } // end match

        // move to the next instruction
        index += 1;
    } // end loop

    Ok(expr_ids)
}
