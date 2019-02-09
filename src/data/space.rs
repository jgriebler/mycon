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

//! The two-dimensional space addressable by a Befunge-98 program.

mod tree;

use std::collections::BTreeMap;

use super::{Value, Point, Delta, SPACE};
use self::tree::*;

/// The space in which a Befunge-98 program resides.
///
/// Internally, the space is represented by a data structure similar to a
/// quadtree, though each subdivision partitions the region into a 16x16 grid of
/// subtrees instead of 2x2. The entire theoretically addressable space is
/// represented by a tree of depth 8.
///
/// Memory for representing parts of this tree will be allocated when data is
/// written to a previously empty region. An uninitialized portion of the tree
/// represents a region containing only empty space (' ' characters), which is
/// completely transparent from the point of view of the program.
#[derive(Clone)]
pub(crate) struct Space {
    tree: FungeTree,
    bounds: Bounds,
}

impl Space {
    /// Creates a new empty `Space`.
    pub(crate) fn new() -> Self {
        Space {
            tree: FungeTree::default(),
            bounds: Bounds::new(),
        }
    }

    /// Creates a new `Space` containing the given source code.
    pub(crate) fn read(code: &str) -> Self {
        let mut space = Space::new();
        let mut longest = 0;
        let mut n_lines = 0;

        for (y, l) in code.lines().enumerate() {
            let n = space.set_line(y as i32, l);
            n_lines += 1;

            if n > longest {
                longest = n;
            }
        }

        for x in 0..longest as i32 {
            let mut n = 0;

            for y in 0..n_lines as i32 {
                if space.get(Point { x, y }) != SPACE {
                    n += 1;
                }
            }

            space.bounds.set_x(x, n);
        }

        space.bounds.set_min_max();

        space
    }

    /// Retrieves the [`Value`] stored at the given [`Point`] in the `Space`.
    ///
    /// If this particular part of the `Space` has not yet been initialized,
    /// `32` (the ' ' character) will be returned.
    ///
    /// [`Value`]: ../type.Value.html
    /// [`Point`]: ../struct.Point.html
    pub(crate) fn get(&self, Point { x, y }: Point) -> Value {
        self.tree.get(x, y)
    }

    /// Puts the [`Value`] at the specified [`Point`] in the `Space`.
    ///
    /// If this particular part of the `Space` has not yet been initialized and
    /// the [`Value`] is not 32 (the ' ' character), a new chunk of cells will
    /// be allocated.
    ///
    /// [`Value`]: ../type.Value.html
    /// [`Point`]: ../struct.Point.html
    pub(crate) fn set(&mut self, Point { x, y }: Point, value: Value) {
        let old = self.tree.set(x, y, value);
        self.bounds.update(Point { x, y }, old, value);
    }

    /// Returns the northwest corner `(x, y)` of the bounding box of the
    /// programs source code.
    ///
    /// The bounds are updated whenever a [`Value`] other than 32 (space) is
    /// written to a [`Point`] outside the current bounding box.
    ///
    /// Note that the bounding box will not be shrunk if such a cell is replaced
    /// by a space again.
    ///
    /// [`Value`]: ../type.Value.html
    /// [`Point`]: ../struct.Point.html
    pub(crate) fn min(&self) -> (i32, i32) {
        self.bounds.min()
    }

    /// Returns the southeast corner `(x, y)` of the bounding box of the
    /// programs source code.
    ///
    /// The bounds are updated whenever a [`Value`] other than 32 (space) is
    /// written to a [`Point`] outside the current bounding box.
    ///
    /// Note that the bounding box will not be shrunk if such a cell is replaced
    /// by a space again.
    ///
    /// [`Value`]: ../type.Value.html
    /// [`Point`]: ../struct.Point.html
    pub(crate) fn max(&self) -> (i32, i32) {
        self.bounds.max()
    }

    /// Advances the [`Point`] `p` by the [`Delta`] `d`, potentially wrapping to
    /// the other side of the `Space`.
    ///
    /// If `p + d` would be outside the bounding box, returns the point of
    /// reentry on the other side, otherwise `p + d` is returned.
    ///
    /// [`Point`]: ../struct.Point.html
    /// [`Delta`]: ../struct.Delta.html
    pub(crate) fn new_position(&self, Point { x, y }: Point, Delta { dx, dy }: Delta) -> Point {
        use std::cmp::min;

        let (min_x, min_y) = self.bounds.min();
        let (max_x, max_y) = self.bounds.max();

        let (last_x, sx) = if dx >= 0 {
            (x > max_x - dx, x - min_x)
        } else {
            (x < min_x - dx, x - max_x)
        };

        let (last_y, sy) = if dy >= 0 {
            (y > max_y - dy, y - min_y)
        } else {
            (y < min_y - dy, y - max_y)
        };

        if last_x || last_y {
            let nx = if dx == 0 {
                i32::max_value()
            } else {
                sx / dx
            };
            let ny = if dy == 0 {
                i32::max_value()
            } else {
                sy / dy
            };
            let n = min(nx, ny);

            Point { x, y } - Delta { dx, dy } * n
        } else {
            Point { x, y } + Delta { dx, dy }
        }
    }

    /// Checks whether adding the [`Delta`] to the [`Point`] would be outside
    /// the bounding box.
    pub(crate) fn is_last(&self, Point { x, y }: Point, Delta { dx, dy }: Delta) -> bool {
        let (min_x, min_y) = self.bounds.min();
        let (max_x, max_y) = self.bounds.max();

        let last_x = if dx >= 0 {
            x > max_x - dx
        } else {
            x < min_x - dx
        };

        let last_y = if dy >= 0 {
            y > max_y - dy
        } else {
            y < min_y - dy
        };

        last_x || last_y
    }

