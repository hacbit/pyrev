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

    pub fn insert<S: AsRef<str>>(&mut self, l: usize, s: S) -> &mut Self {
        self.code.push((l, s.as_ref().to_string()));
        self
    }

    pub fn iter(&mut self) -> impl Iterator<Item = (usize, &String)> + Clone {
        self.code.iter().map(|(i, s)| (*i, s))
    }
}
