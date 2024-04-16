// python bytecode reverse engineering by @hacbit
use pyrev_app::prelude::*;
use pyrev_pyc::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    Cli::new().add_plugins((PycPlugin, )).run()
}
