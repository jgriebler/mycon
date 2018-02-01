//! Helper types for storing program configuration.

use std::io;
use std::io::{BufRead, BufReader, Write};

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
}
