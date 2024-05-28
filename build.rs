//! This build script is used to tell cargo to recompile the crate
//! if any of the files in the `src` directory or its subdirectories are changed.

use std::fs;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const MAX_DEPTH: u8 = 3;

fn main() -> Result<()> {
    recompile_where("src", 0)
}

/// Recursively recompile all files in the directory and its subdirectories
///
/// But only recompile the files in the subdirectories when the depth is less than `MAX_DEPTH`
///
/// Create much more depth file structure is not recommended
fn recompile_where(directory: &str, this_depth: u8) -> Result<()> {
    for entry in fs::read_dir(Path::new(directory))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            println!("cargo:rerun-if-changed={}", path.display());
            if this_depth < MAX_DEPTH {
                recompile_where(path.to_str().unwrap(), this_depth + 1)?;
            }
        }
    }
    Ok(())
}
