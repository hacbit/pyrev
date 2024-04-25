use std::collections::HashMap;
use std::fmt::Debug;

use super::prelude::*;
use pyrev_ast::*;

pub trait Decompiler {
    fn decompile(&self) -> Result<DecompiledCode>;
    #[allow(unused)]
    fn optimize(&self, expr: &Expr) -> Result<Expr>;
}

impl Decompiler for CodeObjectMap {
    /// 从字节码对象映射表中解析为AST, 然后再从AST解析为代码
    fn decompile(&self) -> Result<DecompiledCode> {
        let mut decompiled_code = DecompiledCode::default();
        let mut exprs_map = HashMap::new();
        for (mark, code_object) in self.iter() {
            let expr = Expr::parse(code_object)?;
            let trace = get_trace(code_object)?;

            exprs_map.insert(mark.clone(), (*expr, trace));
        }
        #[cfg(debug_assertions)]
        {
            //dbg!(&exprs_map);
        }

        let mut main_expr = merge("<main>", &exprs_map)?;
        #[cfg(debug_assertions)]
        {
            //dbg!(&main_expr);
        }
        fixed_async_object(&mut main_expr, &exprs_map)?;

        for (i, instruction) in main_expr.iter().enumerate() {
            let code = instruction
                .build()?
                .iter()
                .enumerate()
                .map(|(l, s)| (l + i, s.to_string()))
                .collect::<Vec<_>>();
            decompiled_code.code.extend(code);
        }
        Ok(decompiled_code)
    }

    fn optimize(&self, expr: &Expr) -> Result<Expr> {
        Ok(expr.to_owned())
    }
}

fn fixed_async_object(
    main_expr: &mut Expr,
    maps: &HashMap<String, (Expr, TraceBack)>,
) -> Result<()> {
    // fixed async object
    for (mark, (_, trace)) in maps.iter() {
        if trace.asyncable() {
            let function_query = main_expr.query::<Function>();
            for function in function_query {
                if &function.mark == mark {
                    function.with_mut().patch_by(|f| {
                        f.is_async = true;
                    })?;
                }
            }
        }
    }

    Ok(())
}

/// 用来合并所有的Expr
/// 比如`<main>`有一个函数foo, 就需要把foo的定义合并到`<main>`里面的foo Function的 bodys
fn merge(mark: &str, maps: &HashMap<String, (Expr, TraceBack)>) -> Result<Expr> {
    let (this_expr, traceback) = maps.get(mark).ok_or(format!("No {} expr", &mark))?;

    loop {
        let mut is_merged = true;

        // merge the function
        let function_query = this_expr.query::<Function>();
        for function in function_query {
            if function.bodys.is_empty() {
                let new_bodys = maps
                    .get(&function.mark)
                    .ok_or(format!("No {} expr", &function.mark))?
                    .0
                    .bodys
                    .clone()
                    .into_inner();

                function.with_mut().patch_by(|f| {
                    f.bodys = new_bodys;
                    traceback
                        .get_locals()
                        .iter()
                        .for_each(|(k, Local { name, is_arg, .. })| {
                            if *is_arg {
                                f.args.push(FastVariable {
                                    index: *k,
                                    name: name.to_owned(),
                                    annotation: None,
                                    ..Default::default()
                                })
                            }
                        })
                })?;

                is_merged = false;
            }

            // update the function arguments
            let function_locals = maps
                .get(&function.mark)
                .ok_or(format!("No {} expr", &function.mark))?
                .1
                .get_locals()
                .clone();
            let args = function.args.clone();
            let mut function_args = HashMap::new();
            for (k, Local { name, is_arg, .. }) in function_locals.iter() {
                if *is_arg && !function_args.contains_key(name) {
                    function_args.insert(name, (k, None));
                }
            }
            for fv in args.iter() {
                if function_args.contains_key(&fv.name) {
                    function_args
                        .get_mut(&fv.name)
                        .unwrap()
                        .1
                        .clone_from(&fv.annotation)
                } else {
                    function_args.insert(&fv.name, (&fv.index, fv.annotation.clone()));
                }
            }

            function.with_mut().patch_by(|f| {
                f.args.clear();
                for (arg, (idx, anno)) in function_args.iter() {
                    f.args.push(FastVariable {
                        index: **idx,
                        name: arg.to_string(),
                        annotation: anno.clone(),
                        ..Default::default()
                    })
                }
            })?;
            function
                .with_mut()
                .patch_by(|f| f.args.sort_by(|a, b| a.index.cmp(&b.index)))?;

            #[cfg(debug_assertions)]
            {
                //dbg!(&function);
            }
        }

        // merge the class
        let class_query = this_expr.query::<Class>();
        for class in class_query {
            if class.members.is_empty() {
                let new_members = maps
                    .get(&class.mark)
                    .ok_or(format!("No {} expr", &class.mark))?
                    .0
                    .bodys
                    .clone()
                    .into_inner();

                class.with_mut().patch_by(|c| {
                    c.members = new_members;
                })?;

                is_merged = false;
            }
        }

        // merge the for loop
        let for_query = this_expr.query::<For>();
        let mut want_to_removes = Vec::new();
        for for_loop in for_query {
            if for_loop.body.is_empty() {
                let (new_body, want_to_remove) =
                    find_expr_among(this_expr, for_loop.from, for_loop.to)?;
                for_loop.with_mut().patch_by(|f| {
                    f.body = new_body;
                })?;
                want_to_removes.extend(want_to_remove);
            }
        }
        commit_expr(this_expr, &want_to_removes)?;

        //dbg!(&this_expr);
        if is_merged {
            break;
        }
    }
    Ok(this_expr.to_owned())
}

