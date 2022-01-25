use bevy::prelude::{PluginGroup, SystemSet, Plugin, info};

use crate::label;

pub mod test;

pub struct RenderPlugins;

impl PluginGroup for RenderPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(MainPlug);
    }
}

pub struct MainPlug;
impl Plugin for MainPlug {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system_set(
            SystemSet::new()
                .label(label::System::Render)
                .after(label::System::Logic)
                .with_system(test::print_system)
        );
    }
}