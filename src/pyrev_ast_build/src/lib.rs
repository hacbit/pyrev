//! This crate provides some utilities for building the AST to a Python code.

use std::any::TypeId;

use pyrev_ast::*;
use pyrev_query::*;
use regex::Regex;

/// A helper function to get the expression from the map by query id.
#[inline]
pub fn get_helper<'a>(
    map: &'a Map,
    expr_id: Option<&QueryId>,
) -> Option<&'a ExpressionEnum> {
    expr_id.and_then(|id| map.get_single(*id))
}

/// A helper function to get the mutable expression from the map by optional query id.
#[inline]
pub fn get_mut_helper<'a>(
    map: &'a mut Map,
    expr_id: Option<&QueryId>,
) -> Option<&'a mut ExpressionEnum> {
    expr_id.and_then(move |id| map.get_single_mut(*id))
}

/// A packer for the `get_helper` function.
/// 
/// # Example
/// ```ignore
/// if let Some(assign) = helper!(map, id, as_ref_assign) {
///     // do something
/// }
/// if let Some(value) = helper!(map, id2, as_ref_base_value) {
///    // do something
/// }
/// ```
#[macro_export]
macro_rules! helper {
    ($map:ident, $expr_id:expr, $method:ident) => {
        get_helper($map, $expr_id).and_then(|expr| expr.$method())
    };
}

/// A packer for the `get_mut_helper` function.
/// 
/// # Example
/// ```ignore
/// if let Some(assign) = helper_mut!(map, id, as_ref_assign) {
///    // do something
/// }
/// if let Some(mut func) = helper_mut!(map, id2, as_ref_function) {
///     // do something
/// }
/// ```
#[macro_export]
macro_rules! helper_mut {
    ($map:ident, $expr_id:expr, $method:ident) => {
        get_mut_helper($map, $expr_id).and_then(|expr| expr.$method())
    };
}

/// A helper function to build the Python code from the expression by optional query id.
#[inline]
pub fn build_helper_some(map: &Map, id: Option<QueryId>) -> Option<Vec<String>> {
    let expr = map.get_single(id?)?;
    build(map, expr)
}

/// A helper function to build the Python code from the expression by query id.
#[inline]
pub fn build_helper(map: &Map, id: &QueryId) -> Option<Vec<String>> {
    let expr = map.get_single(*id)?;
    build(map, expr)
}

