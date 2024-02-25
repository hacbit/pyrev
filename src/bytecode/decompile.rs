use pyrev_ast::*;
use super::ast::*;
use super::opcode::{Opcode, OpcodeInstruction};
use super::parse_opcode::*;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Decompiler {
    fn decompile(&self) -> Result<DecompiledCode>;
}

impl Decompiler for CodeObjectMap {
    fn decompile(&self) -> Result<DecompiledCode> {
        let mut decompiled_code = DecompiledCode::new();
        let main_code_object = self.get("<main>").ok_or("main code object not found")?;
        let mut main_expr = Expr::new();
        for (_, instruction) in main_code_object.iter() {
            let expr = Expr::parse(instruction)?;
            main_expr.extend(*expr);
        }

        todo!()
    }
}

pub struct DecompiledCode {
    code: Vec<(usize, String)>,
}

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
