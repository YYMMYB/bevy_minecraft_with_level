use bevy::{prelude::*};

use crate::{
    core::{DIV, TransLevel},
    logic::{block_map::BlockMap, config},
};

use super::context;

pub fn blocks(
    mut commands: Commands,
    cfg: Res<config::Map>,
    dbg: Res<context::Dbg>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    map: Res<BlockMap>,
) {
    for (l, c, b) in map.iter() {
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(shape::Cube { size: DIV.powi(l) }.into()),
            material: dbg.block_mat.clone(),
            transform: Transform::from_translation(
                c.0.as_vec3().trans_level(l - cfg.level_interval,0) + Vec3::ONE * DIV.powi(l - 1),
            ),
            ..Default::default()
        });
    }
}
