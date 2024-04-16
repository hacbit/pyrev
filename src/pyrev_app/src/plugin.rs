#![allow(non_upper_case_globals)]

use clap::{ArgMatches, Command};

use crate::prelude::Cli;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Plugin {
    fn subcommand(&self, cmd: Command) -> (Command, &str);
    fn run(&self, args: &ArgMatches) -> Result<()>;
}

pub trait Plugins<Marker> {
    fn add_to_cli(self, cli: &mut Cli);
}

impl std::fmt::Debug for Box<dyn Plugin> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = std::any::type_name::<Self>();
        f.debug_struct(name).finish()
    }
}

impl<T> From<T> for Box<dyn Plugin>
where
    T: Plugin + 'static,
{
    fn from(plugin: T) -> Self {
        Box::new(plugin)
    }
}

pub struct PluginsTupleMarker;

impl<P: 'static + Plugin> Plugins<PluginsTupleMarker> for P {
    fn add_to_cli(self, cli: &mut Cli) {
        cli.add_plugin(self);
    }
}

macro_rules! impl_plugins_tuples {
    (($($name:ident),*$(,)?)) => {
        #[allow(non_snake_case)]
        impl<$($name: 'static + Plugin),*> Plugins<PluginsTupleMarker> for ($($name,)*) {
            fn add_to_cli(self, _cli: &mut Cli) {
                let ($($name,)*) = self;
                $(
                    _cli.add_plugin($name);
                )*
            }
        }
    };
}

impl_plugins_tuples!(());
impl_plugins_tuples!((P0));
impl_plugins_tuples!((P0, P1));
impl_plugins_tuples!((P0, P1, P2));

#[cfg(test)]
mod test {
    use super::*;
    use clap::*;

    struct TestPlugin;

    impl Plugin for TestPlugin {
        fn subcommand(&self, cmd: Command) -> (Command, &str) {
            (cmd.subcommand(
                Command::new("test").about("this is test subcommand").arg(
                    arg!(
                        -a --arg <A> "this is an argument"
                    )
                    .action(ArgAction::Set)
                    .value_parser(value_parser!(String)),
                ),
            ), "test")
        }

        fn run(&self, args: &ArgMatches) -> Result<()> {
            let a = args
                .try_get_one::<String>("arg")?
                .ok_or("Error: argument `a` not found")?;
            println!("a: {}", a);
            Ok(())
        }
    }

    #[test]
    fn test_params() {
        let mut params = Cli::new(command!());
        params.add_plugins((TestPlugin,));

        dbg!(params);
    }
}
