use atty::Stream;
use colored::Colorize;
use lazy_format::lazy_format;
use std::io::BufRead;
use std::io::Write;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait IStream {
    fn read(self) -> Result<String>;
}

pub trait OStream {
    fn write_console(&mut self) -> Result<()>;
    fn write_file<P: AsRef<Path>>(&mut self, file: P) -> Result<()>;
}

impl<T> IStream for T
where
    T: AsRef<std::path::Path>,
{
    fn read(self) -> Result<String> {
        let file = std::fs::OpenOptions::new().read(true).open(self.as_ref())?;
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
    fn write_console(&mut self) -> Result<()> {
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

    fn write_file<P: AsRef<Path>>(&mut self, file: P) -> Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(file)?;
        for (_, s) in self {
            writeln!(file, "{}", s)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct OrderMap<K, V> {
    marks: Vec<K>,
    code_objects: Vec<V>,
}

#[allow(unused)]
impl<K, V> OrderMap<K, V>
where
    K: PartialEq + Eq + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            marks: Vec::new(),
            code_objects: Vec::new(),
        }
    }

    pub fn insert(&mut self, mark: K, code_object: V) {
        self.marks.push(mark);
        self.code_objects.push(code_object);
    }

    pub fn get<Q: ?Sized>(&self, mark: &Q) -> Option<&V>
    where
        K: std::borrow::Borrow<Q>,
        Q: Ord,
    {
        self.code_objects
            .get(self.marks.iter().position(|m| m.borrow() == mark)?)
    }

    pub fn get_mut<Q: ?Sized>(&mut self, mark: &Q) -> Option<&mut V>
    where
        K: std::borrow::Borrow<Q>,
        Q: Ord,
    {
        self.code_objects
            .get_mut(self.marks.iter().position(|m| m.borrow() == mark)?)
    }

    pub fn contains_key<Q: ?Sized>(&self, mark: &Q) -> bool
    where
        K: std::borrow::Borrow<Q>,
        Q: Ord,
    {
        self.marks.iter().any(|m| m.borrow() == mark)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.marks.iter().zip(self.code_objects.iter())
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.marks.iter()
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
from os import system, popen"#;
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
