use bevy::{
    app::PluginGroupBuilder,
    prelude::{Plugin, PluginGroup},
};

mod block_map;
mod config;
mod map_generator;
mod test;

pub use block_map::*;
pub use config::*;
pub use map_generator::*;
pub use test::*;

pub struct LogicPlugins;

impl PluginGroup for LogicPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(ConfigPlugin)
            .add(BlockMapPlugin)
            .add(GeneratorPlugin)
            .add(TestPlugin);
    }
}
