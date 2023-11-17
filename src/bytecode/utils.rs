use super::bytecode_type::BytecodeType;
use super::op::OP;

use atty::Stream;
use colored::Colorize;
use regex::Regex;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Write};
use std::path::Path;

type Result<T> = std::result::Result<T, Error>;

// #[derive(Debug)]
#[allow(unused)]
struct BytecodeBlock {
    offset: u32,
    bytecode: BytecodeType,
    arg: Option<u32>,
    real_arg: Option<String>,
    jump_offset: Option<u32>,
}

// #[derive(Debug)]
pub struct PyObj {
    line: usize,
    bytecode_lines: Vec<Option<BytecodeBlock>>,
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
            (\s+)?
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
                    (Some(off), Some(bc), a, ra) => Some(BytecodeBlock {
                        offset: off.as_str().parse::<u32>().unwrap(),
                        bytecode: BytecodeType::get(bc.as_str()),
                        arg: a.map(|a| a.as_str().parse::<u32>().unwrap()),
                        real_arg: ra.map(|ra| ra.as_str().to_string()),
                        jump_offset: None,
                    }),
                    _ => None,
                }
            })
            .collect::<Vec<Option<BytecodeBlock>>>();
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
                continue;
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
                        stack.push(None);
                    } else {
                        stack.push(Some(real_arg.unwrap()));
                    }
                }
                BytecodeType::Push => stack.push(None),
                BytecodeType::Pop => {
                    stack.pop();
                }
                BytecodeType::Op => {
                    let right = stack.pop().unwrap().unwrap();
                    let left = stack.pop().unwrap().unwrap();
                    let op = OP::from_str(real_arg.unwrap().as_str()).unwrap();
                    stack.push(Some(op.get_expr(&left, &right)));
                }
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
                BytecodeType::Store => {
                    let value = stack.pop().unwrap().unwrap();
                    let name = stack.pop().unwrap().unwrap();
                    stack.push(Some(format!("{} = {}", name, value)));
                }
                BytecodeType::Return => {}
                _ => (),
            }
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
        let mut line = 0;
        let mut script = String::new();
        for pyobj in self.iter() {
            let pycode = pyobj.to_python();
            pyscripts.push(PyScript {
                line,
                script: Some(pycode),
            })
        }
        pyscripts
    }
}

// 正常输出到终端
fn display_pycode_with_line(pyscripts: &[PyScript]) -> Result<()> {
    for PyScript { line, script } in pyscripts.iter() {
        print!("{:>17}: ", line.to_string().green(),);
        if let Some(script) = script {
            println!("{}", script);
        }
    }
    Ok(())
}

// 重定向标准输出流时，不输出行号
fn display_pycode_without_line(pyscripts: &[PyScript]) -> Result<()> {
    for PyScript { script, .. } in pyscripts.iter() {
        if let Some(script) = script {
            println!("{}", script);
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
pub fn read_file(file_name: &str) -> Result<String> {
    if !Path::new(file_name).exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("file {} not found", file_name),
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
pub fn write_file(file_name: &str, pyscripts: &[PyScript]) -> Result<()> {
    if File::open(file_name).is_ok() {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            format!("file {} already exists", file_name),
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
