//! The two-dimensional space addressable by a Befunge-98 program.

mod tree;

use super::{Value, Point, Delta, SPACE};
use self::tree::*;

// const CACHE_SIZE: usize = 4;

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
pub struct Space {
    tree: QTree,
    bounds: Bounds,
//    cache: [Chunk; CACHE_SIZE],
//    cache_indices: [(isize, isize); CACHE_SIZE],
}

impl Space {
    /// Creates a new empty `Space`.
    pub fn new() -> Space {
        Space {
            tree: QTree::default(),
            bounds: Bounds::new(),
        }
    }

    /// Retrieves the [`Value`] stored at the given [`Point`] in the `Space`.
    ///
    /// If this particular part of the `Space` has not yet been initialized,
    /// `32` (the ' ' character) will be returned.
    ///
    /// [`Value`]: ../type.Value.html
    /// [`Point`]: ../struct.Point.html
    pub fn get(&self, Point { x, y }: Point) -> Value {
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
    pub fn set(&mut self, Point { x, y }: Point, value: Value) {
        self.tree.set(x, y, value);
        if value != SPACE {
            self.bounds.update(Point { x, y });
        }
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
    pub fn min(&self) -> (i32, i32) {
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
    pub fn max(&self) -> (i32, i32) {
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
    pub fn new_position(&self, Point { x, y }: Point, Delta { dx, dy }: Delta) -> Point {
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
    pub fn is_last(&self, Point { x, y }: Point, Delta { dx, dy }: Delta) -> bool {
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
}

impl<'a> From<&'a str> for Space {
    /// Creates a new `Space` containing the given source code.
    fn from(code: &'a str) -> Space {
        let mut space = Space::new();

        let mut x = 0;
        let mut y = 0;

        for c in code.chars() {
            if c == '\n' {
                x = 0;
                y += 1;
            } else if c != '\r' && c != 12 as char {
                space.set(Point { x, y }, Value::from(c as i32));
                x += 1;
            }
        }

        space
    }
}

struct Bounds {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

impl Bounds {
    fn new() -> Bounds {
        Bounds {
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 0,
        }
    }

    fn update(&mut self, Point { x, y }: Point) {
        if x < self.min_x {
            self.min_x = x;
        } else if x > self.max_x {
            self.max_x = x;
        }

        if y < self.min_y {
            self.min_y = y;
        } else if y > self.max_y {
            self.max_y = y;
        }
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

        assert_eq!((0, y), space.min());
        assert_eq!((x, 0), space.max());
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
    fn space_from() {
        let code = "123\n456\n789";
        let space = Space::from(code);

        for i in 0..9 {
            assert_eq!(i + '1' as i32, space.get(Point { x: i % 3, y: i / 3 }));
        }

        assert_eq!((2, 2), space.max());
    }
}
