use atty::Stream;
use colored::Colorize;
use lazy_format::lazy_format;
use std::io::Write;
use std::{io::BufRead, path::PathBuf};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait IStream {
    fn read(self) -> Result<String>;
}

pub trait OStream {
    fn write_console(self) -> Result<()>;
    fn write_file(self, file: PathBuf) -> Result<()>;
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
    fn write_console(self) -> Result<()> {
        if atty::is(Stream::Stdout) {
            let max_line = self.clone().max_by_key(|(i, _)| *i).ok_or("No max line")?.0;
            for (i, s) in self.into_iter() {
                print!(
                    "{}{} ",
                    lazy_format!("{:>max_line$}", i.to_string().green(), max_line = max_line),
                    "|".bright_blue(),
                );
                println!("{}", s);
            }
        } else {
            for (_, s) in self.into_iter() {
                println!("{}", s);
            }
        }
        Ok(())
    }

    fn write_file(self, file: PathBuf) -> Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(file)?;
        for (_, s) in self.into_iter() {
            writeln!(file, "{}", s)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
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
        let _ = file
            .read()
            .unwrap()
            .lines()
            .enumerate()
            .write_console()
            .unwrap();
        //assert!(false);
    }
}
