use std::{
    fs::File,
    io::{Read, Result},
    vec::Vec,
};

mod bytecode;
use bytecode::operator::{reverse_bytecode, PyLine};

mod color;
use color::color::*;
use color::color_utils::*;

// 用于按行数输出代码
type PyScripts = Vec<PyLine>;

trait PyScript {
    fn new() -> Self;
    fn push(&mut self, py_line: PyLine);
    fn pop(&mut self) -> Option<PyLine>;
    fn show(&mut self);
}

impl PyScript for PyScripts {
    fn new() -> PyScripts {
        Vec::new()
    }

    fn push(&mut self, py_line: PyLine) {
        self.push(py_line);
    }

    fn pop(&mut self) -> Option<PyLine> {
        match self.pop() {
            Some(py_line) => Some(py_line),
            None => None,
        }
    }

    fn show(&mut self) {
        self.sort_by(|a, b| a.line.cmp(&b.line)); // 按行数排序
        let mut now_line = 1;
        for py_line in self.iter() {
            if py_line.pyscript.is_empty() {
                continue;
            }
            // 补充空行
            for _ in now_line..py_line.line {
                println!(
                    "{:>17}: ",
                    now_line
                        .to_string()
                        .to_color_string(&ColorMode::from(FrontColor::Green))
                );
            }
            now_line = py_line.line + 1;
            let put_line = py_line
                .line
                .to_string()
                .to_color_string(&FrontColor::Green.into());
            println!("{:>17}: {}", put_line, py_line.pyscript);
        }
    }
}

// 读取bytecode文件
// 按照空行分割，每一段bytecode就是一个Vec<String>
fn read_bytecode_file(file_name: &str) -> Result<Vec<Vec<String>>> {
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


pub fn setup(file_name: &str) -> Result<()> {
    let bytecode_string = read_bytecode_file(file_name)?;
    let mut codes = PyScripts::new();
    for bcodes in bytecode_string.iter() {
        let code = reverse_bytecode(bcodes);
        codes.push(code);
    }
    codes.show();
    Ok(())
}
