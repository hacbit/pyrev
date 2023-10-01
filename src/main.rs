// python bytecode reverse engineering by @hacbit
use std::{env, process::exit};

use repybytecode::*;

fn main() {
    let file_name = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("[x] Problem parsing arguments: no file name");
        exit(0);
    });
    setup(&file_name).unwrap_or_else(|err| {
        eprintln!("[x] Application error: {err}");
        exit(0);
    });
}
