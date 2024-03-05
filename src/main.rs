// python bytecode reverse engineering by @hacbit
use clap::{arg, command, value_parser, ArgAction, Command};
use std::path::PathBuf;

mod app;
mod core;
use app::App;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let matches = command!()
        .arg(arg!([name] "Optional name"))
        .arg(
            arg!(
                -f --file <FILE> "specify bytecode files"
            )
            .action(ArgAction::Append)
            .required(true)
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
        .expect("file is required")
        .cloned()
        .collect::<Vec<_>>();
    let ofiles = matches
        .get_many::<PathBuf>("output")
        .unwrap_or_default()
        .cloned()
        .collect::<Vec<_>>();

    //dbg!(&ifiles);
    //dbg!(&ofiles);
    App::new()
        .insert_resources(ifiles)
        .with_files(ofiles)
        .run()
        .output();

    Ok(())
}
