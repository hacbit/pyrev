use std::collections::HashMap;

use super::ast::*;
use super::parse_opcode::*;
use pyrev_ast::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Decompiler {
    fn decompile(&self) -> Result<DecompiledCode>;
    fn merge(&self, mark: &str, maps: &HashMap<String, Expr>) -> Result<Expr>;
}

impl Decompiler for CodeObjectMap {
    /// 从字节码对象映射表中解析为AST, 然后再从AST解析为代码
    fn decompile(&self) -> Result<DecompiledCode> {
        let mut decompiled_code = DecompiledCode::new();
        let mut exprs_map = HashMap::new();
        for (mark, code_object) in self.iter() {
            let mut expr = Expr::new();
            for (_, instruction) in code_object.iter() {
                let e = Expr::parse(instruction)?;
                //dbg!(&e);
                expr.extend(*e);
            }
            exprs_map.insert(mark.clone(), expr);
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
    /// 比如<main>有一个函数foo, 就需要把foo的定义合并到<main>里面的foo Function的 bodys
    fn merge(&self, mark: &str, maps: &HashMap<String, Expr>) -> Result<Expr> {
        let this_expr = maps.get(mark).ok_or(format!("No {} expr", &mark))?.clone();
        loop {
            let mut is_merged = true;
            let function_query = this_expr.query::<Function>();
            for function in function_query {
                if function.bodys.is_empty() {
                    let new_bodys = maps
                        .get(&function.mark)
                        .ok_or(format!("No {} expr", &function.mark))?
                        .bodys
                        .clone();
                    // 想不到怎么实现 query_mut, 先用unsafe
                    let func_ptr = function as *const Function as *mut Function;
                    unsafe {
                        (*func_ptr).bodys.extend(new_bodys);
                    }
                    is_merged = false;
                }
            }
            //dbg!(&this_expr);
            if is_merged {
                break;
            }
        }
        Ok(this_expr)
    }
}

/// 弃用的API
/* fn process_expression(expr: &mut ExpressionEnum, maps: HashMap<String, Expr>) {
    match expr {
        ExpressionEnum::Function(f) => {
            f.bodys = maps.get(&f.mark).unwrap().bodys.clone();
        }
        _ => process_expression(expr, maps),
    }
} */

#[derive(Debug)]
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
        dbg!(expr);
        assert!(false);
    }
}
