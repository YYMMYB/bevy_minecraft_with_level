use bevy::prelude::{Plugin, App, Res, info};

use crate::logic::block_map::BlockMap;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_startup_system(print_system);
    }
}

pub fn print_system(
    map:Res<BlockMap>
){
    info!("print_system");
    for (l,c,b) in map.iter(){
        info!("{:?}", (l,c,b));
    }
}