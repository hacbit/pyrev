use std::collections::HashMap;
use std::fmt::Debug;

use crate::parse_opcode::LineNumber;

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
            // dbg!(&exprs_map);
        }

        let mut main_expr = merge("<main>", &exprs_map)?;
        #[cfg(debug_assertions)]
        {
            // dbg!(&main_expr);
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
                    function.with_mut_unchecked().patch_by(|mut f| {
                        f.is_async = true;
                    })?;
                }
            }
        }
    }

    Ok(())
}

/// 用来合并所有的Expr
///
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
                    .clone();

                function.with_mut_unchecked().patch_by(|mut f| {
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

            function.with_mut_unchecked().patch_by(|mut f| {
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
                .with_mut_unchecked()
                .patch_by(|mut f| f.args.sort_by(|a, b| a.index.cmp(&b.index)))?;

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
                    .clone();

                class.with_mut_unchecked().patch_by(|mut c| {
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
                for_loop.with_mut_unchecked().patch_by(|mut f| {
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
        expr.with_mut_unchecked().patch_by(|mut e| {
            e.bodys.remove(*idx);
        })?;
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
