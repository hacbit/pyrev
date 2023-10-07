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

// 用于储存一段代码
trait PyScript {
    fn new() -> Self;
    fn push(&mut self, py_line: PyLine);
    fn pop(&mut self) -> Option<PyLine>;
    fn optimize(&mut self) -> PyScripts;
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

    // 优化代码
    fn optimize(&mut self) -> PyScripts {
        self.sort_by(|a, b| a.line.cmp(&b.line)); // 按行数排序
                                                  // 把相同行数的取出放在一个buffer里
        let mut buffer: Vec<PyLine> = vec![];
        let mut now_line = 1;
        let mut now_retraction = 0;
        let mut new_pyscripts = PyScripts::new();
        for py_line in self.iter() {
            if py_line.line == now_line {
                buffer.push(py_line.clone());
            }
            now_line = py_line.line;

            // 如果buffer里有多个元素，就进行优化
            if buffer.len() > 1 {
                /* let mut new_pyscript = String::new();
                for py_line in buffer.iter() {
                    println!("ab {}: {}", py_line.line, py_line.pyscript);
                    new_pyscript.push_str(&py_line.pyscript);
                }
                new_pyscripts.push(PyLine::new(now_line, new_pyscript, py_line.start_offset, None)); */
            } else {
                new_pyscripts.push(py_line.clone());
            }
            buffer.clear();

            let mut this_pyscript = new_pyscripts.pop().unwrap();
            if this_pyscript.pyscript.contains("goto") {
                this_pyscript.pyscript = this_pyscript.pyscript.split("goto").last().unwrap().to_string();
                this_pyscript.retractions = now_retraction;
                now_retraction -= 1;
                new_pyscripts.push(this_pyscript);
            }
        }
        // println!("new_pyscripts: {:?}", new_pyscripts);
        // 以更好的格式输出结构体
        dbg!(&new_pyscripts);
        new_pyscripts
    }

    fn show(&mut self) {
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
fn read_bytecode_file(file: &mut File) -> Result<Vec<Vec<String>>> {
    let mut bytecode_string = String::new();
    file.read_to_string(&mut bytecode_string)?;
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

pub fn setup(file: &mut File) -> Result<()> {
    let bytecode_string = read_bytecode_file(file)?;
    let mut codes = PyScripts::new();
    for bcodes in bytecode_string.iter() {
        let code = reverse_bytecode(bcodes);
        codes.push(code);
    }
    codes.optimize().show();
    Ok(())
}
