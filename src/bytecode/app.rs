use super::opcode::*;
use super::parse_opcode::*;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Resources = HashMap<usize, Vec<OpcodeInstruction>>;

pub struct App {
    files: Vec<PathBuf>,
    output: Vec<PathBuf>,
    // the resource of the bytecode
    resources: Vec<Resources>,
}

impl App {
    pub fn new() -> Self {
        App {
            files: vec![],
            output: vec![],
            resources: vec![],
        }
    }

    pub fn insert_resources(&mut self, ifile: PathBuf) -> &mut Self {
        self.files.push(ifile);
        self
    }

    pub fn run(&mut self) -> Result<()> {
        for file in &self.files {
            let content = fs::read_to_string(file)?;
            let resources = content.parse()?;
            self.resources.push(resources);
        }

        // todo: decompile the bytecode

        Ok(())
    }
}
