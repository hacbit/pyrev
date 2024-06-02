// python bytecode reverse engineering by @hacbit
use pyrev_app::prelude::*;
use pyrev_pyc::prelude::*;
use pyrev_pyinstaller::prelude::*;

fn main() -> Result<()> {
    Cli::new(command!())
        .add_plugins((PycPlugin, PyInstallerPlugin))
        .run()
}
