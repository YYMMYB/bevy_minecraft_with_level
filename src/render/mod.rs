use bevy::{
    pbr::wireframe::WireframePlugin,
    prelude::{info, Plugin, PluginGroup, SystemSet},
    render::{options::WgpuOptions, render_resource::WgpuFeatures},
};

use crate::label;

pub mod context;
pub mod enable_wireframe;
pub mod no_opt;
pub mod scene;
pub mod test;

pub struct RenderPlugins;

impl PluginGroup for RenderPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group
            .add(MainPlug)
            .add(context::Plug)
            .add(enable_wireframe::Plug);
    }
}

pub struct MainPlug;
impl Plugin for MainPlug {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system_set(
            SystemSet::new()
                .label(label::System::Render)
                .after(label::System::Logic)
                .with_system(context::setup)
                .with_system(no_opt::blocks)
                .with_system(test::print_to_console)
                .with_system(scene::spawn_base_env)
                .with_system(enable_wireframe::setup),
        )
        .add_system_set(SystemSet::new().with_system(scene::cam_control));
    }
}
