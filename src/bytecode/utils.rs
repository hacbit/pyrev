use super::binary::*;
use super::bytecode_type::BytecodeType;

use atty::Stream;
use colored::Colorize;
use lazy_format::lazy_format;
use regex::Regex;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
struct BytecodeLine {
    offset: u32,
    bytecode: BytecodeType,
    arg: Option<u32>,
    real_arg: Option<String>,
    jump_offset: Option<u32>,
}

#[derive(Debug)]
pub struct PyObj {
    line: usize,
    bytecode_lines: Vec<Option<BytecodeLine>>,
}

#[derive(Debug)]
pub struct PyScript {
    line: usize,
    script: Option<String>,
}

#[allow(unused)]
impl From<&str> for PyObj {
    fn from(py_line_str: &str) -> Self {
        let re = Regex::new(
            r#"(?x)
            (?P<line>\d+)?      # line  (optional)
            ([\s>]+)?
            (?P<off>\d+)        # offset
            [\s+]
            (?P<bc>[A-Z_]+)     # bytecode
            ([\ ]+)?
            (?P<a>\d+)?         # arg   (optional)
            [\s+]?
            (\((?P<ra>.+)\))?   # real arg  (optional)
            "#,
        )
        .unwrap();
        let mut line = 0;
        let bytecode_lines = re
            .captures_iter(py_line_str)
            .map(|cap| {
                if let Some(l) = cap.name("line") {
                    line = l.as_str().parse::<usize>().unwrap();
                }
                let group = (
                    cap.name("off"),
                    cap.name("bc"),
                    cap.name("a"),
                    cap.name("ra"),
                );
                match group {
                    (Some(off), Some(bc), a, ra) => Some(BytecodeLine {
                        offset: off.as_str().parse::<u32>().unwrap(),
                        bytecode: BytecodeType::get(bc.as_str()),
                        arg: a.map(|a| a.as_str().parse::<u32>().unwrap()),
                        real_arg: ra.map(|ra| ra.as_str().to_string()),
                        jump_offset: None,
                    }),
                    _ => None,
                }
            })
            .collect::<Vec<Option<BytecodeLine>>>();
        PyObj {
            line,
            bytecode_lines,
        }
    }
}

