// python bytecode reverse engineering by @hacbit
use clap::{arg, command, value_parser, ArgAction, Command};
use std::io::Read;
use std::path::PathBuf;

mod app;
mod core;

use app::App;
use core::common::Result;

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
                .about("run the example")
                .arg(arg!(-t --test "print test").action(ArgAction::SetTrue)),
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
        // read from stdin
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        App::new().run_once(buf).with_files(ofiles).output();
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
