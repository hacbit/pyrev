use atty::Stream;
pub use colored::Colorize;
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

/// 一个简单的有序字典
#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone, Default)]
pub struct Local {
    pub name: String,
    pub is_store: bool,
    pub is_arg: bool,
}

#[derive(Debug, Clone, Default)]
pub struct TraceBack {
    /// is_store is true if the arg is stored in the function argument
    /// arg, Local
    /// arg is the index of the argument
    locals: OrderMap<usize, Local>,
    /// start is the start offset of the jump
    /// jump_target is the target offset of the jump
    /// start, jump_target
    jumps: OrderMap<usize, usize>,
    /// is_async is true if the function is async
    is_async: bool,
}

#[allow(unused)]
impl TraceBack {
    pub fn insert_local(&mut self, arg: usize, local: Local) {
        self.locals.insert(arg, local);
    }

    pub fn get_local(&self, arg: &usize) -> Option<&Local> {
        self.locals.get(arg)
    }

    pub fn get_mut_local(&mut self, arg: &usize) -> Option<&mut Local> {
        self.locals.get_mut(arg)
    }

    pub fn get_locals(&self) -> &OrderMap<usize, Local> {
        &self.locals
    }

    pub fn get_mut_locals(&mut self) -> &mut OrderMap<usize, Local> {
        &mut self.locals
    }

    pub fn insert_jump(&mut self, start: usize, jump_target: usize) {
        self.jumps.insert(start, jump_target);
    }

    pub fn get_jump(&self, start: &usize) -> Option<&usize> {
        self.jumps.get(start)
    }

    pub fn get_mut_jump(&mut self, start: &usize) -> Option<&mut usize> {
        self.jumps.get_mut(start)
    }

    pub fn get_jumps(&self) -> &OrderMap<usize, usize> {
        &self.jumps
    }

    pub fn get_mut_jumps(&mut self) -> &mut OrderMap<usize, usize> {
        &mut self.jumps
    }

    pub fn extend(&mut self, tb: Self) {
        self.locals.extend(tb.locals);
        self.jumps.extend(tb.jumps);
    }

    pub fn asyncable(&self) -> bool {
        self.is_async
    }

    pub fn mark_async(&mut self) {
        self.is_async = true;
    }
}

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
