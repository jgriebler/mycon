// Copyright 2018 Johannes M. Griebler
//
// This file is part of mycon.
//
// mycon is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// mycon is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with mycon.  If not, see <https://www.gnu.org/licenses/>.

//! Various types and data structures used for representing program state.

pub(crate) mod space;
pub(crate) mod stack;

use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};

const SPACE: i32 = ' ' as i32;

/// The universal type of data upon which a Befunge-98 program operates.
pub(crate) type Value = i32;

/// A point in funge space.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Point {
    /// The x coordinate of the point.
    pub(crate) x: i32,
    /// The y coordinate of the point.
    pub(crate) y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// An offset vector in funge space.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Delta {
    /// The x component of the offset vector.
    pub(crate) dx: i32,
    /// The y component of the offset vector.
    pub(crate) dy: i32,
}

impl Delta {
    /// Returns the negative to the given `Delta`.
    pub(crate) fn reverse(&self) -> Self {
        Delta {
            dx: -self.dx,
            dy: -self.dy,
        }
    }

    /// Returns the original `Delta` rotated 90 degrees to the left.
    pub(crate) fn rotate_left(&self) -> Self {
        Delta {
            dx: self.dy,
            dy: -self.dx,
        }
    }

    /// Returns the original `Delta` rotated 90 degrees to the right.
    pub(crate) fn rotate_right(&self) -> Self {
        Delta {
            dx: -self.dy,
            dy: self.dx,
        }
    }
}

impl fmt::Display for Delta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.dx, self.dy)
    }
}

impl Add<Delta> for Point {
    type Output = Self;

    fn add(self, delta: Delta) -> Self {
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
    type Output = Self;

    fn sub(self, delta: Delta) -> Self {
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
    type Output = Self;

    fn mul(self, n: i32) -> Self {
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
