pub mod app;

pub mod prelude {
    pub use crate::app::App;
    pub use clap::{arg, command, value_parser, Arg, ArgAction, ArgMatches, Command};
    pub use pyrev_core::prelude::*;
    pub use pyrev_plugin::*;
    use std::path::PathBuf;

    #[derive(Debug)]
    pub struct Cli {
        plugins: Vec<Box<dyn Plugin>>,
        names: Vec<String>,
        cmd: Command,
    }

    impl PluginHolder for Cli {
        fn add_plugin(&mut self, plugin: impl Plugin + 'static) {
            self.plugins.push(Box::new(plugin));
        }
    }

    impl Cli {
        pub fn new(cmd: Command) -> Self {
            Self {
                plugins: vec![Box::new(DefaultPlugin)],
                names: vec![],
                cmd,
            }
        }

        pub fn add_plugins(
            &mut self,
            plugins: impl Plugins<PluginsTupleMarker<Self>, Cli = Self>,
        ) -> &mut Self {
            plugins.add_to_cli(self);
            self
        }

        pub fn run(&mut self) -> Result<()> {
            todo!()
        }
    }

    struct DefaultPlugin;

    impl Plugin for DefaultPlugin {
        fn build(&self, cmd: &mut Command) {
            
        }

        // fn subcommand(&self, cmd: Command) -> (Command, &str) {
        //     (cmd.arg(arg!([name] "Optional name"))
        //     .arg(
        //         arg!(
        //             -f --file <FILE> "specify bytecode files"
        //         )
        //         .action(ArgAction::Set)
        //         // If you don't specify the input file, it will read from stdin
        //         .required(false)
        //         .value_parser(value_parser!(PathBuf)),
        //     )
        //     .arg(
        //         arg!(
        //             -o --output <FILE> "set name of output file which contains the decompiled result"
        //         )
        //         .action(ArgAction::Set)
        //         .required(false)
        //         .value_parser(value_parser!(PathBuf)),
        //     )
        //     .subcommand(
        //         Command::new("test")
        //             .about("test by your given python code")
        //             .arg(
        //                 arg!(
        //                     -c --code "specify the python code to test"
        //                 )
        //                 .action(ArgAction::Set)
        //                 .required(true)
        //                 .value_parser(value_parser!(String)),
        //             )
        //             .arg(
        //                 arg!(
        //                     -m --multiple "test multiple times"
        //                 )
        //                 .action(ArgAction::SetTrue),
        //             ),
        //     ), "default")
        // }

        // fn run(&self, args: &ArgMatches) -> Result<()> {
        //     let mut app = App::new();

        //     if let Some(file) = args.get_one::<PathBuf>("file") {
        //         app.with_file(file);
        //     }
        //     if let Some(file) = args.get_one::<PathBuf>("output") {
        //         app.with_output(file);
        //     }

        //     app.run();

        //     Ok(())
        // }
    }
}
