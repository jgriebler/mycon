use rand;

use data::{Value, Point, Delta};
use data::space::Space;
use data::stack::StackStack;

#[derive(Clone)]
pub struct Ip {
    id: Value,
    position: Point,
    delta: Delta,
    storage: Delta,
    stacks: StackStack,
}

impl Ip {
    pub fn new() -> Ip {
        Ip {
            id: 0,
            position: Point { x: 0, y: 0 },
            delta: Delta { dx: 1, dy: 0 },
            storage: Delta { dx: 0, dy: 0 },
            stacks: StackStack::new(),
        }
    }

    pub fn get(&self, space: &Space) -> Value {
        space.get(self.position)
    }

    pub fn step(&mut self, space: &Space) {
        match space.maybe_wrap(self.position, self.delta) {
            Some(position) => self.position = position,
            None           => self.position += self.delta,
        }
    }

    pub fn set_delta(&mut self, delta: Delta) {
        self.delta = delta;
    }

    pub fn randomize_delta(&mut self) {
        let (dx, dy) = match rand::random::<u8>() % 4 {
            0 => ( 1,  0),
            1 => ( 0,  1),
            2 => (-1,  0),
            3 => ( 0, -1),
            _ => unreachable!(),
        };

        self.set_delta(Delta { dx, dy });
    }

    pub fn reverse(&mut self) {
        self.delta = self.delta.reverse();
    }

    pub fn turn_left(&mut self) {
        self.delta = self.delta.rotate_left();
    }

    pub fn turn_right(&mut self) {
        self.delta = self.delta.rotate_right();
    }

    pub fn push(&mut self, value: Value) {
        self.stacks.push(value);
    }

    pub fn pop(&mut self) -> Value {
        self.stacks.pop()
    }

    pub fn duplicate(&mut self) {
        let v = self.pop();

        self.push(v);
        self.push(v);
    }

    pub fn swap(&mut self) {
        let v = self.pop();
        let w = self.pop();

        self.push(v);
        self.push(w);
    }

    pub fn clear(&mut self) {
        self.stacks.clear();
    }

    pub fn find_command(&mut self, space: &Space) {
        let mut skip = false;

        loop {
            match self.get(space) {
                32        => (),
                59        => skip = !skip,
                _ if skip => (),
                _         => return,
            }

            self.step(space);
        }
    }

    pub fn peek_command(&mut self, space: &Space) -> Value {
        let orig_position = self.position;

        self.find_command(space);

        let ret = self.get(space);

        self.position = orig_position;

        ret
    }
}
