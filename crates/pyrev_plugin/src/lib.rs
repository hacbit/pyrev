//! # pyrev_plugin
//!
//! This crate provides a way to define plugins for a CLI application.
//!
//! ## Example
//!
//! ```ignore
//! use clap::Command;
//! use pyrev_plugin::*;
//!
//! struct MyPlugin;
//!
//! impl Plugin for MyPlugin {
//!     fn build(&self, cmd: &mut Command) {
//!         cmd.arg(arg!([name] "Optional name"));
//!     }
//! }
//!
//! fn main() {
//!     let mut cli = Cli::new(Command::new("myapp"));
//!     cli.add_plugins(MyPlugin).run().unwrap();
//! }
//! ```

#![allow(non_upper_case_globals)]

use clap::{ArgMatches, Command};
use pyrev_plugin_macro::impl_plugin_all_tuples;

pub trait Plugin {
    fn build(&self, cmd: Command) -> Command;

    fn run(&self, _matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

pub trait PluginHolder {
    fn add_plugin(&mut self, plugin: impl Plugin + 'static);
}

pub trait Plugins<Marker> {
    type Cli;

    fn add_to_cli(self, cli: &mut Self::Cli);
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

pub struct PluginsTupleMarker<C: PluginHolder>(std::marker::PhantomData<C>);

impl<P: 'static + Plugin, C: PluginHolder> Plugins<PluginsTupleMarker<C>> for P {
    type Cli = C;

    fn add_to_cli(self, cli: &mut Self::Cli) {
        cli.add_plugin(self);
    }
}

macro_rules! impl_plugins_tuples {
    (($($name:ident),*$(,)?)) => {
        #[allow(non_snake_case)]
        impl<C: PluginHolder, $($name: 'static + Plugin),*> Plugins<PluginsTupleMarker<C>> for ($($name,)*) {
            type Cli = C;

            fn add_to_cli(self, _cli: &mut Self::Cli) {
                let ($($name,)*) = self;
                $(
                    _cli.add_plugin($name);
                )*
            }
        }
    };
}

impl_plugin_all_tuples!(impl_plugins_tuples, 0, 7);
