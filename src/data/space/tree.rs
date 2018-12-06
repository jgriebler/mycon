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

use crate::data::{Value, SPACE};

const CHUNK_SHIFT: usize = 4;
const CHUNK_SHIFT_BACK: usize = 32 - CHUNK_SHIFT;
const CHUNK_SIZE: usize = 1 << CHUNK_SHIFT;
const CHUNK_MASK: usize = (CHUNK_SIZE - 1) << CHUNK_SHIFT_BACK;

#[derive(Clone)]
pub(super) struct Chunk {
    data: [[Value; CHUNK_SIZE]; CHUNK_SIZE],
}

#[derive(Clone)]
pub(super) struct Node<T> {
    data: [[Option<Box<T>>; CHUNK_SIZE]; CHUNK_SIZE],
}

type Tree1 = Node<Chunk>;
type Tree2 = Node<Tree1>;
type Tree3 = Node<Tree2>;
type Tree4 = Node<Tree3>;
type Tree5 = Node<Tree4>;
type Tree6 = Node<Tree5>;

pub(super) type FungeTree = Node<Tree6>;

pub(super) trait Tree: Default {
    fn get(&self, x: i32, y: i32) -> Value;
    fn set(&mut self, x: i32, y: i32, value: Value) -> Value;

//    fn get_chunk(&self, x: i32, y: i32) -> Chunk;
//    fn set_chunk(&mut self, x: i32, y: i32, chunk: Chunk);

    fn set_line<I>(&mut self, y: i32, values: &mut I) -> (bool, u32)
        where I: Iterator<Item = Value>;
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk { data: [[SPACE; CHUNK_SIZE]; CHUNK_SIZE] }
    }
}

impl Tree for Chunk {
    fn get(&self, x: i32, y: i32) -> Value {
        let (i, j) = get_indices(x, y);

        self.data[i][j]
    }

    fn set(&mut self, x: i32, y: i32, value: Value) -> Value {
        let (i, j) = get_indices(x, y);
        let old = self.data[i][j];

        self.data[i][j] = value;
        old
    }

//    fn get_chunk(&self, _: i32, _: i32) -> Chunk {
//        self.clone()
//    }
//
//    fn set_chunk(&mut self, _: i32, _: i32, chunk: Chunk) {
//        *self = chunk
//    }

    fn set_line<I>(&mut self, y: i32, values: &mut I) -> (bool, u32)
        where I: Iterator<Item = Value>
    {
        let (mut i, j) = get_indices(0, y);
        let mut nonspace = 0;

        while let Some(value) = values.next() {
            self.data[i][j] = value;
            i += 1;

            if value != SPACE {
                nonspace += 1;
            }

            if i == CHUNK_SIZE {
                return (false, nonspace);
            }
        }

        return (true, nonspace);
    }
}

impl<T: Tree> Default for Node<T> {
    fn default() -> Self {
        Node { data: Default::default() }
    }
}

impl<T: Tree> Tree for Node<T> {
    fn get(&self, x: i32, y: i32) -> Value {
        let (i, j) = get_indices(x, y);
        let (x, y) = shift(x, y);

        match &self.data[i][j] {
            Some(tree) => tree.get(x, y),
            None       => SPACE,
        }
    }

    fn set(&mut self, x: i32, y: i32, value: Value) -> Value {
        let (i, j) = get_indices(x, y);
        let (x, y) = shift(x, y);

        let mut tree = match self.data[i][j].take() {
            Some(tree) => tree,
            None       => if value == SPACE {
                return SPACE;
            } else {
                Box::new(T::default())
            },
        };

        let old = tree.set(x, y, value);
        self.data[i][j] = Some(tree);
        old
    }

//    fn get_chunk(&self, x: i32, y: i32) -> Chunk {
//        let (i, j) = get_indices(x, y);
//        let (x, y) = shift(x, y);
//
//        match self.data[i][j] {
//            Some(ref tree) => tree.get_chunk(x, y),
//            None           => Chunk::default(),
//        }
//    }
//
//    fn set_chunk(&mut self, x: i32, y: i32, chunk: Chunk) {
//        let (i, j) = get_indices(x, y);
//        let (x, y) = shift(x, y);
//
//        let mut tree = match self.data[i][j].take() {
//            Some(tree) => tree,
//            None       => Box::new(T::default())
//        };
//
//        tree.set_chunk(x, y, chunk);
//        self.data[i][j] = Some(tree);
//    }

    fn set_line<I>(&mut self, y: i32, values: &mut I) -> (bool, u32)
        where I: Iterator<Item = Value>
    {
        let (mut i, j) = get_indices(0, y);
        let (_, y) = shift(0, y);
        let mut nonspace = 0;

        while i != CHUNK_SIZE {
            let mut tree = match self.data[i][j].take() {
                Some(tree) => tree,
                None       => Box::new(T::default()),
            };

            let (done, n) = tree.set_line(y, values);
            self.data[i][j] = Some(tree);
            nonspace += n;

            if done {
                return (done, nonspace);
            }

            i += 1;
        }

        return (false, nonspace);
    }
}

fn get_indices(x: i32, y: i32) -> (usize, usize) {
    ((x as usize & CHUNK_MASK) >> CHUNK_SHIFT_BACK,
    (y as usize & CHUNK_MASK) >> CHUNK_SHIFT_BACK)
}

fn shift(x: i32, y: i32) -> (i32, i32) {
    (x << CHUNK_SHIFT, y << CHUNK_SHIFT)
}
