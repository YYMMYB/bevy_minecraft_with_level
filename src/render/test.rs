use bevy::{prelude::*, pbr::PbrBundle};

use crate::{logic::block_map::BlockMap, core::DIV};

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {

    }
}

pub fn print_to_console(
    map:Res<BlockMap>
){
    info!("print_system");
    for (l,c,b) in map.iter(){
        info!("{:?}", (l,c,b));
    }
}