use pyrev_log::*;
use pyrev_query::Map;
use std::{fs, path::PathBuf, process::exit};

/// The main struct of the application.
#[derive(Default)]
pub struct App {
    /// input file path
    file_path: Option<PathBuf>,
    /// the content of input file
    content: String,
    /// the out file name
    out_path: Option<PathBuf>,
    /// the output of the decompiled code
    _output: Option<String>,
    /// the expression map
    _map: Map,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the file path and read file
    pub fn with_file(&mut self, path: impl Into<PathBuf>) -> &mut Self {
        let path: PathBuf = path.into();
        if path.exists() {
            self.file_path = Some(path.clone());
            if let Ok(data) = fs::read_to_string(&path) {
                self.content = data;
            } else {
                error!("Read `{}` error", path.display());
                exit(-1);
            }
            self
        } else {
            error!("Path `{}` not found", path.display());
            exit(-1);
        }
    }

    /// Specify output file path
    pub fn with_output(&mut self, path: impl Into<PathBuf>) -> &mut Self {
        let path: PathBuf = path.into();
        if path.exists() {
            warn!("Path `{}` already exists", path.display());
            exit(-1);
        } else {
            self.out_path = Some(path);
            self
        }
    }

    pub fn run(&mut self) {}
}