#[allow(unused)]
impl PyObj {
    pub fn to_python(&self) -> String {
        let mut python_code: String = String::new();
        let mut stack: Vec<Option<String>> = Vec::new();
        let mut now_retraction: u32 = 0;
        let mut idx: usize = 0;
        loop {
            let this = self.bytecode_lines.get(idx);
            if this.is_none() {
                break stack
                    .into_iter()
                    .flatten()
                    .collect::<Vec<String>>()
                    .join("\n");
            }
            let this = this.unwrap().as_ref().unwrap();
            let offset = this.offset;
            let bytecode = this.bytecode;
            let arg = this.arg;
            let real_arg = this.real_arg.clone();
            let jump_offset = this.jump_offset;

            match bytecode {
                BytecodeType::Load => {
                    if let Some(real_arg) = real_arg {
                        stack.push(Some(real_arg));
                    } else {
                        stack.push(None);
                    }
                }
                /* BytecodeType::Push => stack.push(None),
                BytecodeType::Pop => {
                    stack.pop();
                } */
                BytecodeType::Binary(binary) => match binary {
                    Binary::Op => {
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
                        let mut end = stack.pop().unwrap().unwrap();
                        let mut start = stack.pop().unwrap().unwrap();
                        if start == "None" {
                            start.clear();
                        }
                        if end == "None" {
                            end.clear();
                        }
                        let target = stack.pop().unwrap().unwrap();
                        stack.push(Some(format!("{}[{}:{}]", target, start, end)));
                    }
                },
                BytecodeType::Call => {
                    let mut args = vec![];
                    let mut arg_str = String::new();
                    let arg_count = arg.unwrap();

                    // call 对应的arg是参数的个数
                    for _ in 0..arg_count {
                        args.push(stack.pop().unwrap().unwrap());
                    }
                    args.reverse(); // 逆序
                    arg_str = args.join(", ");
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
                BytecodeType::Extend => {
                    // Only LIST_EXTEND
                    let etn = stack.pop().unwrap().unwrap();
                    stack.pop();
                    stack.push(Some(format!("[{}]", &etn[1..etn.len() - 1])))
                }
                BytecodeType::Store => {
                    let value = stack.pop().unwrap().unwrap();
                    let name = real_arg.unwrap();
                    stack.push(Some(format!("{} = {}", name, value)));
                }
                BytecodeType::Return => {}
                _ => (),
            }
            idx += 1;
        }
    }
}

pub trait ToPyObj {
    fn to_pyobj(&self) -> Vec<PyObj>;
}

#[allow(unused)]
impl ToPyObj for &str {
    fn to_pyobj(&self) -> Vec<PyObj> {
        let mut pyobj = self
            .trim()
            .split('\n')
            .collect::<Vec<&str>>()
            .split(|line| line.trim().is_empty())
            .skip(1)
            .map(|lines| PyObj::from(lines.join("\n").as_str()))
            .collect::<Vec<PyObj>>();
        pyobj.sort_by(|a, b| a.line.cmp(&b.line));
        pyobj
    }
}

pub trait ToPythonScript {
    fn to_pyscript(&self) -> Vec<PyScript>;
}

#[allow(unused)]
impl ToPythonScript for Vec<PyObj> {
    fn to_pyscript(&self) -> Vec<PyScript> {
        let mut pyscripts: Vec<PyScript> = Vec::new();
        for pyobj in self.iter() {
            let script = pyobj.to_python();
            pyscripts.push(PyScript {
                line: pyobj.line,
                script: Some(script),
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

#[allow(unused)]
pub fn display_pycode(pyscripts: &[PyScript]) -> Result<()> {
    // 判断是否重定向了标准输出流
    if atty::is(Stream::Stdout) {
        display_pycode_with_line(pyscripts)?;
    } else {
        display_pycode_without_line(pyscripts)?;
    }
    Ok(())
}

#[allow(unused)]
pub fn read_file(file_name: &PathBuf) -> Result<String> {
    if !Path::new(file_name).exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("file {:?} not found", file_name),
        ));
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
#[allow(unused)]
pub fn write_file(file_name: &PathBuf, pyscripts: &[PyScript]) -> Result<()> {
    if File::open(file_name).is_ok() {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            format!("file {:?} already exists", file_name),
        ));
    }
    let mut file = File::create(file_name)?;
    for PyScript { line, script } in pyscripts.iter() {
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
        let path = PathBuf::from("./code.txt");
        let content = read_file(&path).unwrap();
        let pyobjs = content.as_str().to_pyobj();
        dbg!(pyobjs);
        assert!(true);
    }

    #[test]
    fn test_binary() {
        let content = r#"
        0           0 RESUME                   0

        1           2 BUILD_LIST               0
                    4 LOAD_CONST               0 ((1, 2, 4, 5, 3, 7))
                    6 LIST_EXTEND              1
                    8 STORE_NAME               0 (a)
      
        2          10 LOAD_NAME                0 (a)
                   12 LOAD_CONST               1 (2)
                   14 LOAD_CONST               2 (4)
                   16 BINARY_SLICE
                   18 POP_TOP
      
        3          20 LOAD_NAME                0 (a)
                   22 LOAD_CONST               1 (2)
                   24 LOAD_CONST               3 (None)
                   26 BINARY_SLICE
                   28 POP_TOP
      
        4          30 LOAD_NAME                0 (a)
                   32 LOAD_CONST               3 (None)
                   34 LOAD_CONST               2 (4)
                   36 BINARY_SLICE
                   38 POP_TOP
      
        5          40 LOAD_NAME                0 (a)
                   42 LOAD_CONST               3 (None)
                   44 LOAD_CONST               3 (None)
                   46 BINARY_SLICE
                   48 POP_TOP
      
        6          50 LOAD_NAME                0 (a)
                   52 LOAD_CONST               3 (None)
                   54 LOAD_CONST               3 (None)
                   56 LOAD_CONST               1 (2)
                   58 BUILD_SLICE              3
                   60 BINARY_SUBSCR
                   64 POP_TOP
      
        7          66 LOAD_NAME                0 (a)
                   68 LOAD_CONST               3 (None)
                   70 LOAD_CONST               3 (None)
                   72 LOAD_CONST               4 (-1)
                   74 BUILD_SLICE              3
                   76 BINARY_SUBSCR
                   80 POP_TOP
      
        8          82 LOAD_NAME                0 (a)
                   84 LOAD_CONST               5 (1)
                   86 LOAD_CONST               6 (5)
                   88 LOAD_CONST               1 (2)
                   90 BUILD_SLICE              3
                   92 BINARY_SUBSCR
                   96 POP_TOP
      
        9          98 LOAD_NAME                0 (a)
                  100 LOAD_CONST               6 (5)
                  102 BINARY_SUBSCR
                  106 POP_TOP
                  108 RETURN_CONST             3 (None)
                    "#;
        let pyobjs = content.to_pyobj();
        dbg!(pyobjs);
        assert!(false);
    }
}
