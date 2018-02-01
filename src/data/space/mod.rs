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

    /// For a given [`Point`] `p` and [`Delta`] `d`, checks whether `p + d`
    /// would leave the bounding box.
    ///
    /// If `p + d` would be outside the bounding box, returns the point of
    /// reentry on the other side wrapped in `Some`, otherwise `None`.
    ///
    /// [`Point`]: ../struct.Point.html
    /// [`Delta`]: ../struct.Delta.html
    pub fn maybe_wrap(&self, Point { x, y }: Point, Delta { dx, dy }: Delta) -> Option<Point> {
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

            Some(Point { x, y } - Delta { dx, dy } * n)
        } else {
            None
        }
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
            } else {
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
    fn space_get_set() {
        let mut space = Space::new();

        for x in 0..32 {
            space.set(Point { x, y: 0 }, x);
        }

        for n in 64..128 {
            space.set(Point { x: n, y: n }, -n);
        }

        for x in 0..32 {
            assert_eq!(x, space.get(Point { x, y: 0 }));
        }

        for n in 64..128 {
            assert_eq!(-n, space.get(Point { x: n, y: n }));
        }

        assert_eq!(SPACE, space.get(Point { x: -42, y: 70 }));
        assert_eq!((0, 0), space.min());
        assert_eq!((127, 127), space.max());

        space.set(Point { x: -10, y: -10 }, SPACE);

        assert_eq!((0, 0), space.min());

        space.set(Point { x: -10, y: -10 }, 'x' as i32);

        assert_eq!((-10, -10), space.min());
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
