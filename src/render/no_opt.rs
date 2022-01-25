use bevy::{prelude::*, pbr::wireframe::Wireframe};

use crate::{core::DIV, logic::block_map::BlockMap};

pub fn blocks(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, map: Res<BlockMap>) {
    for (l, c, b) in map.iter() {
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(shape::Cube { size: DIV.powi(l) }.into()),
                ..Default::default()
            })
            ;
    }
}
