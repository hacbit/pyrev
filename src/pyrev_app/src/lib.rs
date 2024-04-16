/// defined Plugin trait
/// you can implement Plugin trait for a struct, and added to app plugin
pub mod plugin;

pub mod app;

pub mod prelude {
    pub use crate::app::App;
    pub use crate::plugin::{Plugin, *};
    pub use clap::{arg, command, value_parser, Arg, ArgAction, ArgMatches, Command};
    pub use pyrev_core::prelude::*;
    use std::io::Read;
    use std::path::PathBuf;

    #[derive(Debug)]
    pub struct Cli {
        plugins: Vec<Box<dyn Plugin>>,
    }

    impl Default for Cli {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Cli {
        pub fn new() -> Self {
            Self {
                plugins: vec![Box::new(DefaultPlugin)],
            }
        }

        pub fn add_plugin<P: Plugin + 'static>(&mut self, plugin: P) -> &mut Self {
            self.plugins.push(Box::new(plugin));
            self
        }

        pub fn add_plugins<P: Plugins<PluginsTupleMarker>>(&mut self, plugins: P) -> &mut Self {
            plugins.add_to_cli(self);
            self
        }

        pub fn run(&mut self) -> Result<()> {
            let mut plugins = self.plugins.iter();
            let mut cmd = plugins.next().unwrap().subcommand();

            for plugin in plugins {
                cmd = cmd.subcommand(plugin.subcommand());
            }

            let matches = cmd.get_matches();

            for plugin in &self.plugins {
                if let Some(matches) = matches.subcommand_matches(plugin.subcommand().get_name()) {
                    plugin.run(&matches)?;
                }
            }
            Ok(())
        }
    }

    struct DefaultPlugin;

    impl Plugin for DefaultPlugin {
        fn subcommand(&self) -> Command {
            command!("pyrev")
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
        }

        fn run(&self, args: &ArgMatches) -> Result<()> {
            let ifiles = args
                .get_many::<PathBuf>("file")
                .unwrap_or_default()
                .cloned()
                .collect::<Vec<_>>();
            let ofiles = args
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
    }
}
