use bevy::{prelude::{Plugin, ResMut}, pbr::wireframe::{WireframePlugin, WireframeConfig}, render::{render_resource::WgpuFeatures, options::WgpuOptions}};

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .insert_resource(WgpuOptions {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..Default::default()
        })
        .add_plugin(WireframePlugin);
    }
}

pub fn setup(
    mut cfg:ResMut<WireframeConfig>
){
    cfg.global = true;
}