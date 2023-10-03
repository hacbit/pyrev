// python bytecode reverse engineering by @hacbit
use std::{env, process::exit};
mod color;
use color::color::*;
use color::color_utils::*;
use repybytecode::setup;

fn main() {
    let warn_color = ColorMode::from(Color::Red);
    let file_name = env::args().nth(1).unwrap_or_else(|| {
        eprintln!(
            "[{}] Problem parsing arguments: {}",
            "x".to_color_string(&warn_color),
            "no file name".to_color_string(&warn_color)
        );
        exit(0);
    });
    setup(&file_name).unwrap_or_else(|err| {
        eprintln!(
            "[{}] Application error: {err}",
            "x".to_color_string(&warn_color),
            err = err.to_string().to_color_string(&warn_color)
        );
        exit(0);
    });
}
