//! Various types and data structures used for representing program state.

pub mod space;
pub mod stack;

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};

pub const SPACE: i32 = ' ' as i32;

/// The universal type of data upon which a Befunge-98 program operates.
pub type Value = i32;

/// A point in funge space.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// An offset vector in funge space.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Delta {
    pub dx: i32,
    pub dy: i32,
}

impl Delta {
    /// Returns the negative to the given `Delta`.
    pub fn reverse(&self) -> Delta {
        Delta {
            dx: -self.dx,
            dy: -self.dy,
        }
    }

    /// Returns the original `Delta` rotated 90 degrees to the left.
    pub fn rotate_left(&self) -> Delta {
        Delta {
            dx: self.dy,
            dy: -self.dx,
        }
    }

    /// Returns the original `Delta` rotated 90 degrees to the right.
    pub fn rotate_right(&self) -> Delta {
        Delta {
            dx: -self.dy,
            dy: self.dx,
        }
    }
}

impl Add<Delta> for Point {
    type Output = Point;

    fn add(self, delta: Delta) -> Point {
        Point {
            x: self.x + delta.dx,
            y: self.y + delta.dy,
        }
    }
}

impl AddAssign<Delta> for Point {
    fn add_assign(&mut self, delta: Delta) {
        self.x += delta.dx;
        self.y += delta.dy;
    }
}

impl Sub<Delta> for Point {
    type Output = Point;

    fn sub(self, delta: Delta) -> Point {
        Point {
            x: self.x - delta.dx,
            y: self.y - delta.dy,
        }
    }
}

impl SubAssign<Delta> for Point {
    fn sub_assign(&mut self, delta: Delta) {
        self.x -= delta.dx;
        self.y -= delta.dy;
    }
}

impl Mul<i32> for Delta {
    type Output = Delta;

    fn mul(self, n: i32) -> Delta {
        Delta {
            dx: self.dx * n,
            dy: self.dy * n,
        }
    }
}

impl MulAssign<i32> for Delta {
    fn mul_assign(&mut self, n: i32) {
        self.dx *= n;
        self.dy *= n;
    }
}
