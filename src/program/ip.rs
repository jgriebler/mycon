//! A single instruction pointer in a running program.

use rand;

use data::{Value, Point, Delta};
use data::space::Space;
use data::stack::StackStack;

/// An instruction pointer in a running program.
#[derive(Clone)]
pub struct Ip {
    id: Value,
    position: Point,
    delta: Delta,
    storage: Delta,
    stacks: StackStack,
}

impl Ip {
    /// Creates a new `Ip` at the origin, facing east.
    ///
    /// The `Ip` will be in the configuration it should have at program start:
    /// Its [`Delta`] will be `(1, 0)`, its [`StackStack`] will contain a single
    /// empty stack.
    ///
    /// [`Delta`]: ../../data/struct.Delta.html
    /// [`StackStack`]: ../../data/stack/struct.StackStack.html
    pub fn new() -> Ip {
        Ip {
            id: 0,
            position: Point { x: 0, y: 0 },
            delta: Delta { dx: 1, dy: 0 },
            storage: Delta { dx: 0, dy: 0 },
            stacks: StackStack::new(),
        }
    }

    /// Returns the [`Value`] at the `Ip`'s current position.
    ///
    /// [`Value`]: ../../data/struct.Value.html
    pub fn get(&self, space: &Space) -> Value {
        space.get(self.position)
    }

    pub fn get_offset(&mut self, space: &Space) {
        let y = self.pop();
        let x = self.pop();

        let v = space.get(Point { x, y } + self.storage);
        self.push(v);
    }

    pub fn put_offset(&mut self, space: &mut Space) {
        let y = self.pop();
        let x = self.pop();
        let v = self.pop();

        space.set(Point { x, y } + self.storage, v);
    }

    /// Advances the `Ip`'s position by step of its current [`Delta`].
    ///
    /// [`Delta`]: ../../data/struct.Delta.html
    pub fn step(&mut self, space: &Space) {
        match space.maybe_wrap(self.position, self.delta) {
            Some(position) => self.position = position,
            None           => self.position += self.delta,
        }
    }

    pub fn jump(&mut self, space: &Space, n: Value) {
        let delta = self.delta;

        self.delta *= n - 1;
        self.step(space);

        self.delta = delta;
    }

    /// Sets the `Ip`'s [`Delta`] to a new value.
    ///
    /// [`Delta`]: ../../data/struct.Delta.html
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

    /// Pops the top [`Value`] off the `Ip`'s [`StackStack`].
    ///
    /// [`Value`]: ../../data/struct.Value.html
    /// [`StackStack`]: ../../data/stack/struct.StackStack.html
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

    /// Advances the `Ip`'s position to the next command in its path.
    ///
    /// Any intervening empty space or areas delimited by semicolons will be
    /// skipped.
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
