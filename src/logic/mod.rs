use bevy::{app::PluginGroupBuilder, prelude::{PluginGroup, Plugin}};

mod block_map;
mod config;
mod map_generator;

pub use block_map::*;
pub use config::*;
pub use map_generator::*;

pub struct LogicPlugins;

impl PluginGroup for LogicPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(ConfigPlugin)
            .add(BlockMapPlugin)
            .add(GeneratorPlugin);
    }
}
