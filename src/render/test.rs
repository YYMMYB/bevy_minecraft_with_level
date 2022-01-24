use bevy::prelude::{Plugin, App, Res, info};

use crate::logic::BlockMap;

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(print_system);
    }
}

fn print_system(
    map:Res<BlockMap>
){
    info!("print_system");
    for (l,c,b) in map.iter(){
        info!("{:?}", (l,c,b));
    }
}