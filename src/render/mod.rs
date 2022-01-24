use bevy::prelude::PluginGroup;
use self::test::TestPlugin;


mod test;

pub struct RenderPlugins;

impl PluginGroup for RenderPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(TestPlugin);
    }
}

