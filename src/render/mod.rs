use bevy::prelude::PluginGroup;

pub mod test;

pub struct RenderPlugins;

impl PluginGroup for RenderPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(test::Plug);
    }
}

