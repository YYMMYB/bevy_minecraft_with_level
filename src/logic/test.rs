use bevy::{
    math::IVec3,
    prelude::{info, Plugin, Res, ResMut},
};

use crate::{core::Coord, logic::map_generator::{Tower, OneBlock}};

use super::{block_map::BlockMap, config::Map, map_generator::Generator};

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(build_block_map);
    }
}

pub fn build_block_map(mut map: ResMut<BlockMap>, world_config: Res<Map>) {
    info!("build_block_map");
    let gen = Tower { cfg: &world_config };
    let a: Coord = IVec3::new(0, -1024, 0).into();
    let b: Coord = IVec3::new(16, 1024, 16).into();
    map.generate_in(gen, -2..10, a..b);
}

