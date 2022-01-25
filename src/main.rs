#![feature(unchecked_math)]

use bevy::{
    prelude::App,
    DefaultPlugins,
};
use logic::LogicPlugins;
use render::RenderPlugins;

mod core;

mod logic;
mod render;

mod label;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LogicPlugins)
        .add_plugins(RenderPlugins)
        .run();
}
