// python bytecode reverse engineering by @hacbit
use clap::{arg, command, value_parser, ArgAction, Command};
use repybytecode::*;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
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
                -o --output <FILE> "specify an output file"
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

    let ifile = matches.get_one::<String>("file").unwrap();
    let ofile = matches.get_one::<String>("output");
    println!("ifile: {:?}", ifile);
    println!("ofile: {:?}", ofile);
    App::new(ifile).add(ofile).run().unwrap();
    Ok(())
}
