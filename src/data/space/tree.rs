use super::{Value, SPACE};

const CHUNK_SHIFT: usize = 4;
const CHUNK_SHIFT_TOTAL: usize = 7 * CHUNK_SHIFT;
const CHUNK_SIZE: usize = 1 << CHUNK_SHIFT;
const CHUNK_MASK: usize = (CHUNK_SIZE - 1) << CHUNK_SHIFT_TOTAL;

#[derive(Clone)]
pub struct Chunk {
    data: [[Value; CHUNK_SIZE]; CHUNK_SIZE],
}

#[derive(Clone)]
pub struct Node<T> {
    data: [[Option<Box<T>>; CHUNK_SIZE]; CHUNK_SIZE],
}

type Tree1 = Node<Chunk>;
type Tree2 = Node<Tree1>;
type Tree3 = Node<Tree2>;
type Tree4 = Node<Tree3>;
type Tree5 = Node<Tree4>;
type Tree6 = Node<Tree5>;

pub type QTree = Node<Tree6>;

pub trait Tree: Default {
    fn get(&self, x: i32, y: i32) -> Value;
    fn set(&mut self, x: i32, y: i32, value: Value);

    fn get_chunk(&self, x: i32, y: i32) -> Chunk;
    fn set_chunk(&mut self, x: i32, y: i32, chunk: Chunk);
}

impl Default for Chunk {
    fn default() -> Chunk {
        Chunk { data: [[SPACE; CHUNK_SIZE]; CHUNK_SIZE] }
    }
}

impl Tree for Chunk {
    fn get(&self, x: i32, y: i32) -> Value {
        let (i, j) = get_indices(x, y);

        self.data[i][j]
    }

    fn set(&mut self, x: i32, y: i32, value: Value) {
        let (i, j) = get_indices(x, y);

        self.data[i][j] = value;
    }

    fn get_chunk(&self, _: i32, _: i32) -> Chunk {
        self.clone()
    }

    fn set_chunk(&mut self, _: i32, _: i32, chunk: Chunk) {
        *self = chunk
    }
}

impl<T: Tree> Default for Node<T> {
    fn default() -> Node<T> {
        Node { data: Default::default() }
    }
}

impl<T: Tree> Tree for Node<T> {
    fn get(&self, x: i32, y: i32) -> Value {
        let (i, j) = get_indices(x, y);
        let (x, y) = shift(x, y);

        match self.data[i][j] {
            Some(ref tree) => tree.get(x, y),
            None           => SPACE,
        }
    }

    fn set(&mut self, x: i32, y: i32, value: Value) {
        let (i, j) = get_indices(x, y);
        let (x, y) = shift(x, y);

        let mut tree = match self.data[i][j].take() {
            Some(tree) => tree,
            None       => if value == SPACE {
                return;
            } else {
                Box::new(T::default())
            },
        };

        tree.set(x, y, value);
        self.data[i][j] = Some(tree);
    }

    fn get_chunk(&self, x: i32, y: i32) -> Chunk {
        let (i, j) = get_indices(x, y);
        let (x, y) = shift(x, y);

        match self.data[i][j] {
            Some(ref tree) => tree.get_chunk(x, y),
            None           => Chunk::default(),
        }
    }

    fn set_chunk(&mut self, x: i32, y: i32, chunk: Chunk) {
        let (i, j) = get_indices(x, y);
        let (x, y) = shift(x, y);

        let mut tree = match self.data[i][j].take() {
            Some(tree) => tree,
            None       => Box::new(T::default())
        };

        tree.set_chunk(x, y, chunk);
        self.data[i][j] = Some(tree);
    }
}

fn get_indices(x: i32, y: i32) -> (usize, usize) {
    ((x as usize & CHUNK_MASK) >> CHUNK_SHIFT_TOTAL,
    (y as usize & CHUNK_MASK) >> CHUNK_SHIFT_TOTAL)
}

fn shift(x: i32, y: i32) -> (i32, i32) {
    (y << CHUNK_SHIFT, x << CHUNK_SHIFT)
}
