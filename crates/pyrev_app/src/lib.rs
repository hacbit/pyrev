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
                cmd,
            }
        }

        /// Add some plugins to the CLI
        ///
        /// # Example
        /// ```ignore
        /// Cli::new().add_plugins((
        ///     MyPlugin1, MyPlugin2
        /// )).run().unwrap();
        /// ```
        #[inline]
        pub fn add_plugins(
            &mut self,
            plugins: impl Plugins<PluginsTupleMarker<Self>, Cli = Self>,
        ) -> &mut Self {
            plugins.add_to_cli(self);
            self
        }

        /// Load the plugins and build the command
        pub fn build(&mut self) -> &mut Self {
            let mut cmd = self.cmd.clone();
            for plugin in self.plugins.iter() {
                cmd = plugin.build(cmd);
            }
            self.cmd = cmd;
            self
        }

        pub fn run(&mut self) -> Result<()> {
            let args = self.cmd.clone().get_matches();
            for plugin in self.plugins.iter() {
                plugin.run(&args)?;
            }
            Ok(())
        }
    }

    struct DefaultPlugin;

    impl Plugin for DefaultPlugin {
        fn build(&self, cmd: Command) -> Command {
            cmd.args([
                arg!([name] "Optional name"),
                arg!(
                    -f --file <FILE> "specify bytecode files"
                )
                .action(ArgAction::Set)
                // If you don't specify the input file, it will read from stdin
                .required(false)
                .value_parser(value_parser!(PathBuf)),
                arg!(
                    -o --output <FILE> "set name of output file which contains the decompiled result"
                )
                .action(ArgAction::Set)
                .required(false)
                .value_parser(value_parser!(PathBuf)),
            ])
        }

        fn run(&self, args: &ArgMatches) -> Result<()> {
            let mut app = App::new();

            if let Some(file) = args.get_one::<PathBuf>("file") {
                app.with_file(file);
            }
            if let Some(file) = args.get_one::<PathBuf>("output") {
                app.with_output(file);
            }

            app.run();

            Ok(())
        }
    }
}
