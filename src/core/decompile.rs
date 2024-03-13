use std::collections::HashMap;

use super::ast::*;
use super::common::*;
use super::parse_opcode::*;
use pyrev_ast::*;

pub trait Decompiler {
    fn decompile(&self) -> Result<DecompiledCode>;
    fn merge(&self, mark: &str, maps: &HashMap<String, (Expr, TraceBack)>) -> Result<Expr>;
}

impl Decompiler for CodeObjectMap {
    /// 从字节码对象映射表中解析为AST, 然后再从AST解析为代码
    fn decompile(&self) -> Result<DecompiledCode> {
        let mut decompiled_code = DecompiledCode::new();
        let mut exprs_map = HashMap::new();
        for (mark, code_object) in self.iter() {
            let mut expr = Expr::new();
            let mut trace = TraceBack::new();
            for (_, instruction) in code_object.iter() {
                let (e, t) = Expr::parse(instruction)?;
                //dbg!(&e);
                expr.extend(*e);
                trace.extend(t);
            }

            #[cfg(debug_assertions)]
            {
                println!("{:?}", &trace);
            }

            exprs_map.insert(mark.clone(), (expr, trace));
        }
        //dbg!(&exprs_map);
        let main_expr = self.merge("<main>", &exprs_map)?;
        //dbg!(&main_expr);

        for (i, instruction) in main_expr.bodys.iter().enumerate() {
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

    /// 用来合并所有的Expr
    /// 比如`<main>`有一个函数foo, 就需要把foo的定义合并到`<main>`里面的foo Function的 bodys
    fn merge(&self, mark: &str, maps: &HashMap<String, (Expr, TraceBack)>) -> Result<Expr> {
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

                    function.with_mut().patch_by(|f| {
                        f.bodys = new_bodys;
                        traceback.locals.iter().for_each(|(k, (v, b))| {
                            if !b {
                                f.args.push(FastVariable {
                                    index: *k,
                                    name: v.to_string(),
                                    annotation: None,
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
                    .locals
                    .clone();
                let args = function.args.clone();
                let mut function_args = HashMap::new();
                for (k, (v, b)) in function_locals.iter() {
                    if !b {
                        function_args.insert(v, (k, None));
                    }
                }
                for fv in args.iter() {
                    if function_args.contains_key(&fv.name) {
                        function_args.get_mut(&fv.name).unwrap().1 = fv.annotation.clone();
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
                        })
                    }
                })?;
                function
                    .with_mut()
                    .get_mut()
                    .args
                    .sort_by(|a, b| a.index.cmp(&b.index));

                #[cfg(debug_assertions)]
                {
                    println!("{:?}", &function);
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

                    class.with_mut().patch_by(|c| {
                        c.members = new_members;
                    })?;

                    is_merged = false;
                }
            }

            //dbg!(&this_expr);
            if is_merged {
                break;
            }
        }
        Ok(this_expr.to_owned())
    }
}

#[derive(Debug, PartialEq)]
pub struct DecompiledCode {
    code: Vec<(LineNumber, String)>,
}

#[allow(unused)]
impl DecompiledCode {
    pub fn new() -> Self {
        Self { code: Vec::new() }
    }

    pub fn insert<S: AsRef<str>>(&mut self, l: usize, s: S) {
        self.code.push((l, s.as_ref().to_string()));
    }

    pub fn iter(&mut self) -> impl Iterator<Item = (usize, &std::string::String)> + Clone {
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
        let code_objects = input.to_string().parse().unwrap();
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
