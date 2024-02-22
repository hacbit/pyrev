// python bytecode reverse engineering by @hacbit
use clap::{arg, command, value_parser, ArgAction, Command};
use std::path::PathBuf;

mod bytecode;
use bytecode::utils::Decompiler;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let matches = command!()
        .arg(arg!([name] "Optional name"))
        .arg(
            arg!(
                -f --file <FILE> "specify a bytecode file"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -o --output <FILE> "set name of output file which contains the decompiled result"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .subcommand(
            Command::new("test")
                .about("run the example")
                .arg(arg!(-t --test "print test").action(ArgAction::SetTrue)),
        )
        .get_matches();

    let ifile = matches.get_one::<PathBuf>("file").unwrap();
    let ofile = matches.get_one::<PathBuf>("output");

    let decompiler = Decompiler::from(ifile).decompile();
    if let Err(err) = if let Some(ofile) = ofile {
        decompiler.to_file(ofile)
    } else {
        decompiler.to_stdout()
    } {
        println!("{}", err);
    }
    Ok(())
}
