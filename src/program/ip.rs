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

//! A single instruction pointer in a running program.

mod instruction;

use crate::config::Trace;
use crate::data::{Value, Point, Delta};
use crate::data::space::Space;
use crate::data::stack::StackStack;
use super::Context;

/// An instruction pointer in a running program.
#[derive(Clone)]
pub(super) struct Ip {
    id: Value,
    position: Point,
    delta: Delta,
    storage: Point,
    stacks: StackStack,
    string: bool,
    saw_space: bool,
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
    pub(super) fn new() -> Ip {
        Ip {
            id: 0,
            position: Point { x: -1, y: 0 },
            delta: Delta { dx: 1, dy: 0 },
            storage: Point { x: 0, y: 0 },
            stacks: StackStack::new(),
            string: false,
            saw_space: false,
        }
    }

    /// Returns the [`Value`] at the `Ip`'s current position.
    ///
    /// [`Value`]: ../../data/struct.Value.html
    fn get_current(&self, space: &Space) -> Value {
        space.get(self.position)
    }

    /// Sets `Ip`'s identifier.
    pub(super) fn set_id(&mut self, id: Value) {
        self.id = id;
    }

    /// Executes a single command and moves the `Ip` to the next.
    pub(super) fn tick(&mut self, ctx: &mut Context) {
        self.step(&ctx.space);

        if !self.string {
            self.find_command(&ctx.space);
        } else if self.saw_space {
            self.skip_space(&ctx.space);
        }

        let v = self.get_current(&ctx.space);

        if self.string {
            if v == 34 {
                self.string = false;
            } else {
                self.push(v);
            }

            self.saw_space = v == 32;

            return;
        }

        if let Some(c) = std::char::from_u32(v as u32) {
            self.execute(ctx, c);
        } else {
            self.reflect();
        }
    }

    /// Advances the `Ip`'s position by one step of its current [`Delta`].
    ///
    /// [`Delta`]: ../../data/struct.Delta.html
    fn step(&mut self, space: &Space) {
        self.position = space.new_position(self.position, self.delta);
    }

    /// Executes a single command, without moving the `Ip`'s afterwards.
    fn execute(&mut self, ctx: &mut Context, command: char) {
        match command {
            ' '         => panic!("attempted to execute ' '"),
            '!'         => self.negate(),
            '"'         => self.string_mode(),
            '#'         => self.trampoline(ctx),
            '$'         => self.discard(),
            '%'         => self.rem(),
            '&'         => self.input_decimal(ctx),
            '\''        => self.fetch_char(ctx),
            '('         => self.load_semantics(),
            ')'         => self.unload_semantics(),
            '*'         => self.mul(),
            '+'         => self.add(),
            ','         => self.output_char(ctx),
            '-'         => self.sub(),
            '.'         => self.output_decimal(ctx),
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
            '='         => self.system_execute(ctx),
            '>'         => self.go_east(),
            '?'         => self.randomize_delta(),
            '@'         => self.stop(ctx),
            'A' ... 'Z' => self.reflect(), // TODO implement
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
            'g'         => self.get(ctx),
            'h'         => self.reflect(),
            'i'         => self.read_file(ctx),
            'j'         => self.jump(ctx),
            'k'         => self.iterate(ctx),
            'l'         => self.reflect(),
            'm'         => self.reflect(),
            'n'         => self.clear(),
            'o'         => self.write_file(ctx),
            'p'         => self.put(ctx),
            'q'         => self.terminate(ctx),
            'r'         => self.reflect(),
            's'         => self.store_char(ctx),
            't'         => self.split(ctx),
            'u'         => self.dig(),
            'v'         => self.go_south(),
            'w'         => self.compare(),
            'x'         => self.absolute_delta(),
            'y'         => self.get_sysinfo(ctx),
            'z'         => (),
            '{'         => self.begin_block(),
            '|'         => self.if_north_south(),
            '}'         => self.end_block(),
            '~'         => self.input_char(ctx),
            _           => self.reflect(),
        }

        ctx.config.do_trace(Trace::new(self.id, command, self.position, &self.stacks));
    }

    /// Sets the `Ip`'s [`Delta`] to a new value.
    ///
    /// [`Delta`]: ../../data/struct.Delta.html
    fn set_delta(&mut self, delta: Delta) {
        self.delta = delta;
    }

    /// Pushes a [`Value`] to the `Ip`'s [`StackStack`].
    ///
    /// [`Value`]: ../../data/struct.Value.html
    /// [`StackStack`]: ../../data/stack/struct.StackStack.html
    fn push(&mut self, value: Value) {
        self.stacks.push(value);
    }

    /// Pushes a string on the `Ip`'s [`StackStack`].
    ///
    /// Returns the number of cells that were pushed.
    ///
    /// [`StackStack`]: ../../data/stack/struct.StackStack.html
    fn push_string(&mut self, s: &str) -> usize {
        self.stacks.push_string(s)
    }

    /// Pops the top [`Value`] off the `Ip`'s [`StackStack`].
    ///
    /// [`Value`]: ../../data/struct.Value.html
    /// [`StackStack`]: ../../data/stack/struct.StackStack.html
    fn pop(&mut self) -> Value {
        self.stacks.pop()
    }

    /// Pops a string off the `Ip`'s [`StackStack`].
    ///
    /// [`StackStack`]: ../../data/stack/struct.StackStack.html
    fn pop_string(&mut self) -> Option<String> {
        self.stacks.pop_string()
    }

    /// Advances the `Ip`'s position to the next command in its path.
    ///
    /// Any intervening empty space or areas delimited by semicolons will be
    /// skipped.
    pub(super) fn find_command(&mut self, space: &Space) {
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
    fn peek_command(&mut self, space: &Space) -> Value {
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
    fn skip_space(&mut self, space: &Space) {
        while self.get_current(space) == 32 {
            self.step(space);
        }
    }
}
