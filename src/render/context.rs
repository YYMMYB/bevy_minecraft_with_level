use bevy::{
    math,
    pbr::{AlphaMode, StandardMaterial},
    prelude::{Assets, Color, Entity, Handle, Plugin, ResMut},
};

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Ent::default())
            .insert_resource(Ctr::default())
            .insert_resource(Dbg::default());
    }
}

pub fn setup(mut dbg: ResMut<Dbg>, mut materials: ResMut<Assets<StandardMaterial>>) {
    dbg.block_mat = materials.add(StandardMaterial {
        base_color: Color::rgba(0.2f32, 0.88f32, 0.94f32, 1f32),
        // alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });
}

#[derive(Default, Debug)]
pub struct Ent {
    pub camera: Option<Entity>,
}

#[derive(Debug)]
pub struct Ctr {
    pub view_speed: f32,
    pub move_speed: f32,
}

impl Default for Ctr {
    fn default() -> Self {
        Self {
            view_speed: 0.001f32,
            move_speed: 5f32,
        }
    }
}

pub struct Dbg {
    pub block_mat: Handle<StandardMaterial>,
}

impl Default for Dbg {
    fn default() -> Self {
        Self {
            block_mat: Handle::default(),
        }
    }
}
