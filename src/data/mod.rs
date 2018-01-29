pub mod space;
pub mod stack;

use std::ops::Add;

pub const SPACE: i32 = ' ' as i32;

pub type Value = i32;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Delta {
    pub dx: i32,
    pub dy: i32,
}

impl Add<Delta> for Point {
    type Output = Point;

    fn add(self, del: Delta) -> Point {
        Point {
            x: self.x + del.dx,
            y: self.y + del.dy,
        }
    }
}