/// build ast to python code
pub fn build(map: &Map, expression: &ExpressionEnum) -> Option<Vec<String>> {
    let mut codes = vec![];
    match expression {
        ExpressionEnum::Class(class) => {
            let class_name = &class.name;
            codes.push(format!("class {}:", class_name));

            let mut class_members = class.members.iter();
            let filter_members = ["__module__", "__qualname__"];

            let mut next_expr = class_members.next();
            // expect to skip the __module__ and __qualname__ members
            loop {
                if let Some(assign) =
                    // get_helper(map, next_expr).and_then(|expr| expr.as_ref_assign())
                    helper!(map, next_expr, as_ref_assign)
                {
                    if let Some(name) = helper!(map, assign.target.as_ref(), as_ref_base_value)
                    {
                        if filter_members.contains(&name.value.as_str()) {
                            next_expr = class_members.next();
                            continue;
                        } else {
                            break;
                        }
                    }
                } else {
                    // unexpected member
                    // but no warning
                    break;
                }
            }

            let mut has_doc = false;

            if let Some(assign) = helper!(map, next_expr, as_ref_assign) {
                if let Some(name) = helper!(map, assign.target.as_ref(), as_ref_base_value) {
                    if name.value == "__doc__" {
                        has_doc = true;
                        let docstring = build_helper_some(map, assign.value)?.join("").replace("\\n", "\n");
                        let docstring = docstring.trim_matches('\'');
                        codes.push("    \"\"\"".to_string());
                        for line in docstring.lines().filter(|l| !l.trim().is_empty()) {
                            codes.push(line.to_string());
                        }
                        codes.push("    \"\"\"".to_string());
                        codes.push("".to_string());
                    }
                }
            }

            let re = Regex::new(r"def [A-Za-z_]+\((?P<args>[\S_]*)\)").ok()?;
            let add_self_to_no_arg_func = |line: String| {
                if let Some(caps) = re.captures(&line) {
                    if let Some(args) = caps.name("args") {
                        if args.is_empty() {
                            line.replace("()", "(self, *args)")
                        } else {
                            line.replace(args.as_str(), &format!("{}, *args", args.as_str()))
                        }
                    } else {
                        line
                    }
                } else {
                    line
                }
            };

            if !has_doc {
                if let Some(expr) = next_expr {
                    let expr_code = build_helper(map, expr)?;
                    for line in expr_code {
                        codes.push(format!("    {}", add_self_to_no_arg_func(line)));
                    }
                    if !codes.last().unwrap().trim().is_empty() {
                        codes.push("".to_string());
                    }
                }
                // if None
                // it may be an empty class
                // it doesn't report error
                else {
                    codes.push("    pass".to_string());
                }
            }

            for expr in class_members {
                let expr_code = build_helper(map, expr)?;
                for line in expr_code {
                    codes.push(format!("    {}", add_self_to_no_arg_func(line)));
                }
                if !codes.last().unwrap().trim().is_empty() {
                    codes.push("".to_string());
                }
            }
        }
        ExpressionEnum::Function(function) => {
            let mut args_code = String::new();
            let mut ret_code = String::new();
            let mut defaults_iter = function.defaults.iter();
            let mut default_offset = function.args.len() - function.defaults.len();
            for (arg, anno) in function.args_iter() {
                if arg == "return" {
                    ret_code.push_str(&format!(
                        " -> {}",
                        anno.as_ref().map_or("None".to_string(), |a| a.to_string())
                    ));
                    continue;
                }

                if anno.is_none() {
                    args_code.push_str(arg);
                } else {
                    args_code.push_str(&format!("{}: {}", arg, anno.as_ref()?));
                }

                if default_offset == 0 {
                    args_code.push_str(&format!(
                        " = {}",
                        defaults_iter.next()?
                    ));
                } else {
                    // There are no parameters with default values yet.
                    default_offset -= 1;
                }

                args_code.push_str(", ");
            }

            match function.name.as_str() {
                "<lambda>" => {
                    let lambda_args = args_code.trim_end_matches(", ");
                    let lambda_body = build_helper(map, function.bodys.first()?)?.join("");
                    let lambda_body = lambda_body.trim_start_matches("return ");

                    if lambda_body.starts_with("yield") {
                        codes.push(format!(
                            "lambda {}: ({})",
                            lambda_args,
                            lambda_body
                        ));
                    } else {
                        codes.push(format!(
                            "lambda {}: {}",
                            lambda_args,
                            lambda_body
                        ));
                    }
                }
                "<listcomp>" => {
                    codes.push(format!(
                        "[{} for {} in {}]",
                        build_helper(map, function.bodys.first()?)?.join(""),
                        args_code.trim_end_matches(", "),
                        build_helper(map, function.bodys.get(1)?)?.join("")
                    ))
                }
                _ => {
                    let first_line = if function.is_async {
                        format!(
                            "async def {}({}){}:",
                            function.name,
                            args_code.trim_end_matches(", "),
                            ret_code
                        )
                    } else {
                        format!(
                            "def {}({}){}:",
                            function.name,
                            args_code.trim_end_matches(", "),
                            ret_code
                        )
                    };

                    codes.push(first_line);
                    for expr_id in function.bodys.iter() {
                        let expr_code = build_helper(map, expr_id)?;
                        for line in expr_code {
                            codes.push(format!("    {}", line));
                        }
                    }

                    if codes.len() == 1 {
                        codes.push("    pass".to_string());
                    }
                    codes.push("".to_string());
                }
            }
        }
        ExpressionEnum::FastVariable(fast_var) => {
            if fast_var.name == "None" {
                codes.push("".to_string())
            } else if fast_var.name == "0" {
                // do nothing
            } else {
                codes.push(fast_var.name.clone())
            }
        }
        ExpressionEnum::Return(ret) => {
            let value_code = build_helper_some(map, ret.value)?.join("");

            if !value_code.is_empty() {
                codes.push(format!("return {}", value_code))
            }
        }
        ExpressionEnum::Yield(y) => {
            let value_code = build_helper_some(map, y.value)?.join("");

            if !value_code.is_empty() {
                codes.push(format!("yield {}", value_code))
            }
        }
        ExpressionEnum::Assign(assign) => {
            let target_code = build_helper_some(map, assign.target)?.join("");
            let value_code = build_helper_some(map, assign.value)?.join("");

            codes.push(format! {
                "{} {} {}",
                target_code,
                assign.operator,
                value_code
            })
        }
        ExpressionEnum::Alias(alias) => {
            let target_code = build_helper_some(map, alias.target)?.join("");
            let alias_code = build_helper_some(map, alias.alias)?.join("");
            codes.push(format!("{} as {}", target_code, alias_code))
        }
        ExpressionEnum::Try(t) => {
            for id in t.body.iter() {
                let expr_code = build_helper(map, id)?;
                for line in expr_code {
                    codes.push(format!("    {}", line));
                }
            }

            for except in t.except.iter() {
                let expr_code = build_helper(map, except)?;
                codes.extend(expr_code);
            }

            if let Some(id) = t.finally {
                let expr_code = build_helper(map, &id)?;
                codes.extend(expr_code);
            }
        }
        ExpressionEnum::Except(except) => {
            let exception_code = build_helper_some(map, except.exception)?.join("");
            
            if exception_code.is_empty() {
                codes.push("except:".to_string())
            } else {
                codes.push(format!("except {}:", exception_code))
            }

            for id in except.body.iter() {
                let expr_code = build_helper(map, id)?;
                for line in expr_code {
                    codes.push(format!("    {}", line));
                }
            }
        }
        ExpressionEnum::Finally(finally) => {
            codes.push("finally:".to_string());
            for id in finally.body.iter() {
                let expr_code = build_helper(map, id)?;
                for line in expr_code {
                    codes.push(format!("    {}", line));
                }
            }
        }
        ExpressionEnum::Assert(assert) => {
            let test_code = build_helper_some(map, assert.test)?.join("");
            let msg_code = build_helper_some(map, assert.msg)?.join("");

            if msg_code.is_empty() {
                codes.push(format!("assert {}", test_code))
            } else {
                codes.push(format!("assert {}, {}", test_code, msg_code))
            }
        }
        ExpressionEnum::Raise(raise) => {
            let exception_code = build_helper_some(map, raise.exception)?.join("");

            codes.push(format!("raise {}", exception_code))
        }
        ExpressionEnum::Await(a) => {
            let awaitable_code = build_helper_some(map, a.awaitable_expr)?.join("");

            codes.push(format!("await {}", awaitable_code))
        }
        ExpressionEnum::BaseValue(base) => {
            if base.value == "None" {
                codes.push("".to_string())
            } else {
                codes.push(base.value.clone())
            }
        }
        ExpressionEnum::Call(call) => {
            let func_code = build_helper_some(map, call.func)?.join("");
            let mut args_code = vec![];

            for id in call.args.iter() {
                let arg_code = build_helper(map, id)?.join("");
                args_code.push(arg_code);
            }

            if func_code.starts_with("lambda ") {
                codes.push(format!("({})({})", func_code, args_code.join(", ").trim_end_matches(", ")))
            } else {
                codes.push(format!("{}({})", func_code, args_code.join(", ").trim_end_matches(", ")))
            }
        }
        ExpressionEnum::FormatValue(format_value) => {
            let value_code = build_helper_some(map, format_value.value)?.join("");

            codes.push(value_code)
        }
        ExpressionEnum::Format(format) => {
            let mut format_string = String::new();

            for val in format.format_values.iter() {
                let val_code = build_helper(map, val)?.join("");
                
                if val.type_id() == TypeId::of::<FormatValue>() {
                    format_string.push_str(&format!("{{{}}}", val_code))
                } else {
                    format_string.push_str(val_code.trim_matches('\''))
                }
            }

            codes.push(format!(
                "f\"{}\"",
                format_string.replace('"', "\\\"")
            ))
        }
        ExpressionEnum::BinaryOperation(bin_op) => {
            let left_code = build_helper_some(map, bin_op.left)?.join("");
            let right_code = build_helper_some(map, bin_op.right)?.join("");

            codes.push(format!("{} {} {}", left_code, bin_op.operator, right_code))
        }
        ExpressionEnum::UnaryOperation(unary_op) => {
            let operant_code = build_helper_some(map, unary_op.target)?.join("");

            codes.push(format!(
                "{}{}",
                match unary_op.unary_type {
                    UnaryType::Negative => "-",
                    UnaryType::Invert => "~",
                    UnaryType::Not => "not ",
                    UnaryType::Positive => unreachable!(),
                },
                operant_code
            ))
        }
        ExpressionEnum::Import(import) => {
            if import.bk_module.is_none() {
                // no have from
                if import.alias.is_none() {
                    codes.push(format!("import {}", import.module))
                } else {
                    codes.push(format!(
                        "import {} as {}",
                        import.module,
                        import.alias.as_ref()?.trim_end_matches(", ")
                    ))
                }
            } else {
                // have from
                codes.push(format!(
                    "from {} import {}",
                    import.module,
                    import.bk_module.as_ref()?.trim_end_matches(", ")
                ))
            }
        }
        ExpressionEnum::Container(container) => {
            let mut value_codes = vec![];
            for id in container.values.iter() {
                let val_code = build_helper(map, id)?;
                value_codes.extend(val_code);
            }

            value_codes.iter_mut().for_each(|s| {
                if s.is_empty() {
                    *s = "None".to_string()
                }
            });

            match container.container_type {
                ContainerType::List => {
                    codes.push(format!("[{}]", value_codes.join(", ")))
                }
                ContainerType::Tuple => {
                    codes.push(format!("({})", value_codes.join(", ")))
                }
                ContainerType::Set => {
                    codes.push(format!("{{{}}}", value_codes.join(", ")))
                }
                ContainerType::Dict => {
                    let mut kv_codes = vec![];

                    for (k, v) in value_codes.iter().enumerate() {
                        if k % 2 == 0 {
                            kv_codes.push(format!(
                                "{}: {}",
                                v,
                                value_codes.get(k + 1)?
                            ))
                        }
                    }

                    codes.push(format!("{{{}}}", kv_codes.join(", ")))
                }
            }
        }
        ExpressionEnum::Subscr(subscr) => {
            let idx_code = build_helper_some(map, subscr.index)?.join("");
            let target_code = build_helper_some(map, subscr.target)?.join("");

            codes.push(format!("{}[{}]", target_code, idx_code))
        }
        ExpressionEnum::Slice(slice) => {
            let origin_code = build_helper_some(map, slice.origin)?.join("");
            let slice_code = slice
                .slice
                .iter()
                .map(|s| Some(build_helper(map, s)?.join("")))
                .collect::<Option<Vec<_>>>()?
                .join(":");

            codes.push(format!(
                "{}[{}]",
                origin_code,
                slice_code
            ))
        }
        ExpressionEnum::Attribute(attr) => {
            let parent_code = build_helper_some(map, attr.parent)?.join("");
            let attr_code = build_helper_some(map, attr.attr)?.join("");

            codes.push(format!("{}.{}", parent_code, attr_code))
        }
        ExpressionEnum::With(with) => {
            let item_code = build_helper_some(map, with.item)?.join("");
            let first_line = if with.is_async {
                format!("async with {}:", item_code)
            } else {
                format!("with {}:", item_code)
            };

            codes.push(first_line);

            if with.body.is_empty() {
                codes.push("    pass".to_string())
            } else {
                for id in with.body.iter() {
                    let expr_code = build_helper(map, id)?;
                    for line in expr_code {
                        codes.push(format!("    {}", line));
                    }
                }
            }
        }
        ExpressionEnum::If(if_else) => {
            if let Some(id) = if_else.test.as_ref() {
                let test_code = build_helper(map, id)?.join("");
                codes.push(format!("if {}:", test_code))
            } else {
                codes.push("else:".to_string())
            }

            for id in if_else.body.iter() {
                if let Some(jump) = helper!(map, Some(id), as_ref_jump) {
                    if jump.is_backward {
                        codes.push("    continue".to_string())
                    }
                } else {
                    let expr_code = build_helper(map, id)?;
                    for line in expr_code {
                        codes.push(format!("    {}", line));
                    }
                }
            }

            if let Some(or_else) = if_else.or_else.as_ref() {
                let or_else_code = build_helper(map, or_else)?;
                
                if or_else_code.first()?.starts_with("if ") {
                    // elif
                    codes.push(format!("el{}", or_else_code.first()?));
                    codes.extend(or_else_code.into_iter().skip(1))
                } else {
                    // starts with 'else'
                    codes.extend(or_else_code);
                }
            }
        }
        ExpressionEnum::For(f) => {
            let iter_code = build_helper_some(map, f.iterator)?.join("");
            let item_code = build_helper_some(map, f.items)?.join("");

            let first_line = if f.is_async {
                format!("async for {} in {}:", item_code, iter_code)
            } else {
                format!("for {} in {}:", item_code, iter_code)
            };

            codes.push(first_line);

            if f.body.is_empty() {
                codes.push("    pass".to_string())
            } else {
                for id in f.body.iter() {
                    let expr_code = build_helper(map, id)?;
                    for line in expr_code {
                        codes.push(format!("    {}", line));
                    }
                }
            }
        }
        _ => {}
    }

    Some(codes)
}