    fn set_line(&mut self, y: i32, line: &str) -> u32 {
        let mut l = 0;

        let f = |c| {
            let v = c as i32;
            if v == 12 {
                None
            } else {
                l += 1;
                Some(v)
            }
        };

        let mut n = 0;

        for (x, v) in line.chars().filter_map(f).enumerate() {
            self.tree.set(x as i32, y, v);

            if v != SPACE {
                n += 1;
            }
        }

        self.bounds.set_y(y, n);

        l
    }
}

#[derive(Clone)]
struct Bounds {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
    nonempty_x: BTreeMap<i32, u32>,
    nonempty_y: BTreeMap<i32, u32>,
}

impl Bounds {
    fn new() -> Bounds {
        Bounds {
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 0,
            nonempty_x: BTreeMap::new(),
            nonempty_y: BTreeMap::new(),
        }
    }

    fn set_x(&mut self, x: i32, n: u32) {
        *self.nonempty_x.entry(x).or_insert(0) += n;
    }

    fn set_y(&mut self, y: i32, n: u32) {
        *self.nonempty_y.entry(y).or_insert(0) += n;
    }

    fn update(&mut self, Point { x, y }: Point, old: Value, new: Value) {
        if old == SPACE && new != SPACE {
            *self.nonempty_x.entry(x).or_insert(0) += 1;
            *self.nonempty_y.entry(y).or_insert(0) += 1;

            self.set_min_max();
        } else if old != SPACE && new == SPACE {
            self.nonempty_x.entry(x).and_modify(|r| *r -= 1);
            self.nonempty_y.entry(y).and_modify(|r| *r -= 1);

            self.set_min_max();
        }
    }

    fn set_min_max(&mut self) {
        let f = |(i, n): (&i32, &u32)| {
            if *n == 0 {
                None
            } else {
                Some(*i)
            }
        };

        self.min_x = self.nonempty_x.iter().filter_map(f).next().unwrap_or(0);
        self.min_y = self.nonempty_y.iter().filter_map(f).next().unwrap_or(0);
        self.max_x = self.nonempty_x.iter().filter_map(f).next_back().unwrap_or(0);
        self.max_y = self.nonempty_y.iter().filter_map(f).next_back().unwrap_or(0);
    }

    fn min(&self) -> (i32, i32) {
        (self.min_x, self.min_y)
    }

    fn max(&self) -> (i32, i32) {
        (self.max_x, self.max_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn space_get_uninit() {
        let space = Space::new();

        assert_eq!(SPACE, space.get(Point { x: 0, y: 0 }));
    }

    #[test]
    fn space_get_empty() {
        let mut space = Space::new();

        space.set(Point { x: 0, y: 0 }, 40);

        assert_eq!(SPACE, space.get(Point { x: 1, y: 0 }));
    }

    #[test]
    fn space_set_get() {
        let mut space = Space::new();

        let position = Point { x: 3, y: 6 };
        let value = 45;

        space.set(position, value);

        assert_eq!(value, space.get(position));
    }

    #[test]
    fn space_set_get_large() {
        let mut space = Space::new();

        let position = Point { x: 2147483647, y: -1029771328 };
        let value = 1307812;

        space.set(position, value);

        assert_eq!(value, space.get(position));
    }

    #[test]
    fn space_set_get_multiple() {
        let mut space = Space::new();

        let data = [
            (Point { x:  0, y:  0 },  12),
            (Point { x:  3, y:  2 },   0),
            (Point { x: -2, y: -1 }, -42),
            (Point { x:  1, y: -3 },   6),
        ];

        for &(p, v) in data.iter() {
            space.set(p, v);
        }

        for &(p, v) in data.iter() {
            assert_eq!(v, space.get(p));
        }
    }

    #[test]
    fn space_init_bounds() {
        let mut space = Space::new();

        let (x, y) = (2, -3);

        space.set(Point { x, y }, 12);

        assert_eq!((x, y), space.min());
        assert_eq!((x, y), space.max());
    }

    #[test]
    fn space_grow_bounds() {
        let mut space = Space::new();

        space.set(Point { x: 0, y: 0 }, 42);

        let (x0, y0) = (-3, 5);
        let (x1, y1) = (2, -1);

        space.set(Point { x: x0, y: y0 }, 1);
        space.set(Point { x: x1, y: y1 }, 2);

        assert_eq!((-3, -1), space.min());
        assert_eq!((2, 5), space.max());
    }

    #[test]
    fn space_keep_bounds() {
        let mut space = Space::new();

        space.set(Point { x: 0, y: 0 }, 42);
        space.set(Point { x: -2, y: 3 }, SPACE);

        assert_eq!((0, 0), space.min());
        assert_eq!((0, 0), space.max());
    }

    #[test]
    fn space_read() {
        let code = "123\n456\n789";
        let space = Space::read(code);

        for i in 0..9 {
            assert_eq!(i + '1' as i32, space.get(Point { x: i % 3, y: i / 3 }));
        }

        assert_eq!((2, 2), space.max());
    }

    #[test]
    fn space_read_bounds() {
        let code = " a  b\nc d\n e";
        let space = Space::read(code);

        let nx: Vec<_> = space.bounds.nonempty_x.iter().collect();
        let ny: Vec<_> = space.bounds.nonempty_y.iter().collect();

        assert_eq!(&[(&0, &1), (&1, &2), (&2, &1), (&3, &0), (&4, &1)], &nx[..]);
        assert_eq!(&[(&0, &2), (&1, &2), (&2, &1)], &ny[..]);
    }
}
