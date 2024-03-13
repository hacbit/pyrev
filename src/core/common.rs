use atty::Stream;
use colored::Colorize;
use lazy_format::lazy_format;
use std::io::BufRead;
use std::io::Write;
use std::path::Path;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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
    T: Iterator<Item = (usize, S)> + Clone,
    S: AsRef<str> + std::fmt::Display,
{
    /// 将迭代器中的内容写入控制台
    /// 带有行号，输出内容会被着色
    fn write_console(&mut self) -> Result<()> {
        // 判断是否重定向
        // 如果被重定向(else分支), 则不着色(因为重定向到文件不需要行号和颜色信息)
        if atty::is(Stream::Stdout) {
            let max_wide = self
                .clone()
                .max_by_key(|(i, _)| *i)
                .unwrap()
                .0
                .to_string()
                .len();
            for (i, s) in self {
                print!(
                    "{}{} ",
                    lazy_format!("{:>max$}", i.to_string().green(), max = max_wide + 2),
                    "|".bright_blue(),
                );
                println!("{}", s);
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

/// 一个简单的有序字典
#[derive(Debug, Clone)]
pub struct OrderMap<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
}

/// 实现了HashMap的几个基本方法
#[allow(unused)]
impl<K, V> OrderMap<K, V>
where
    K: PartialEq + Eq + Clone + Ord,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn insert(&mut self, mark: K, code_object: V) {
        self.keys.push(mark);
        self.values.push(code_object);
    }

    pub fn extend(&mut self, map: Self) {
        for (k, v) in map.iter() {
            if !self.contains_key(k) {
                self.insert(k.clone(), v.clone());
            }
        }
    }

    pub fn get<Q>(&self, mark: &Q) -> Option<&V>
    where
        K: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.values
            .get(self.keys.iter().position(|m| m.borrow() == mark)?)
    }

    pub fn get_mut<Q>(&mut self, mark: &Q) -> Option<&mut V>
    where
        K: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.values
            .get_mut(self.keys.iter().position(|m| m.borrow() == mark)?)
    }

    pub fn contains_key<Q>(&self, mark: &Q) -> bool
    where
        K: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.keys.iter().any(|m| m.borrow() == mark)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.keys.iter().zip(self.values.iter())
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.keys.iter()
    }
}

#[derive(Debug, Clone)]
pub struct TraceBack {
    pub locals: OrderMap<usize, (String, bool)>,
}

#[allow(unused)]
impl TraceBack {
    pub fn new() -> Self {
        Self {
            locals: OrderMap::new(),
        }
    }

    pub fn insert_local(&mut self, arg: usize, argval: String, is_store: bool) {
        self.locals.insert(arg, (argval, is_store));
    }

    pub fn extend(&mut self, tb: Self) {
        self.locals.extend(tb.locals);
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    const TEST_DIR: &str = "test/";

    #[test]
    fn test_read() {
        let file = PathBuf::from(TEST_DIR).join("import.py");
        let target = r#"import sys
from pwn import *
import numpy as np
import matplotlib.pyplot as plt
from os import system as sys, popen"#;
        assert_eq!(file.read().unwrap(), target);
    }

    #[test]
    fn test_write() {
        let file = PathBuf::from(TEST_DIR).join("for.py");
        let a = file.read().unwrap();
        dbg!(a
            .lines()
            .enumerate()
            .map(|(i, s)| (i + 1, s))
            .write_console()
            .unwrap());

        //assert!(false);
    }
}
