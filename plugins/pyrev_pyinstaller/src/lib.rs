//! # pyrev_pyinstaller
//!
//! `pyrev_pyinstaller` is a library for extracting and analyzing PyInstaller archives.

mod pyinst_archive;

pub mod prelude {
    use std::path::PathBuf;
    use std::process::exit;

    use crate::pyinst_archive::extract_pyinstaller_archive;
    use pyrev_internal::prelude::*;

    pub struct PyInstallerPlugin;

    impl Plugin for PyInstallerPlugin {
        fn subcommand(&self) -> Option<Command> {
            Some(
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
            )
        }

        fn run(&self, args: &ArgMatches) {
            let archive_path = args.get_one::<PathBuf>("extract").unwrap_or_else(|| {
                error!("Please specify a PyInstaller archive file to extract");
                exit(-1);
            });

            if let Err(err) = extract_pyinstaller_archive(archive_path) {
                error!("{}", err);
            }
        }
    }
}
