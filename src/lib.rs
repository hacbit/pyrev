mod bytecode;
use bytecode::utils::*;
use std::io::Error;
use std::path::PathBuf;

type Result<T> = std::result::Result<T, Error>;

pub struct App {
    ifile: PathBuf,         // input file
    ofile: Option<PathBuf>, // output file
}

impl App {
    pub fn new(ifile: &PathBuf) -> Self {
        App {
            ifile: ifile.to_owned(),
            ofile: None,
        }
    }

    pub fn add(&mut self, ofile: Option<&PathBuf>) -> &mut Self {
        if let Some(ofile) = ofile {
            self.ofile = Some(ofile.clone());
        }
        self
    }

    pub fn run(&self) -> Result<()> {
        let pyscript = read_file(&self.ifile)?.as_str().to_pyobj().to_pyscript();
        if let Some(ofile) = &self.ofile {
            write_file(ofile, &pyscript)
        } else {
            display_pycode(&pyscript)
        }
    }
}
