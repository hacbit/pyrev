//! This crate provides some utilities for building the AST to a Python code.

#![feature(concat_idents)]

use pyrev_ast::*;
use pyrev_query::*;
use regex::Regex;

pub fn get_helper<'a>(
    map: &'a Map,
    expr_id: Option<&QueryId>,
) -> Option<&'a ExpressionEnum> {
    expr_id.and_then(|id| map.get_single(*id))
}

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

pub fn build_helper_some(map: &Map, id: Option<QueryId>) -> Option<Vec<String>> {
    let expr = map.get_single(id?)?;
    build(map, expr)
}

pub fn build_helper(map: &Map, id: &QueryId) -> Option<Vec<String>> {
    let expr = map.get_single(*id)?;
    build(map, expr)
}

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
        _ => {}
    }

    Some(codes)
}
