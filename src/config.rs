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

//! Helper types for storing program configuration.

use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read, Write};
use std::process::Command;

use data::Value;

enum Input<'a> {
    Owned(Box<BufRead>),
    Borrowed(&'a mut BufRead),
}

impl<'a> Read for Input<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Input::Owned(r)    => r.read(buf),
            Input::Borrowed(r) => r.read(buf),
        }
    }
}

impl<'a> BufRead for Input<'a> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        match self {
            Input::Owned(r)    => r.fill_buf(),
            Input::Borrowed(r) => r.fill_buf(),
        }
    }

    fn consume(&mut self, amt: usize) {
        match self {
            Input::Owned(r)    => r.consume(amt),
            Input::Borrowed(r) => r.consume(amt),
        }
    }
}

enum Output<'a> {
    Owned(Box<Write>),
    Borrowed(&'a mut Write),
}

impl<'a> Write for Output<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Output::Owned(w)    => w.write(buf),
            Output::Borrowed(w) => w.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Output::Owned(w)    => w.flush(),
            Output::Borrowed(w) => w.flush(),
        }
    }
}

/// Specifies how to react when the program tries to access a file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileView {
    /// Gives complete access to the real filesystem.
    Real,
    /// Denies any file access. The 'i' and 'o' instructions will fail and the
    /// interpreter will report that they are unsupported.
    Deny,
}

/// Specifies what action to take when the program attempts to execute a shell
/// command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExecAction {
    /// Allows any commands issued by the program to be executed by the system
    /// shell.
    Real,
    /// Denies the ability to execute commands. The '=' instruction will fail
    /// and the interpreter will report that it is unsupported.
    Deny,
}

/// Tracks information on how the program interacts with its environment.
///
/// An `Environment` keeps track of where input should be read, where output
/// should be written, what files the program may access and how to locate them,
/// and how to react when it tries to execute a shell command.
///
/// This API will soon be overhauled to provide a cleaner and more expressive
/// interface.
pub struct Environment<'env> {
    input: Input<'env>,
    input_buffer: String,
    output: Output<'env>,
    file_view: FileView,
    exec_action: ExecAction,
}

impl<'env> Environment<'env> {
    /// Creates a new `Environment` referencing the standard input and output.
    ///
    /// The `Environment` will give the program full access to the file system
    /// and allow it to execute shell commands.
    pub fn new() -> Self {
        Environment {
            input: Input::Owned(Box::new(BufReader::new(io::stdin()))),
            input_buffer: String::new(),
            output: Output::Owned(Box::new(io::stdout())),
            file_view: FileView::Real,
            exec_action: ExecAction::Real,
        }
    }

    /// Sets the input stream of the `Environment`.
    pub fn input(self, input: &'env mut impl BufRead) -> Self {
        Self {
            input: Input::Borrowed(input),
            ..self
        }
    }

    /// Sets the output stream of the `Environment`.
    pub fn output(self, output: &'env mut impl Write) -> Self {
        Self {
            output: Output::Borrowed(output),
            ..self
        }
    }

    /// Sets the [`FileView`] of the `Environment`.
    ///
    /// [`FileView`]: enum.FileView.html
    pub fn file_view(self, file_view: FileView) -> Self {
        Self {
            file_view,
            ..self
        }
    }

    /// Sets the [`ExecAction`] of the `Environment`.
    ///
    /// [`ExecAction`]: enum.ExecAction.html
    pub fn exec_action(self, exec_action: ExecAction) -> Self {
        Self {
            exec_action,
            ..self
        }
    }

    /// Tries to write a number to the `Environment`'s output stream.
    ///
    /// Returns `true` if it succeeded, `false` otherwise.
    pub(crate) fn write_decimal(&mut self, n: i32) -> bool {
        write!(self.output, "{} ", n).is_ok()
    }

    /// Tries to write a `char` to the `Environment`'s output stream.
    ///
    /// Returns `true` if it succeeded, `false` otherwise.
    pub(crate) fn write_char(&mut self, c: char) -> bool {
        write!(self.output, "{}", c).is_ok()
    }

    /// Tries to read a number from the `Environment`'s input stream.
    ///
    /// Returns `Some` read number if it succeeded, `None` otherwise.
    pub(crate) fn read_decimal(&mut self) -> Option<i32> {
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

        // TODO Should this return 0 if no digits were encountered? That seems to be what it's
        // doing right now. Consult the specification about this.
        Some(ret)
    }

    /// Tries to read a `char` from the `Environment`'s input stream.
    ///
    /// Returns `Some` read `char` if it succeeded, `None` otherwise.
    pub(crate) fn read_char(&mut self) -> Option<char> {
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
    pub(crate) fn write_file(&self, path: &str, data: &str) -> bool {
        match self.file_view {
            FileView::Real => (),
            FileView::Deny  => return false,
        }

        let mut f = match File::create(path) {
            Ok(f)  => f,
            Err(_) => return false,
        };

        f.write_all(data.as_bytes()).is_ok()
    }

    /// Tries to read from a file.
    ///
    /// Returns `Some` read string, or `None` if it failed.
    pub(crate) fn read_file(&self, path: &str) -> Option<String> {
        match self.file_view {
            FileView::Real => (),
            FileView::Deny  => return None,
        }

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

    /// Takes a string and tries to execute it with `sh`.
    ///
    /// Returns `Some` [`Value`] with `sh`'s exit code if it was able to obtain
    /// it, and `None` otherwise.
    ///
    /// If a [`Value`] is returned, the exit code can (in general) not be used
    /// to determine whether an error was raised by `sh` trying to execute the
    /// given command, or by the command itself.
    ///
    /// Also, a return of `None` can mean that the attempt to execute `sh`
    /// failed, that `sh` was terminated by a signal or that this
    /// `Environment`'s settings don't allow command execution.
    ///
    /// [`Value`]: ../../data/type.Value.html
    pub(crate) fn execute(&self, cmd: &str) -> Option<Value> {
        if self.exec_action != ExecAction::Deny {
            match Command::new("sh").args(&["-c", cmd]).status() {
                Ok(st) => st.code(),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    /// Returns flags containing information about functionality available to
    /// the program.
    ///
    /// The flags are in the format returned by the `y`-instruction to a running
    /// Befunge-98 program.
    pub(crate) fn flags(&self) -> Value {
        // 't' is always supported.
        // TODO Should this be configurable?
        let mut flags = 1;

        if self.file_view != FileView::Deny {
            // 'i' and 'o' are supported.
            flags |= 0x6;
        }

        if self.exec_action != ExecAction::Deny {
            // '=' is supported.
            flags |= 0x8;
        }

        flags
    }

    /// Returns a value indicating the behavior of the `=`-instruction.
    pub(crate) fn operating_paradigm(&self) -> Value {
        if self.exec_action != ExecAction::Deny {
            2
        } else {
            0
        }
    }

    /// Returns an iterator over the command-line arguments of the program.
    pub(crate) fn cmd_args(&self) -> impl Iterator<Item = String> {
        env::args().rev()
    }

    /// Returns an iterator over the environment variables.
    pub(crate) fn env_vars(&self) -> impl Iterator<Item = (String, String)> {
        env::vars()
    }
}