fn find_expr_among(
    expr: &Expr,
    offset: usize,
    target_offset: usize,
) -> Result<(Vec<ExpressionEnum>, Vec<usize>)> {
    let mut res = Vec::new();
    let mut want_to_remove = Vec::new();
    for (i, e) in expr.iter().enumerate() {
        let (start, end) = e.get_offset();
        #[cfg(debug_assertions)]
        {
            //dbg!(start, end, offset, target_offset);
        }
        if start > offset && end < target_offset {
            res.push(e.to_owned());
            want_to_remove.push(i);
        }
    }
    Ok((res, want_to_remove))
}

fn commit_expr(expr: &Expr, want_to_remove: &[usize]) -> Result<()> {
    for idx in want_to_remove.iter().rev() {
        expr.bodys.borrow_mut().remove(*idx);
    }
    Ok(())
}

#[derive(Debug, PartialEq, Default)]
pub struct DecompiledCode {
    code: Vec<(LineNumber, String)>,
}

#[allow(unused)]
impl DecompiledCode {
    pub fn insert<S: AsRef<str>>(&mut self, l: usize, s: S) {
        self.code.push((l, s.as_ref().to_string()));
    }

    pub fn iter(&mut self) -> impl Iterator<Item = (usize, &std::string::String)> + Clone + Debug {
        self.code.iter().map(|(i, s)| (*i, s))
    }
}

#[cfg(test)]
mod tests {
    use super::super::parse_opcode::*;
    use super::*;

    #[test]
    fn test_parse_code_object() {
        let input = r#"  0           0 RESUME                   0

        2           2 LOAD_CONST               0 (<code object foo at 0x00000223C0B267F0, file "<dis>", line 2>)
                    4 MAKE_FUNCTION            0
                    6 STORE_NAME               0 (foo)
      
        4           8 PUSH_NULL
                   10 LOAD_NAME                1 (print)
                   12 PUSH_NULL
                   14 LOAD_NAME                0 (foo)
                   16 PRECALL                  0
                   20 CALL                     0
                   30 PRECALL                  1
                   34 CALL                     1
                   44 POP_TOP
                   46 LOAD_CONST               1 (None)
                   48 RETURN_VALUE
      
      Disassembly of <code object foo at 0x00000223C0B267F0, file "<dis>", line 2>:
        2           0 RESUME                   0
      
        3           2 LOAD_CONST               1 (1)
                    4 RETURN_VALUE"#;
        let code_objects = input.parse_opcode().unwrap();
        //dbg!(code_object);
        let expr = code_objects.decompile().unwrap();
        //dbg!(expr);
        //assert!(false);
        assert_eq!(
            expr,
            DecompiledCode {
                code: vec![
                    (0, "def foo():".into(),),
                    (1, "    return 1".into(),),
                    (1, "print(foo())".into(),),
                ],
            }
        )
    }
}
