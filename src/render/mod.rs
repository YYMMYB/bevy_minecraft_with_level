use bevy::prelude::PluginGroup;
use self::debug::DebugPlugin;


mod debug;

pub struct RenderPlugins;

impl PluginGroup for RenderPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(DebugPlugin);
    }
}

