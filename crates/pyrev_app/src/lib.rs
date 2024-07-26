pub mod app;

pub mod prelude {
    pub use crate::app::App;
    pub use clap::{arg, command, value_parser, Arg, ArgAction, ArgMatches, Command};
    use pyrev_log::*;
    use pyrev_plugin::*;
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
                cmd = if let Some(sub) = plugin.subcommand() {
                    self.names.push(sub.get_name().to_string());
                    cmd.subcommand(sub)
                } else {
                    self.names.push("".to_string());
                    cmd
                }
                .args(plugin.args());
            }
            self.cmd = cmd;
            self
        }

        pub fn run(&mut self) {
            let args = match self.cmd.clone().try_get_matches() {
                Ok(args) => args,
                Err(e) => {
                    error!("Failed to parse arguments: {}", e);
                    return;
                }
            };
            let mut found = false;

            debug_assert_eq!(self.plugins.len(), self.names.len());
            // Check if any subcommand is found
            // skip the default plugin
            for (plugin, name) in self.plugins.iter().zip(self.names.iter()).skip(1) {
                if let Some(arg) = args.subcommand_matches(name) {
                    plugin.run(arg);
                    found = true;
                    break;
                }
            }

            // If no subcommand is found, run the default plugin
            if !found {
                self.plugins[0].run(&args);
            }
        }
    }

    struct DefaultPlugin;

    impl Plugin for DefaultPlugin {
        fn args(&self) -> Vec<Arg> {
            vec![
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
            ]
        }

        fn run(&self, args: &ArgMatches) {
            let mut app = App::new();

            if let Some(file) = args.get_one::<PathBuf>("file") {
                app.with_file(file);
            }
            if let Some(file) = args.get_one::<PathBuf>("output") {
                app.with_output(file);
            }

            app.run();
        }
    }
}
