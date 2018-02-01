//! A single instruction pointer in a running program.

mod instruction;

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
    pub fn get_current(&self, space: &Space) -> Value {
        space.get(self.position)
    }

    /// Executes a single command and moves the `Ip` to the next.
    pub fn tick(&mut self, space: &mut Space) -> ExecResult {
        let v = self.get_current(space);

        let result = if let Some(c) = ::std::char::from_u32(v as u32) {
            self.execute(space, c)
        } else {
            self.reverse();
            ExecResult::Done
        };

        self.step(space);
        self.find_command(space);
        result
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

    /// Executes a single command, without moving the `Ip`'s afterwards.
    pub fn execute(&mut self, space: &mut Space, command: char) -> ExecResult {
        match command {
            ' '         => unreachable!("space"),
            '!'         => self.negate(),
            '"'         => self.reverse(), // TODO implement
            '#'         => self.trampoline(space),
            '$'         => self.discard(),
            '%'         => self.rem(),
            '&'         => self.reverse(), // TODO implement
            '\''        => self.reverse(), // TODO implement
            '('         => self.reverse(), // TODO implement
            ')'         => self.reverse(), // TODO implement
            '*'         => self.mul(),
            '+'         => self.add(),
            ','         => self.reverse(), // TODO implement
            '-'         => self.sub(),
            '.'         => self.reverse(), // TODO implement
            '/'         => self.div(),
            '0'         => self.push_zero(),
            '1'         => self.push_one(),
            '2'         => self.push_two(),
            '3'         => self.push_three(),
            '4'         => self.push_four(),
            '5'         => self.push_five(),
            '6'         => self.push_six(),
            '7'         => self.push_seven(),
            '8'         => self.push_eight(),
            '9'         => self.push_nine(),
            ':'         => self.duplicate(),
            ';'         => unreachable!("semicolon"),
            '<'         => self.go_west(),
            '='         => self.reverse(), // TODO implement
            '>'         => self.go_east(),
            '?'         => self.randomize_delta(),
            '@'         => return ExecResult::DeleteIp,
            'A' ... 'Z' => self.reverse(), // TODO implement
            '['         => self.turn_left(),
            '\\'        => self.swap(),
            ']'         => self.turn_right(),
            '^'         => self.go_north(),
            '_'         => self.if_east_west(),
            '`'         => self.greater_than(),
            'a'         => self.push_ten(),
            'b'         => self.push_eleven(),
            'c'         => self.push_twelve(),
            'd'         => self.push_thirteen(),
            'e'         => self.push_fourteen(),
            'f'         => self.push_fifteen(),
            'g'         => self.get(space),
            'h'         => self.reverse(),
            'i'         => self.reverse(), // TODO implement
            'j'         => self.jump(space),
            'k'         => self.reverse(), // TODO implement
            'l'         => self.reverse(),
            'm'         => self.reverse(),
            'n'         => self.clear(),
            'o'         => self.reverse(), // TODO implement
            'p'         => self.put(space),
            'q'         => return ExecResult::Terminate(self.pop()),
            'r'         => self.reverse(),
            's'         => self.reverse(), // TODO implement
            't'         => self.reverse(), // TODO implement
            'u'         => self.reverse(), // TODO implement
            'v'         => self.go_south(),
            'w'         => self.compare(),
            'x'         => self.absolute_delta(),
            'y'         => self.reverse(), // TODO implement
            'z'         => (),
            '{'         => self.reverse(), // TODO implement
            '|'         => self.if_north_south(),
            '}'         => self.reverse(), // TODO implement
            '~'         => self.reverse(), // TODO implement
            _           => self.reverse(),
        }

        ExecResult::Done
    }

    /// Sets the `Ip`'s [`Delta`] to a new value.
    ///
    /// [`Delta`]: ../../data/struct.Delta.html
    pub fn set_delta(&mut self, delta: Delta) {
        self.delta = delta;
    }

    /// Pushes a [`Value`] to the `Ip`'s [`StackStack`].
    ///
    /// [`Value`]: ../../data/struct.Value.html
    /// [`StackStack`]: ../../data/stack/struct.StackStack.html
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

    /// Advances the `Ip`'s position to the next command in its path.
    ///
    /// Any intervening empty space or areas delimited by semicolons will be
    /// skipped.
    pub fn find_command(&mut self, space: &Space) {
        let mut skip = false;

        loop {
            match self.get_current(space) {
                32        => (),
                59        => skip = !skip,
                _ if skip => (),
                _         => return,
            }

            self.step(space);
        }
    }

    /// Finds the next command in the `Ip`'s path, without moving it.
    pub fn peek_command(&mut self, space: &Space) -> Value {
        let orig_position = self.position;

        self.find_command(space);

        let ret = self.get_current(space);

        self.position = orig_position;

        ret
    }
}

pub enum ExecResult {
    Done,
    AddIp(Ip),
    DeleteIp,
    Terminate(Value),
}
