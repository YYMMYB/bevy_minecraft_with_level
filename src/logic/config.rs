

pub struct Plug;
impl bevy::app::Plugin for Plug {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Map::default());
    }
}

pub struct Map{
    pub level_interval:i32,
}

impl Default for Map {
    fn default() -> Self {
        Self { level_interval:1 }
    }
}


