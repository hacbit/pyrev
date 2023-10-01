use std::{
    fs::File,
    io::{self, ErrorKind, Read, Result, Write},
    vec::Vec,
};

mod bytecodemap;
use bytecodemap::*;

#[allow(unused)]
struct ScriptLine {
    script_line_number: u32,
    script_line: String,
}

#[allow(dead_code)]
type Scripts = Vec<ScriptLine>;

#[allow(unused)]
impl ScriptLine {
    fn new() -> ScriptLine {
        ScriptLine {
            script_line_number: 0,
            script_line: String::new(),
        }
    }

    fn write_python(&self, python_script: &mut File, now_line_number: &mut u32) -> Result<()> {
        if self.script_line_number == *now_line_number {
            python_script.write(self.script_line.as_bytes())?;
        } else if self.script_line_number < *now_line_number {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                "script_line_number is not monotonic increasing",
            ));
        } else {
            python_script.write(b"\n")?;
            *now_line_number += 1;
            self.write_python(python_script, now_line_number)?;
        }
        Ok(())
    }
}

pub fn read_bytecode_file(file_name: &str) -> Result<Vec<Vec<String>>> {
    let mut bytecode_string = String::new();
    File::open(file_name)?.read_to_string(&mut bytecode_string)?;
    let mut bytecode: Vec<_> = vec![];
    let mut buffer: Vec<String> = vec![];
    for line in bytecode_string.lines() {
        if line.is_empty() {
            bytecode.push(buffer);
            buffer = vec![];
            continue;
        }
        buffer.push(line.to_string());
    }
    if !buffer.is_empty() {
        bytecode.push(buffer);
    }
    Ok(bytecode)
}

// 提供API给外部调用
pub fn setup(file_name: &str) -> Result<()> {
    let bytecode_string = read_bytecode_file(file_name)?;
    // let mut pyscript = File::create("code.py")?;
    for bcodes in bytecode_string.iter() {
        let code = test(bcodes);
        println!("{}", code);
    }
    Ok(())
}
