use std::ops::Range;

use bevy::{math::IVec3, prelude::{Plugin, info, debug}, utils::HashMap};

use crate::core::*;

use super::map_generator::Generator;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(BlockMap::default());
    }
}

pub struct BlockMap {
    map: HashMap<i32, HashMap<Coord, Block>>,
}

impl Default for BlockMap {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl BlockMap {
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (i32, Coord, &'a Block)> {
        self.map
            .iter()
            .map(|(&l, m)| m.iter().map(move |(&c, b)| (l, c, b)))
            .flatten()
    }

    pub fn generate_in(
        &mut self,
        gen: impl Generator,
        level_range: Range<i32>,
        coord_range: Range<Coord>,
    ) {
        let lv0 = level_range.start;
        for l in level_range {
            let coord_range = coord_range.clone().trans_level(lv0, l);
            debug!("{:?}", (l, &coord_range));
            for c in xzy_iter(coord_range) {
                if let Some(b) = gen.get(l, c) {
                    if !self.map.contains_key(&l) {
                        self.map.insert(l, Default::default());
                    }
                    self.map.get_mut(&l).unwrap().insert(c, b);
                }
            }
        }
    }
}
