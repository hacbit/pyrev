//! A simple logging library packing with colorized output.

use atty::Stream;
pub use colored::Colorize;
use lazy_format::lazy_format;
use std::{
    io::{BufRead, Write},
    path::Path,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        println!("[INFO] {}", format!($($arg)*).bright_green())
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        eprintln!("[WARN] {}", format!($($arg)*).bright_yellow())
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        eprintln!("[ERROR] {}", format!($($arg)*).bright_red())
    };
}

pub trait IStream {
    fn read(&self) -> Result<String>;
}

pub trait OStream {
    fn write_console(&mut self) -> Result<()>;
    fn write_file<P: AsRef<Path>>(&mut self, file: P) -> Result<()>;
}

impl<T> IStream for T
where
    T: AsRef<std::path::Path>,
{
    /// 读取文件内容
    fn read(&self) -> Result<String> {
        let file = std::fs::OpenOptions::new().read(true).open(self)?;
        let reader = std::io::BufReader::new(file);
        let content = reader
            .lines()
            .map(|line| line.unwrap())
            .collect::<Vec<String>>()
            .join("\n");
        Ok(content)
    }
}

impl<T, S> OStream for T
where
    T: Iterator<Item = (usize, S)> + Clone + std::fmt::Debug,
    S: AsRef<str> + std::fmt::Display,
{
    /// 将迭代器中的内容写入控制台
    /// 带有行号，输出内容会被着色
    fn write_console(&mut self) -> Result<()> {
        // 判断是否重定向
        // 如果被重定向(else分支), 则不着色(因为重定向到文件不需要行号和颜色信息)
        let mut line: usize = 1;
        if atty::is(Stream::Stdout) {
            let max_wide = self
                .clone()
                .max_by_key(|(i, _)| *i)
                .ok_or(format!("[WriteConsole] Can't get max wide: {:?}", self))?
                .0
                .to_string()
                .len();
            for (_, s) in self {
                print!(
                    "{}{} ",
                    lazy_format!(
                        "{:>max$}",
                        line.to_string().bright_green(),
                        max = max_wide + 2
                    ),
                    "|".bright_blue(),
                );
                println!("{}", s);
                line += 1;
            }
        } else {
            for (_, s) in self {
                println!("{}", s);
            }
        }
        Ok(())
    }

    /// 将迭代器中的内容写入文件
    fn write_file<P: AsRef<Path>>(&mut self, file: P) -> Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file)?;
        for (_, s) in self {
            writeln!(file, "{}", s)?;
        }
        Ok(())
    }
}
