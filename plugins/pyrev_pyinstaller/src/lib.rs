//! # pyrev_pyinstaller
//!
//! `pyrev_pyinstaller` is a library for extracting and analyzing PyInstaller archives.

mod pyinst_archive;

pub mod prelude {
    use std::path::PathBuf;

    use crate::pyinst_archive::extract_pyinstaller_archive;
    use pyrev_app::prelude::*;

    pub struct PyInstallerPlugin;

    impl Plugin for PyInstallerPlugin {
        fn subcommand(&self, cmd: Command) -> (Command, &str) {
            (
                cmd.subcommand(
                    Command::new("pyinstaller")
                        .about("extract and analyze PyInstaller archives")
                        .arg(
                            arg!(
                                -e --extract <FILE> "specify a PyInstaller archive file to extract"
                            )
                            .action(ArgAction::Set)
                            .required(false)
                            .value_parser(value_parser!(PathBuf)),
                        ),
                ),
                "pyinstaller",
            )
        }

        fn run(&self, args: &ArgMatches) -> Result<()> {
            let archive_path = args
                .try_get_one::<PathBuf>("extract")?
                .ok_or("File not found")?;

            extract_pyinstaller_archive(archive_path)?;

            Ok(())
        }
    }
}
