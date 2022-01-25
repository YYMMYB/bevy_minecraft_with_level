use bevy::prelude::{Entity, Plugin};

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Ent::default())
        ;
    }
}

#[derive(Default,Debug)]
pub struct Ent {
    pub camera:Option<Entity>,
}