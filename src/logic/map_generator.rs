use bevy::prelude::Plugin;

use crate::core::{Block, Coord, DIV};

use super::config::*;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut bevy::prelude::App) {}
}

pub trait Generator {
    fn get(&self, lv: i32, c: Coord) -> Option<Block>;
}

pub struct Test<'a> {
    pub cfg: &'a Map,
}

impl<'a> Generator for Test<'a> {
    fn get(&self, lv: i32, c: Coord) -> Option<Block> {
        let mut h0 = DIV * (1f32 - DIV.powi(lv));
        if lv > 0 {
            h0 = h0 - DIV;
        }
        let h = h0 * DIV.powi(-lv + self.cfg.level_interval);
        let h = h.round() as i32;

        if c.0.y == h {
            Some(Block::default())
        } else {
            None
        }
    }
}
