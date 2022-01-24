use bevy::{prelude::App, DefaultPlugins};
use logic::LogicPlugins;

mod core;

mod logic;
mod render;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LogicPlugins)
        .run();
}
