//! Helper types for storing program configuration.

use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read, Write};
use std::iter;

use data::Value;

/// Tracks information on how the program interacts with its environment.
///
/// An `IoContext` keeps track of where input should be read, where output
/// should be written, what files the program may access and how to locate them,
/// and how to react when it tries to execute a shell command.
pub struct IoContext {
    input: Box<BufRead>,
    input_buffer: String,
    output: Box<Write>,
}

impl IoContext {
    /// Creates a new `IoContext` referencing the standard input and output.
    pub fn stdio() -> IoContext {
        IoContext {
            input: Box::new(BufReader::new(io::stdin())),
            input_buffer: String::new(),
            output: Box::new(io::stdout()),
        }
    }

    /// Tries to write a number to the `IoContext`'s output stream.
    ///
    /// Returns `true` if it succeeded, `false` otherwise.
    pub fn write_decimal(&mut self, n: i32) -> bool {
        write!(self.output, "{} ", n).is_ok()
    }

    /// Tries to write a `char` to the `IoContext`'s output stream.
    ///
    /// Returns `true` if it succeeded, `false` otherwise.
    pub fn write_char(&mut self, c: char) -> bool {
        write!(self.output, "{}", c).is_ok()
    }

    /// Tries to read a number from the `IoContext`'s input stream.
    ///
    /// Returns `Some` read number if it succeeded, `None` otherwise.
    pub fn read_decimal(&mut self) -> Option<i32> {
        if self.output.flush().is_err() {
            return None;
        }

        if self.input_buffer.is_empty() {
            if self.input.read_line(&mut self.input_buffer).is_err() {
                return None;
            }
        }

        let mut found = false;
        let mut ret = 0;
        let mut stop = 0;
        for (i, b) in self.input_buffer.bytes().enumerate() {
            if (b as char).is_digit(10) {
                found = true;
                ret *= 10;
                ret += (b - '0' as u8) as i32;
            } else if found {
                if b == '\n' as u8 {
                    stop = i + 1;
                } else {
                    stop = i;
                }
            }
        }

        self.input_buffer.drain(0..stop);
        
        Some(ret)
    }

    /// Tries to read a `char` from the `IoContext`'s input stream.
    ///
    /// Returns `Some` read `char` if it succeeded, `None` otherwise.
    pub fn read_char(&mut self) -> Option<char> {
        if self.output.flush().is_err() {
            return None;
        }

        if self.input_buffer.is_empty() {
            if self.input.read_line(&mut self.input_buffer).is_err() {
                return None;
            }
        }

        let c = self.input_buffer.chars().nth(0).unwrap();
        let mut stop = 1;

        while !self.input_buffer.is_char_boundary(stop) {
            stop += 1;
        }

        self.input_buffer.drain(0..stop);

        Some(c)
    }

    /// Tries to write the given string to a file.
    ///
    /// Returns `true` if it succeeded, `false` otherwise.
    pub fn write_file(&self, path: &str, data: &str) -> bool {
        let mut f = match File::create(path) {
            Ok(f)  => f,
            Err(_) => return false,
        };

        f.write_all(data.as_bytes()).is_ok()
    }

    /// Tries to read from a file.
    ///
    /// Returns `Some` read string, or `None` if it failed.
    pub fn read_file(&self, path: &str) -> Option<String> {
        let mut f = match File::open(path) {
            Ok(f)  => f,
            Err(_) => return None,
        };

        let mut s = String::new();

        if f.read_to_string(&mut s).is_err() {
            None
        } else {
            Some(s)
        }
    }

    /// Returns flags containing information about the interpreter.
    ///
    /// The flags are in the format returned by the 'y'-instruction to a running
    /// Befunge-98 program.
    pub fn flags(&self) -> Value {
        0b00111
    }

    /// Returns a value indicating the behavior of the '='-instruction.
    pub fn operating_paradigm(&self) -> Value {
        0
    }

    /// Returns an iterator over the command-line arguments of the program.
    pub fn cmd_args(&self) -> iter::Rev<env::Args> {
        env::args().rev()
    }

    /// Returns an iterator over the environment variables.
    pub fn env_vars(&self) -> env::Vars {
        env::vars()
    }
}
