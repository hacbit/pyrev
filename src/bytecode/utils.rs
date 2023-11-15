use super::bytecode::Bytecode;
use super::op::OP;
use super::valuetype::ValueType;
use crate::color::color::*;
use crate::color::color_utils::*;
use regex::Regex;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
#[allow(unused)]
struct BytecodeLine {
    offset: u32,
    bytecode: Bytecode,
    arg: Option<u32>,
    real_arg: Option<String>,
    jump_offset: Option<u32>,
}

#[derive(Debug)]
pub struct PyLine {
    line: u32,
    bytecode_lines: Vec<Option<BytecodeLine>>,
}

#[allow(unused)]
impl From<&str> for PyLine {
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
                match cap.name("line") {
                    Some(l) => line = l.as_str().parse::<u32>().unwrap(),
                    None => (),
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
                        bytecode: Bytecode::get(bc.as_str()),
                        arg: if a.is_none() {
                            None
                        } else {
                            a.unwrap().as_str().parse::<u32>().ok()
                        },
                        real_arg: if ra.is_none() {
                            None
                        } else {
                            Some(ra.unwrap().as_str().to_string())
                        },
                        jump_offset: None,
                    }),
                    _ => None,
                }
            })
            .collect::<Vec<Option<BytecodeLine>>>();
        PyLine {
            line,
            bytecode_lines,
        }
    }
}

#[allow(unused)]
fn parse_input(input: &str) -> Vec<PyLine> {
    input
        .trim()
        .split("\n")
        .collect::<Vec<&str>>()
        .split(|line| line.trim().is_empty())
        .map(|lines| PyLine::from(lines.join("\n").as_str()))
        .collect::<Vec<PyLine>>()
}

#[allow(unused)]
pub fn parse_file(file_name: &str) -> Vec<PyLine> {
    let mut file = File::open(file_name).unwrap_or_else(|err| {
        eprintln!(
            "[{}] Application error: {err}",
            "x".to_color_string(&ColorMode::from(FrontColor::Red)),
            err = err
                .to_string()
                .to_color_string(&ColorMode::from(FrontColor::Red)),
        );
        std::process::exit(0);
    });
    let mut bytecode_string = String::new();
    file.read_to_string(&mut bytecode_string).unwrap();
    parse_input(&bytecode_string)
}

#[allow(unused)]
impl PyLine {
    fn to_python(&self) -> String {
        let mut python_code = String::new();
        let mut stack = vec![];
        let mut now_retraction = 0;
        let mut idx = 0;
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
                Bytecode::Load => {
                    if real_arg.is_none() {
                        stack.push(None);
                    } else {
                        stack.push(real_arg.clone());
                    }
                }
                Bytecode::Push => stack.push(None),
                Bytecode::Pop => {
                    stack.pop();
                }
                Bytecode::Op => {
                    let (left, right) = (stack.pop().unwrap(), stack.pop().unwrap());
                    let op = OP::from_str(real_arg.unwrap().as_str()).unwrap();
                    stack.push(Some(
                        op.get_expr(left.unwrap().as_str(), right.unwrap().as_str()),
                    ));
                }
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_regex() {
        let py_line = PyLine::from(
            r#" 1 >>             58 LOAD_NAME                2 (print)
        60 LOAD_NAME                0 (arr)"
        "#,
        );
        dbg!(py_line);
        // assert!(false);
    }

    #[test]
    fn test_parser() {
        let py_lines = parse_input(
            r#"
        1           2 BUILD_LIST               0
                    4 LOAD_CONST               0 ((1, 2, 3, 'a', 'py'))
                    6 LIST_EXTEND              1
                    8 STORE_NAME               0 (arr)

        2          10 LOAD_NAME                0 (arr)
                   12 GET_ITER
              >>   14 FOR_ITER                10 (to 38)
                   18 STORE_NAME               1 (i)

        3          20 PUSH_NULL
                   22 LOAD_NAME                2 (print)
                   24 LOAD_NAME                1 (i)
                   26 CALL                     1
                   34 POP_TOP
                   36 JUMP_BACKWARD           12 (to 14)
        "#,
        );
        dbg!(py_lines);
        // assert!(false);
    }
}
