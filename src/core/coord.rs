use std::{
    cmp::Ordering,
    collections::binary_heap::Iter,
    ops::{self, *},
};

use bevy::math::{BVec3, IVec3};
use derive_more::{
    Add, BitAnd, BitOr, BitXor, Constructor, Div, From, Into, Mul, Neg, Not, Rem, Shl, Shr, Sub,
};

pub const DIVI: i32 = 2i32;
pub const DIV: f32 = 2f32;

pub trait TransLevel {
    fn trans_level(self, from: i32, to: i32) -> Self;
}

impl TransLevel for Coord {
    fn trans_level(self, from: i32, to: i32) -> Self {
        let lv = from - to;
        if lv >= 0 {
            self << lv
        } else {
            self >> (-lv)
        }
    }
}

impl TransLevel for Range<Coord> {
    fn trans_level(self, from: i32, to: i32) -> Self {
        self.start.trans_level(from, to)..self.end.trans_level(from, to)
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    From,
    Into,
    Constructor,
    Not,
    Neg,
    Add,
    Sub,
    BitAnd,
    BitOr,
    BitXor,
    Mul,
    Div,
    Rem,
    Shr,
    Shl,
)]
pub struct Coord(pub IVec3);

impl Coord {
    pub fn cmpeq(self, other: Self) -> BVec3 {
        self.0.cmpeq(other.0)
    }
    pub fn cmpne(self, other: Self) -> BVec3 {
        self.0.cmpne(other.0)
    }
    pub fn cmpge(self, other: Self) -> BVec3 {
        self.0.cmpge(other.0)
    }
    pub fn cmpgt(self, other: Self) -> BVec3 {
        self.0.cmpgt(other.0)
    }
    pub fn cmple(self, other: Self) -> BVec3 {
        self.0.cmple(other.0)
    }
    pub fn cmplt(self, other: Self) -> BVec3 {
        self.0.cmplt(other.0)
    }
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.cmpgt(*other).all() {
            Some(Ordering::Greater)
        } else if self.cmplt(*other).all() {
            Some(Ordering::Less)
        } else if self == other {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}

pub fn xzy_iter(r: Range<Coord>) -> impl Iterator<Item = Coord> {
    (r.start.0.z..r.end.0.z)
        .map(move |z| {
            (r.start.0.y..r.end.0.y)
                .map(move |y| (r.start.0.x..r.end.0.x).map(move |x| IVec3::new(x, y, z).into()))
        })
        .flatten()
        .flatten()
}
