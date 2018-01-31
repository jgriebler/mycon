//! A representation of a running Befunge-98 program.

pub mod ip;

use std::io;
use std::io::{Read, Write};

use data::{Value, Delta};
use data::space::Space;
use self::ip::Ip;

/// An instance of a Befunge-98 program.
///
/// This manages all data associated to the running program, like the
/// addressable space, all currently active instruction pointers and program
/// configuration.
pub struct Program {
    space: Space,
    ips: Vec<Ip>,
    current: usize,
    exit: Option<Value>,
    input: Box<Read>,
    input_buffer: String,
    output: Box<Write>,
}

impl Program {
    fn init(space: Space) -> Program {
        let input = Box::new(io::stdin());
        let output = Box::new(io::stdout());

        Program {
            space,
            ips: vec![Ip::new()],
            current: 0,
            exit: None,
            input,
            input_buffer: String::new(),
            output,
        }
    }

    /// Creates a new empty `Program`.
    pub fn new() -> Program {
        Program::init(Space::new())
    }

    /// Initializes a `Program` with the given source code.
    pub fn read(code: &str) -> Program {
        Program::init(Space::from(code))
    }

    /// Executes the current instruction of a single [`Ip`].
    ///
    /// The [`Ip`] will execute a single 'tick' as defined by the Funge-98
    /// specification and then advance its position up to the next command,
    /// skipping any intermediate spaces and areas delimited by semicolons and
    /// wrapping around to the other side of the program if it steps out of the
    /// program area.
    ///
    /// [`Ip`]: ip/struct.Ip.html
    pub fn step_single(&mut self) {
        let ip = &mut self.ips[self.current];

        match ip.get(&self.space) as u8 as char {
            ' ' => unreachable!("space"),
            '!' => if ip.pop() == 0 {
                ip.push(1);
            } else {
                ip.push(0);
            },
            '"' => ip.reverse(), // TODO implement
            '#' => ip.step(&self.space),
            '$' => { ip.pop(); },
            '%' => {
                let b = ip.pop();
                let a = ip.pop();
                ip.push(a % b);
            },
            '&' => ip.reverse(), // TODO implement
            '\'' => ip.reverse(), // TODO implement
            '(' => ip.reverse(), // TODO implement
            ')' => ip.reverse(), // TODO implement
            '*' => {
                let b = ip.pop();
                let a = ip.pop();
                ip.push(a * b);
            },
            '+' => {
                let b = ip.pop();
                let a = ip.pop();
                ip.push(a + b);
            },
            ',' => ip.reverse(), // TODO implement
            '-' => {
                let b = ip.pop();
                let a = ip.pop();
                ip.push(a - b);
            },
            '.' => ip.reverse(), // TODO implement
            '/' => {
                let b = ip.pop();
                let a = ip.pop();
                ip.push(if b == 0 { 0 } else { a / b });
            },
            '0' => ip.push(0),
            '1' => ip.push(1),
            '2' => ip.push(2),
            '3' => ip.push(3),
            '4' => ip.push(4),
            '5' => ip.push(5),
            '6' => ip.push(6),
            '7' => ip.push(7),
            '8' => ip.push(8),
            '9' => ip.push(9),
            ':' => ip.duplicate(),
            ';' => unreachable!("semicolon"),
            '<' => ip.set_delta(Delta { dx: -1, dy: 0 }),
            '=' => ip.reverse(), // TODO implement
            '>' => ip.set_delta(Delta { dx: 1, dy: 0 }),
            '?' => ip.randomize_delta(),
            '@' => ip.reverse(), // TODO implement
            'A' ... 'Z' => ip.reverse(), // TODO implement
            '[' => ip.turn_left(),
            '\\' => ip.swap(),
            ']' => ip.turn_right(),
            '^' => ip.set_delta(Delta { dx: 0, dy: -1 }),
            '_' => {
                let delta = if ip.pop() == 0 {
                    Delta { dx: 1, dy: 0 }
                } else {
                    Delta { dx: -1, dy: 0 }
                };
                ip.set_delta(delta);
            },
            '`' => {
                let b = ip.pop();
                let a = ip.pop();
                ip.push(if a > b { 1 } else { 0 });
            },
            'a' => ip.push(10),
            'b' => ip.push(11),
            'c' => ip.push(12),
            'd' => ip.push(13),
            'e' => ip.push(14),
            'f' => ip.push(15),
            'g' => ip.get_offset(&self.space),
            'h' => ip.reverse(),
            'i' => ip.reverse(), // TODO implement
            'j' => {
                let n = ip.pop();
                ip.jump(&self.space, n);
            },
            'k' => ip.reverse(), // TODO implement
            'l' => ip.reverse(),
            'm' => ip.reverse(),
            'n' => ip.clear(),
            'o' => ip.reverse(), // TODO implement
            'p' => ip.put_offset(&mut self.space),
            'q' => self.exit = Some(ip.pop()),
            'r' => ip.reverse(),
            's' => ip.reverse(), // TODO implement
            't' => ip.reverse(), // TODO implement
            'u' => ip.reverse(), // TODO implement
            'v' => ip.set_delta(Delta { dx: 0, dy: 1 }),
            'w' => {
                let b = ip.pop();
                let a = ip.pop();
                if a > b {
                    ip.turn_right();
                } else if a < b {
                    ip.turn_left();
                }
            },
            'x' => {
                let dy = ip.pop();
                let dx = ip.pop();
                ip.set_delta(Delta { dx, dy });
            },
            'y' => ip.reverse(), // TODO implement
            'z' => (),
            '{' => ip.reverse(), // TODO implement
            '|' => {
                let delta = if ip.pop() == 0 {
                    Delta { dx: 0, dy: 1 }
                } else {
                    Delta { dx: 0, dy: -1 }
                };
                ip.set_delta(delta);
            },
            '}' => ip.reverse(), // TODO implement
            '~' => ip.reverse(), // TODO implement
            _ => ip.reverse(),
        }

        ip.step(&self.space);
        ip.find_command(&self.space);
    }

    /// Executes the current instruction of every active [`Ip`].
    ///
    /// Similarly to [`step_single`], each [`Ip`] will be advanced to its next
    /// command.
    ///
    /// [`Ip`]: ip/struct.Ip.html
    pub fn step_all(&mut self) {
        let now = self.current;

        loop {
            self.step_single();
            self.current += 1;
            self.current %= self.ips.len();

            if self.current == now {
                break;
            }
        }
    }

    /// Runs the program to completion.
    ///
    /// Instructions will continuously be executed until the program encounters
    /// an error, all [`Ip`]s stop by encountering an `@`-instruction or the
    /// program is stopped with a `q`-instruction.
    ///
    /// [`Ip`]: ip/struct.Ip.html
    pub fn run(&mut self) -> Value {
        loop {
            self.step_all();

            if let Some(value) = self.exit {
                return value;
            }
        }
    }
}
