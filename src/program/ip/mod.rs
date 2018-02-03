//! A single instruction pointer in a running program.

mod instruction;

use data::{Value, Point, Delta};
use data::space::Space;
use data::stack::StackStack;
use super::config::IoContext;

/// An instruction pointer in a running program.
#[derive(Clone)]
pub struct Ip {
    id: Value,
    position: Point,
    delta: Delta,
    storage: Point,
    stacks: StackStack,
    string: bool,
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
            storage: Point { x: 0, y: 0 },
            stacks: StackStack::new(),
            string: false,
        }
    }

    /// Returns the [`Value`] at the `Ip`'s current position.
    ///
    /// [`Value`]: ../../data/struct.Value.html
    pub fn get_current(&self, space: &Space) -> Value {
        space.get(self.position)
    }

    /// Executes a single command and moves the `Ip` to the next.
    pub fn tick(&mut self, space: &mut Space, io: &mut IoContext, new_id: Value) -> ExecResult {
        if !self.string {
            self.find_command(space);
        }

        let v = self.get_current(space);

        if self.string {
            if v == 34 {
                self.string = false;
                self.step(space);
                self.find_command(space);
            } else {
                self.push(v);
                self.step(space);

                if v == 32 {
                    self.skip_space(space);
                }
            }

            return ExecResult::Done;
        }

        let result = if let Some(c) = ::std::char::from_u32(v as u32) {
            self.execute(space, io, new_id, c)
        } else {
            self.reverse();
            ExecResult::Done
        };

        self.step(space);

        if !self.string {
            self.find_command(space);
        }

        result
    }

    /// Advances the `Ip`'s position by one step of its current [`Delta`].
    ///
    /// [`Delta`]: ../../data/struct.Delta.html
    pub fn step(&mut self, space: &Space) {
        self.position = space.new_position(self.position, self.delta);
    }

    /// Executes a single command, without moving the `Ip`'s afterwards.
    pub fn execute(&mut self, space: &mut Space, io: &mut IoContext, new_id: Value, command: char) -> ExecResult {
        match command {
            ' '         => panic!("attempted to execute ' '"),
            '!'         => self.negate(),
            '"'         => self.string_mode(),
            '#'         => self.trampoline(space),
            '$'         => self.discard(),
            '%'         => self.rem(),
            '&'         => self.input_decimal(io),
            '\''        => self.fetch_char(space),
            '('         => self.load_semantics(),
            ')'         => self.unload_semantics(),
            '*'         => self.mul(),
            '+'         => self.add(),
            ','         => self.output_char(io),
            '-'         => self.sub(),
            '.'         => self.output_decimal(io),
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
            ';'         => panic!("attempted to execute ';'"),
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
            'i'         => self.read_file(space, io),
            'j'         => self.jump(space),
            'k'         => return self.iterate(space, io, new_id),
            'l'         => self.reverse(),
            'm'         => self.reverse(),
            'n'         => self.clear(),
            'o'         => self.write_file(space, io),
            'p'         => self.put(space),
            'q'         => return ExecResult::Terminate(self.pop()),
            'r'         => self.reverse(),
            's'         => self.store_char(space),
            't'         => return ExecResult::AddIp(self.split(space, new_id)),
            'u'         => self.dig(),
            'v'         => self.go_south(),
            'w'         => self.compare(),
            'x'         => self.absolute_delta(),
            'y'         => self.get_sysinfo(space, io),
            'z'         => (),
            '{'         => self.begin_block(),
            '|'         => self.if_north_south(),
            '}'         => self.end_block(),
            '~'         => self.input_char(io),
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

    /// Pushes a string on the `Ip`'s [`StackStack`].
    ///
    /// Returns the number of cells that were pushed.
    ///
    /// [`StackStack`]: ../../data/stack/struct.StackStack.html
    pub fn push_string(&mut self, s: &str) -> usize {
        self.stacks.push_string(s)
    }

    /// Pops the top [`Value`] off the `Ip`'s [`StackStack`].
    ///
    /// [`Value`]: ../../data/struct.Value.html
    /// [`StackStack`]: ../../data/stack/struct.StackStack.html
    pub fn pop(&mut self) -> Value {
        self.stacks.pop()
    }

    /// Pops a string off the `Ip`'s [`StackStack`].
    ///
    /// [`StackStack`]: ../../data/stack/struct.StackStack.html
    pub fn pop_string(&mut self) -> Option<String> {
        self.stacks.pop_string()
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

        self.step(space);
        self.find_command(space);

        let ret = self.get_current(space);

        self.position = orig_position;

        ret
    }

    /// Skips all empty space in the path of the `Ip`.
    ///
    /// Similar to [`find_command`], except that semicolons are treated just
    /// like any other non-space character.
    ///
    /// This function will be used if the `Ip` is in string mode, in which each
    /// encountered character will be pushed to the [`StackStack`], but any
    /// contiguous sequence of spaces will be collapsed into one.
    ///
    /// [`find_command`]: #method.find_command
    /// [`StackStack`]: ../../data/stack/struct.StackStack.html
    pub fn skip_space(&mut self, space: &Space) {
        while self.get_current(space) == 32 {
            self.step(space);
        }
    }
}

/// The result of executing an instruction.
///
/// The variant indicates which further steps need to be taken by the caller to
/// update its information on the current program state.
pub enum ExecResult {
    /// Nothing needs to be done.
    Done,

    /// The instruction created a new [`Ip`] that should be added to the list.
    ///
    /// [`Ip`]: struct.Ip.html
    AddIp(Ip),

    /// The [`Ip`] executing the instruction got stopped and should be deleted.
    ///
    /// [`Ip`]: struct.Ip.html
    DeleteIp,

    /// The program should be terminated with the given [`Value`].
    ///
    /// [`Value`]: ../../data/type.Value.html
    Terminate(Value),
}
