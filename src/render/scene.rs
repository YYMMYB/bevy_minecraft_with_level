use bevy::{prelude::*, input::Input};

use super::context;

pub fn spawn_base_env(
    mut ctx_ent:ResMut<context::Ent>,
    mut commands:Commands
    
){
    let cam = commands.spawn_bundle(PerspectiveCameraBundle{
        transform:Transform::from_xyz(0f32, 0f32, 10f32),
        ..Default::default()
    }).id();
    ctx_ent.camera = Some(cam);
}

pub fn cam_control(
    mut cam:Query<&Camera>,
    mouse:Res<Input<MouseButton>>
){}