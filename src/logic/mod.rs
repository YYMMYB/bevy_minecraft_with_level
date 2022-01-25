use crate::label;
use bevy::{
    app::PluginGroupBuilder,
    prelude::{Plugin, PluginGroup, SystemLabel, SystemSet},
};

pub mod block_map;
pub mod config;
pub mod map_generator;
pub mod test;

pub struct LogicPlugins;

impl PluginGroup for LogicPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(config::Plug)
            .add(block_map::Plug)
            .add(map_generator::Plug)
            .add(test::Plug);
    }
}
