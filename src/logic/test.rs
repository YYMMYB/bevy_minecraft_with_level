use bevy::{prelude::{Plugin, ResMut, Res, info}, math::IVec3};

use crate::core::Coord;

use super::{BlockMap, WorldConfig, TestMap};

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(build_block_map);
    }
}

fn build_block_map(
    mut map:ResMut<BlockMap>,
    world_config:Res<WorldConfig>,
){
    info!("build_block_map");
    let gen = TestMap{
        cfg:&world_config
    };
    let a:Coord = IVec3::new(1,1024,1).into();
    map.generate_in(gen, -5..10, -a..a);
}