use super::types::*;

use atty::Stream;
use colored::Colorize;
use lazy_format::lazy_format;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BytecodeLine {
    offset: usize,
    bytecode: BytecodeType,
    arg: Option<usize>,
    real_arg: Option<String>,
    jump_offset: Option<usize>,
}

#[derive(Debug)]
pub struct PyObj {
    line: usize,
    retractions: usize,
    bytecode_lines: Vec<Option<BytecodeLine>>,
}

#[derive(Debug)]
pub struct PyScript {
    line: usize,
    script: Option<String>,
}

impl Iterator for PyObj {
    type Item = PyObj;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl From<&str> for PyObj {
    fn from(py_line_str: &str) -> Self {
        let re = Regex::new(
            r#"(?x)
            (?P<line>\d+)?          # line  (optional)
            ([\s>]+)?
            (?P<off>\d+)            # offset
            [\s+]
            (?P<bc>[A-Z0-9_]+)      # bytecode
            ([\ ]+)?
            (?P<a>\d+)?             # arg   (optional)
            [\s+]?
            (\((?P<ra>.+)\))?       # real arg  (optional)
            "#,
        )
        .unwrap();
        let mut line = 0;
        let bytecode_lines = re
            .captures_iter(py_line_str)
            .map(|cap| {
                if let Some(l) = cap.name("line") {
                    line = l.as_str().parse::<usize>().unwrap_or(0)
                }
                let group = (
                    cap.name("off"),
                    cap.name("bc"),
                    cap.name("a"),
                    cap.name("ra"),
                );
                match group {
                    (Some(off), Some(bc), a, ra) => Some(BytecodeLine {
                        offset: off.as_str().parse::<usize>().unwrap(),
                        bytecode: BytecodeType::get(bc.as_str()),
                        arg: a.map(|a| a.as_str().parse::<usize>().unwrap()),
                        real_arg: ra.map(|ra| ra.as_str().to_string()),
                        jump_offset: None,
                    }),
                    _ => None,
                }
            })
            .collect::<Vec<Option<BytecodeLine>>>();
        PyObj {
            line,
            retractions: 0,
            bytecode_lines,
        }
    }
}

impl PyObj {
    pub fn to_python(&self) -> String {
        let mut stack: Vec<Option<String>> = Vec::new();
        // let mut is_import = false;
        let mut idx: usize = 0;
        loop {
            let this = self.bytecode_lines.get(idx);
            if this.is_none() {
                break stack.pop().unwrap().unwrap();
            }
            let this = this.unwrap().as_ref().unwrap();
            let bytecode = this.bytecode;
            let arg = this.arg;
            let real_arg = this.real_arg.clone();
            // println!("stack: {:?}", stack);
            match bytecode {
                BytecodeType::Load => {
                    if let Some(real_arg) = real_arg {
                        if real_arg == "None" {
                            stack.push(None);
                        } else {
                            stack.push(Some(real_arg));
                        }
                    } else {
                        stack.push(None);
                    }
                }
                /* BytecodeType::Push => stack.push(None),
                BytecodeType::Pop => {
                    stack.pop();
                } */
                BytecodeType::Unary(unary) => {
                    let expr = stack.pop().unwrap().unwrap();
                    stack.push(Some(unary.get_expr(&expr)));
                }
                BytecodeType::Binary(binary) => match binary {
                    Binary::Op | Binary::Compare => {
                        let op = OP::get(real_arg.unwrap().as_str()).unwrap();
                        let right = stack.pop().unwrap().unwrap();
                        let left = stack.pop().unwrap().unwrap();
                        stack.push(Some(op.get_expr(&left, &right)));
                    }
                    Binary::Subscr => {
                        let idx = stack.pop().unwrap().unwrap();
                        let target = stack.pop().unwrap().unwrap();
                        stack.push(Some(format!("{}[{}]", target, idx)));
                    }
                    Binary::Slice => {
                        let end = stack.pop().unwrap().unwrap();
                        let start = stack.pop().unwrap().unwrap();
                        /* if start == "None" {
                            start.clear();
                        }
                        if end == "None" {
                            end.clear();
                        } */
                        let target = stack.pop().unwrap().unwrap();
                        stack.push(Some(format!("{}[{}:{}]", target, start, end)));
                    }
                    /*
                    当 IS_OP 或者 CONTAINS_OP 的 arg 为 0 时，表示in/is，为1时表示 not in/is not
                     */
                    Binary::In => {
                        let op = if arg.unwrap() == 0 { "in" } else { "not in" };
                        let right = stack.pop().unwrap().unwrap();
                        let left = stack.pop().unwrap().unwrap();
                        stack.push(Some(format!("{} {} {}", left, op, right)));
                    }
                    Binary::Is => {
                        let op = if arg.unwrap() == 0 { "is" } else { "is not" };
                        let right = stack.pop().unwrap().unwrap();
                        let left = stack.pop().unwrap().unwrap();
                        stack.push(Some(format!("{} {} {}", left, op, right)));
                    } // END BINARY
                },
                BytecodeType::Call => {
                    let mut args = vec![];
                    let arg_count = arg.unwrap();

                    // call 对应的arg是参数的个数
                    for _ in 0..arg_count {
                        args.push(stack.pop().unwrap().unwrap());
                    }
                    args.reverse(); // 逆序
                    let arg_str = args.join(", ");
                    let func = stack.pop().unwrap().unwrap();
                    stack.push(Some(format!("{}({})", func, arg_str)));
                }
                BytecodeType::Build(value_type) => {
                    if arg.unwrap() == 0 {
                        stack.push(Some(value_type.build(None)))
                    } else {
                        let mut args = vec![];
                        for _ in 0..arg.unwrap() {
                            args.push(stack.pop().unwrap().unwrap());
                        }
                        args.reverse();
                        stack.push(Some(value_type.build(Some(args.join(", ").as_str()))));
                    }
                }
                BytecodeType::Import(import_type) => {
                    let old_expr = stack.pop().unwrap();
                    if let Some(old_expr) = old_expr {
                        match import_type {
                            ImportType::ImportName => {
                                stack.push(Some(format!("import {}", real_arg.unwrap())))
                            }
                            ImportType::ImportFrom => {
                                if old_expr.starts_with("import") {
                                    let import_name = old_expr
                                        .split_whitespace()
                                        .collect::<Vec<&str>>()
                                        .pop()
                                        .unwrap();
                                    if import_name.split('.').collect::<Vec<&str>>().pop().unwrap()
                                        != real_arg.as_ref().unwrap()
                                    {
                                        stack.push(Some(format!(
                                            "from {} import {}",
                                            import_name,
                                            real_arg.unwrap()
                                        )));
                                    } else {
                                        stack.push(Some(old_expr))
                                    }
                                } else {
                                    stack.push(Some(format!("{}, {}", old_expr, real_arg.unwrap())))
                                }
                            }
                        }
                    } else {
                        stack.push(Some(format!("import {}", real_arg.unwrap())))
                    }
                }
                BytecodeType::Extend => {
                    // Only LIST_EXTEND
                    let etn = stack.pop().unwrap().unwrap();
                    stack.pop();
                    stack.push(Some(format!("[{}]", &etn[1..etn.len() - 1])))
                }
                BytecodeType::Store => {
                    let value = stack.pop().unwrap().unwrap();
                    let name = real_arg.unwrap();
                    if value.contains("import") {
                        let import_ = value
                            .split_whitespace()
                            .collect::<Vec<&str>>()
                            .pop()
                            .unwrap();
                        if import_ != name {
                            stack.push(Some(value + " as " + &name))
                        } else {
                            stack.push(Some(value));
                        }
                    } else {
                        stack.push(Some(format!("{} = {}", name, value)));
                    }
                }
                BytecodeType::Return => {}
                // Other 是不稳定模块， 懒得分类或者不好分类的，塞了一些不常用的指令
                BytecodeType::Other => {
                    if let Some("INTRINSIC_IMPORT_STAR") = real_arg.as_deref() {
                        let expr = stack.pop().unwrap().unwrap();
                        assert!(expr.starts_with("import"));
                        let import_name = expr
                            .split_whitespace()
                            .collect::<Vec<&str>>()
                            .pop()
                            .unwrap();
                        stack.push(Some(format!("from {} import *", import_name)));
                    }
                }
                _ => (),
            }
            idx += 1;
        }
    }
}

#[derive(Debug)]
pub struct PyCodeString {
    code: String,
}

impl From<String> for PyCodeString {
    fn from(code: String) -> Self {
        PyCodeString { code }
    }
}

impl PyCodeString {
    pub fn to_pyobjs(&self) -> PyObjs {
        let pyobjs = self
            .code
            .trim()
            .split('\n')
            .collect::<Vec<&str>>()
            .split(|line| line.trim().is_empty())
            .skip(1)
            .map(|lines| PyObj::from(lines.join("\n").as_str()))
            .collect::<Vec<PyObj>>();
        /* let mut stack: Vec<(usize, usize)> = Vec::new();
        let lines = pyobjs.iter().map(|pyobj| pyobj.line).collect::<Vec<usize>>();
        // 相同行号的两段代码之间的代码缩进加1
        lines.iter().enumerate().for_each(|(idx, line)| {
            for (i, l) in lines[idx + 1..].iter().enumerate() {
                if line == l {
                    stack.push((idx, i)); // start, end
                }
            }
        });
        for (start, end) in stack.iter() {
            for pyobj in pyobjs.iter_mut().skip(*start + 1).take(*end) {
                pyobj.retractions += 1;
            }
        } */
        // 返回的pyobj中，相同行号的按照顺序拼接
        let mut map: HashMap<usize, Vec<Vec<Option<BytecodeLine>>>> = HashMap::new();
        for pyobj in pyobjs {
            map.entry(pyobj.line)
                .or_default()
                .push(pyobj.bytecode_lines);
        }
        let mut pyobjs = map
            .iter()
            .map(|(line, bytecode_lines)| PyObj {
                line: *line,
                retractions: 0,
                bytecode_lines: bytecode_lines
                    .iter()
                    .flatten()
                    .cloned()
                    .collect::<Vec<Option<BytecodeLine>>>(),
            })
            .collect::<Vec<PyObj>>();
        pyobjs.sort_by(|a, b| a.line.cmp(&b.line));
        PyObjs { objs: pyobjs }
    }
}

#[derive(Debug)]
pub struct PyObjs {
    objs: Vec<PyObj>,
}

impl PyObjs {
    // 将PyObj转换为PyScript
    pub fn to_pyscript(&self) -> Vec<PyScript> {
        let mut pyscripts: Vec<PyScript> = Vec::new();
        for pyobj in self.objs.iter() {
            let script = pyobj.to_python();
            pyscripts.push(PyScript {
                line: pyobj.line,
                script: Some("    ".repeat(pyobj.retractions) + script.as_str()),
            });
        }
        pyscripts
    }
}

// 正常输出到终端
fn display_pycode_with_line(pyscripts: &[PyScript]) -> Result<()> {
    let max_line = pyscripts[pyscripts.len() - 1].line.to_string().len() + 1;
    for PyScript { line, script } in pyscripts.iter() {
        print!(
            "{}{} ",
            lazy_format!(
                "{:>max_line$}",
                line.to_string().bright_green(),
                max_line = max_line
            ),
            "|".bright_blue(),
        );
        if let Some(script) = script {
            println!("{}", script);
        } else {
            println!();
        }
    }
    Ok(())
}

// 重定向标准输出流时，不输出行号
fn display_pycode_without_line(pyscripts: &[PyScript]) -> Result<()> {
    for PyScript { script, .. } in pyscripts.iter() {
        if let Some(script) = script {
            println!("{}", script);
        } else {
            println!();
        }
    }
    Ok(())
}

pub fn display_pycode(pyscripts: &[PyScript]) -> Result<()> {
    // 判断是否重定向了标准输出流
    if atty::is(Stream::Stdout) {
        display_pycode_with_line(pyscripts)?;
    } else {
        display_pycode_without_line(pyscripts)?;
    }
    Ok(())
}

pub fn read_file(file_name: &PathBuf) -> Result<String> {
    if !Path::new(file_name).exists() {
        return Err(format!("file {:?} not found", file_name).into());
    }
    let mut file = File::open(file_name).unwrap_or_else(|err| {
        eprintln!(
            "[{}] Application error: {err}",
            "x".red(),
            err = err.to_string().red(),
        );
        panic!();
    });
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    Ok(content)
}

// 如果在args中指定了输出文件名，则尝试写入到指定文件中
pub fn write_file(file_name: &PathBuf, pyscripts: &[PyScript]) -> Result<()> {
    if File::open(file_name).is_ok() {
        return Err(format!("file {:?} already exists", file_name).into());
    }
    let mut file = File::create(file_name)?;
    for PyScript { script, .. } in pyscripts.iter() {
        if let Some(script) = script {
            file.write_all(script.as_bytes())?;
        }
        file.write_all("\n".as_bytes())?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_pyobj() {
        let path = PathBuf::from("./test/code.txt");
        let content = read_file(&path).unwrap();
        let pyobjs = PyCodeString::from(content).to_pyobjs();
        dbg!(pyobjs);
        assert!(false);
    }
}
