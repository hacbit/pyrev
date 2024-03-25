use crate::core::common::*;
use crate::core::decompile::*;
use crate::core::parse_opcode::*;
use colored::Colorize;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct App {
    // files order
    files: Vec<PathBuf>,
    // the resource of the bytecode
    resources: HashMap<PathBuf, CodeObjectMap>,
    // the out file name
    output_files: Vec<PathBuf>,
    // the output of the decompiled code
    output: Vec<Result<DecompiledCode>>,
}

#[allow(unused)]
impl App {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            resources: HashMap::new(),
            output_files: Vec::new(),
            output: Vec::new(),
        }
    }

    /// 插入一个资源(要解析的字节码文件路径)
    pub fn insert_resource<P: Into<PathBuf>>(&mut self, path: P) -> &mut Self {
        let path = path.into();
        if path.is_file() {
            let resource = &path.read().unwrap();
            let code_object_map = resource.parse_opcode().unwrap();
            if self
                .resources
                .insert(path.clone(), code_object_map)
                .is_none()
            {
                self.files.push(path);
            } else {
                println!(
                    "{}",
                    format!("Warning: {} is already in the resources", path.display())
                        .bright_yellow()
                );
            }
        } else {
            println!(
                "{}",
                format!("Warning: {} is not exists or is not a file", path.display()).bright_yellow()
            );
        }
        self
    }

    /// 插入多个
    pub fn insert_resources<P: Into<PathBuf>>(&mut self, paths: Vec<P>) -> &mut Self {
        for path in paths {
            self.insert_resource(path);
        }
        self
    }

    /// 指定输出地址
    pub fn with_file<P: Into<PathBuf>>(&mut self, path: P) -> &mut Self {
        let path = path.into();
        if !path.exists() && !self.output_files.contains(&path) {
            self.output_files.push(path);
        } else {
            println!(
                "{}",
                format!("Warning: {} is already in the output files", path.display())
                    .bright_yellow()
            );
        }
        self
    }

    /// 指定多个输出地址
    pub fn with_files<P: Into<PathBuf>>(&mut self, paths: Vec<P>) -> &mut Self {
        for path in paths {
            self.with_file(path);
        }
        self
    }

    pub fn run(&mut self) -> &mut Self {
        for path in self.files.iter() {
            let code_object_map = self
                .resources
                .get(path)
                .unwrap_or_else(|| panic!("[App run] resource {} not found", path.display()));
            let decompiled_result = code_object_map.decompile();
            self.output.push(decompiled_result);
        }
        self
    }

    pub fn run_once(&mut self, _stdin: String) -> &mut Self {
        let parsed_map = _stdin.parse_opcode().unwrap();
        let decompiled_result = parsed_map.decompile();
        self.files.push(PathBuf::from("[Temp file]"));
        self.output.push(decompiled_result);
        self
    }

    /// 会按照输入文件路径和输出文件路径的插入顺序导出
    /// 如果没有匹配到输出文件路径, 则会输出到控制台
    pub fn output(&mut self) {
        let mut paths = self.files.iter();
        self.output
            .iter_mut()
            .enumerate()
            .for_each(|(i, decompiled_result)| {
                println!(
                    "{}",
                    format!(
                        "Try to decompile {}",
                        paths.next().expect("[App output] iter end").display()
                    )
                    .bright_green()
                );
                if let Some(file) = self.output_files.get(i) {
                    match decompiled_result {
                        Ok(decompiled_code) => decompiled_code.iter().write_file(file).unwrap(),
                        Err(err) => eprintln!(
                            "{}",
                            format!(
                                "The file {} decompiled failed: {}",
                                self.files[i].display(),
                                err
                            )
                            .bright_red()
                        ),
                    }
                } else {
                    match decompiled_result {
                        Ok(decompiled_code) => decompiled_code.iter().write_console().unwrap(),
                        Err(err) => eprintln!(
                            "{}",
                            format!(
                                "The file {} decompiled failed: {}",
                                self.files[i].display(),
                                err
                            )
                            .bright_red()
                        ),
                    }
                }
            });
    }
}
