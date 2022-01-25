use crate::label;
use bevy::{
    app::PluginGroupBuilder,
    prelude::{Plugin, PluginGroup, SystemLabel, SystemSet, info, Commands}, ecs::component::Components,
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
            .add(MainPlug);
    }
}

pub struct MainPlug;
impl Plugin for MainPlug {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system_set(
            SystemSet::new()
                .label(label::System::Logic)
                .with_system(test::build_block_map)
        );
    }
}