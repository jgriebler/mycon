//! Helper types for storing program configuration.

use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read, Write};
use std::iter;
use std::process::Command;

use data::Value;

enum Input<'a> {
    Owned(Box<BufRead>),
    Borrowed(&'a mut BufRead),
}

impl<'a> Read for Input<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            &mut Input::Owned(ref mut r)    => r.read(buf),
            &mut Input::Borrowed(ref mut r) => r.read(buf),
        }
    }
}

impl<'a> BufRead for Input<'a> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        match self {
            &mut Input::Owned(ref mut r)    => r.fill_buf(),
            &mut Input::Borrowed(ref mut r) => r.fill_buf(),
        }
    }

    fn consume(&mut self, amt: usize) {
        match self {
            &mut Input::Owned(ref mut r)    => r.consume(amt),
            &mut Input::Borrowed(ref mut r) => r.consume(amt),
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
            &mut Output::Owned(ref mut w)    => w.write(buf),
            &mut Output::Borrowed(ref mut w) => w.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            &mut Output::Owned(ref mut w)    => w.flush(),
            &mut Output::Borrowed(ref mut w) => w.flush(),
        }
    }
}

#[derive(PartialEq, Eq)]
enum FileAccess {
    Allow,
    Deny,
}

#[derive(PartialEq, Eq)]
enum ExecAccess {
    Allow,
    Deny,
}

// TODO Clean up this API.
/// Tracks information on how the program interacts with its environment.
///
/// An `Environment` keeps track of where input should be read, where output
/// should be written, what files the program may access and how to locate them,
/// and how to react when it tries to execute a shell command.
///
/// This API will soon be overhauled to provide a cleaner and more expressive
/// interface.
pub struct Environment<'a> {
    input: Input<'a>,
    input_buffer: String,
    output: Output<'a>,
    file_access: FileAccess,
    exec_access: ExecAccess,
}

impl<'a> Environment<'a> {
    /// Creates a new `Environment` referencing the standard input and output.
    pub fn stdio() -> Environment<'static> {
        Environment {
            input: Input::Owned(Box::new(BufReader::new(io::stdin()))),
            input_buffer: String::new(),
            output: Output::Owned(Box::new(io::stdout())),
            file_access: FileAccess::Allow,
            exec_access: ExecAccess::Allow,
        }
    }

    pub fn with_io<R, W>(input: &'a mut R, output: &'a mut W) -> Environment<'a>
        where R: BufRead,
              W: Write,
    {
        Environment {
            input: Input::Borrowed(input),
            input_buffer: String::new(),
            output: Output::Borrowed(output),
            file_access: FileAccess::Allow,
            exec_access: ExecAccess::Allow,
        }
    }

    /// Sets the input stream of the `Environment`.
    pub fn set_input<R: BufRead + 'static>(&mut self, input: R) {
        self.input = Input::Owned(Box::new(input));
    }

    /// Sets the output stream of the `Environment`.
    pub fn set_output<W: Write + 'static>(&mut self, output: W) {
        self.output = Output::Owned(Box::new(output));
    }

    pub fn input_from<R: BufRead>(&mut self, input: &'a mut R) {
        self.input = Input::Borrowed(input);
    }

    pub fn output_to<W: Write>(&mut self, output: &'a mut W) {
        self.output = Output::Borrowed(output);
    }

    /// Tries to write a number to the `Environment`'s output stream.
    ///
    /// Returns `true` if it succeeded, `false` otherwise.
    pub fn write_decimal(&mut self, n: i32) -> bool {
        write!(self.output, "{} ", n).is_ok()
    }

    /// Tries to write a `char` to the `Environment`'s output stream.
    ///
    /// Returns `true` if it succeeded, `false` otherwise.
    pub fn write_char(&mut self, c: char) -> bool {
        write!(self.output, "{}", c).is_ok()
    }

    /// Tries to read a number from the `Environment`'s input stream.
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

    /// Tries to read a `char` from the `Environment`'s input stream.
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
        match self.file_access {
            FileAccess::Allow => (),
            FileAccess::Deny  => return false,
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
    pub fn read_file(&self, path: &str) -> Option<String> {
        match self.file_access {
            FileAccess::Allow => (),
            FileAccess::Deny  => return None,
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
    pub fn execute(&self, cmd: &str) -> Option<Value> {
        if self.exec_access != ExecAccess::Deny {
            match Command::new("sh").args(&["-c", cmd]).status() {
                Ok(st) => st.code(),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    /// Returns flags containing information about the interpreter.
    ///
    /// The flags are in the format returned by the `y`-instruction to a running
    /// Befunge-98 program.
    pub fn flags(&self) -> Value {
        let mut flags = 1;

        if self.file_access != FileAccess::Deny {
            flags |= 0x6;
        }

        if self.exec_access != ExecAccess::Deny {
            flags |= 0x8;
        }

        flags
    }

    /// Returns a value indicating the behavior of the `=`-instruction.
    pub fn operating_paradigm(&self) -> Value {
        if self.exec_access != ExecAccess::Deny {
            2
        } else {
            0
        }
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
