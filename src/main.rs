#![feature(unchecked_math)]

use bevy::{prelude::App, DefaultPlugins};
use logic::LogicPlugins;
use render::RenderPlugins;

mod core;

mod logic;
mod render;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // todo 顺序
        .add_plugins(LogicPlugins)
        .add_plugins(RenderPlugins)
        .run();
}
