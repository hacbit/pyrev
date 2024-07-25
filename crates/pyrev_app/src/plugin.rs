#![allow(non_upper_case_globals)]

use clap::{ArgMatches, Command};

use crate::prelude::Cli;
use pyrev_app_macro::impl_plugin_all_tuples;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Plugin {
    fn build(&self, cmd: &mut Command);
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


impl_plugin_all_tuples!(impl_plugins_tuples, 0, 7);
