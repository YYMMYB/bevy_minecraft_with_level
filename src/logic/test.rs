use bevy::{prelude::{Plugin, ResMut, Res, info}, math::IVec3};

use crate::{core::Coord, logic::map_generator::Test};

use super::{config::Map, block_map::BlockMap};

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(build_block_map);
    }
}

fn build_block_map(
    mut map:ResMut<BlockMap>,
    world_config:Res<Map>,
){
    info!("build_block_map");
    let gen = Test{
        cfg:&world_config
    };
    let a:Coord = IVec3::new(1,1024,1).into();
    map.generate_in(gen, -5..10, -a..a);
}