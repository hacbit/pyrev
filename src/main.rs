// python bytecode reverse engineering by @hacbit
use clap::{arg, command, value_parser, ArgAction, Command};
use pyrev_core::prelude::*;
use std::io::Read;
use std::path::PathBuf;

mod app;

use app::App;

fn main() -> Result<()> {
    let matches = command!()
        .arg(arg!([name] "Optional name"))
        .arg(
            arg!(
                -f --file <FILE> "specify bytecode files"
            )
            .action(ArgAction::Append)
            // If you don't specify the input file, it will read from stdin
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -o --output <FILE> "set name of output file which contains the decompiled result"
            )
            .action(ArgAction::Append)
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .subcommand(
            Command::new("test")
                .about("test by your given python code")
                .arg(
                    arg!(
                        -c --code "specify the python code to test"
                    )
                    .action(ArgAction::Set)
                    .required(true)
                    .value_parser(value_parser!(String)),
                )
                .arg(
                    arg!(
                        -m --multiple "test multiple times"
                    )
                    .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("pyc").about("decompile pyc files").arg(
                arg!(
                    -d --directory "specify the directory to search for pyc files"
                )
                .action(ArgAction::Set)
                .value_parser(value_parser!(PathBuf)),
            ),
        )
        .get_matches();

    let ifiles = matches
        .get_many::<PathBuf>("file")
        .unwrap_or_default()
        .cloned()
        .collect::<Vec<_>>();
    let ofiles = matches
        .get_many::<PathBuf>("output")
        .unwrap_or_default()
        .cloned()
        .collect::<Vec<_>>();

    if ifiles.is_empty() {
        if atty::is(atty::Stream::Stdin) {
            warn!("No input files specified");
            return Ok(());
        } else {
            // read from stdin
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            App::new().run_once(buf).with_files(ofiles).output();
        }
    } else {
        //dbg!(&ifiles);
        //dbg!(&ofiles);
        App::new()
            .insert_resources(ifiles)
            .with_files(ofiles)
            .run()
            .output();
    }

    Ok(())
}
