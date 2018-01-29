mod tree;

use super::{Value, SPACE};
use self::tree::*;

// const CACHE_SIZE: usize = 4;

pub struct Space {
    tree: QTree,
    bounds: Bounds,
//    cache: [Chunk; CACHE_SIZE],
//    cache_indices: [(isize, isize); CACHE_SIZE],
}

impl Space {
    pub fn new() -> Space {
        Space {
            tree: QTree::default(),
            bounds: Bounds::new(),
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Value {
        self.tree.get(x, y)
    }

    pub fn set(&mut self, x: i32, y: i32, value: Value) {
        self.tree.set(x, y, value);
        if value != SPACE {
            self.bounds.update(x, y);
        }
    }

    pub fn min(&self) -> (i32, i32) {
        self.bounds.min()
    }

    pub fn max(&self) -> (i32, i32) {
        self.bounds.max()
    }
}

impl<'a> From<&'a str> for Space {
    fn from(code: &'a str) -> Space {
        let mut space = Space::new();

        let mut x = 0;
        let mut y = 0;

        for c in code.chars() {
            if c == '\n' {
                x = 0;
                y += 1;
            } else {
                space.set(x, y, Value::from(c as i32));
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

    fn update(&mut self, x: i32, y: i32) {
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
            space.set(x, 0, x);
        }

        for n in 64..128 {
            space.set(n, n, -n);
        }

        for x in 0..32 {
            assert_eq!(x, space.get(x, 0));
        }

        for n in 64..128 {
            assert_eq!(-n, space.get(n, n));
        }

        assert_eq!(SPACE, space.get(-42, 70));
        assert_eq!((0, 0), space.min());
        assert_eq!((127, 127), space.max());

        space.set(-10, -10, SPACE);

        assert_eq!((0, 0), space.min());

        space.set(-10, -10, 'x' as i32);

        assert_eq!((-10, -10), space.min());
    }

    #[test]
    fn space_from() {
        let code = "123\n456\n789";
        let space = Space::from(code);

        for i in 0..9 {
            assert_eq!(i + '1' as i32, space.get(i % 3, i / 3));
        }

        assert_eq!((2, 2), space.max());
    }
}
