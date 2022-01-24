use bevy::prelude::Plugin;


pub struct ConfigPlugin;
impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(WorldConfig::default());
    }
}

pub struct WorldConfig{
    pub level_interval:i32,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self { level_interval:5 }
    }
}


