//! A representation of a running Befunge-98 program.

pub mod config;
pub mod ip;

use std::io::{BufRead, Write};

use data::Value;
use data::space::Space;
use self::config::Environment;
use self::ip::Ip;

/// An instance of a Befunge-98 program.
///
/// This manages all data associated to the running program, like the
/// addressable space, all currently active instruction pointers and program
/// configuration.
pub struct Program<'a> {
    context: Context<'a>,
    ips: Vec<Ip>,
    current: usize,
    exit: Option<Value>,
    new_id: Value,
}

impl<'a> Program<'a> {
    fn init(space: Space, env: Environment<'a>) -> Program<'a> {
        let mut ip = Ip::new();
        ip.find_command(&space);

        let context = Context {
            space,
            env,
            changes: Vec::new(),
        };

        Program {
            context,
            ips: vec![ip],
            current: 0,
            exit: None,
            new_id: 1,
        }
    }

    /// Creates a new empty `Program`.
    pub fn new() -> Program<'static> {
        Program::init(Space::new(), Environment::stdio())
    }

    /// Initializes a `Program` with the given source code.
    pub fn read(code: &str) -> Program<'static> {
        Program::init(Space::from(code), Environment::stdio())
    }

    pub fn read_with_io<R, W>(code: &str, input: &'a mut R, output: &'a mut W) -> Program<'a>
        where R: BufRead,
              W: Write,
    {
        Program::init(Space::from(code), Environment::with_io(input, output))
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
        self.ips[self.current].tick(&mut self.context);
        let mut offset = 1;

        for result in self.context.changes.drain(..) {
            match result {
                ExecResult::AddIp(mut new) => {
                    new.set_id(self.new_id);
                    self.new_id += 1;
                    self.ips.insert(self.current, new);
                    offset += 1;
                },
                ExecResult::DeleteIp       => {
                    self.ips.remove(self.current);
                    offset -= 1;
                },
                ExecResult::Terminate(v)   => {
                    self.exit = Some(v);
                },
            }
        }

        if self.ips.is_empty() {
            self.exit = Some(0);
            return;
        }

        self.current += (self.ips.len() as isize + offset) as usize;
        self.current %= self.ips.len();
    }

    /// Executes the current instruction of every active [`Ip`].
    ///
    /// Similarly to [`step_single`], each [`Ip`] will be advanced to its next
    /// command.
    ///
    /// [`Ip`]: ip/struct.Ip.html
    /// [`step_single`]: #method.step_single
    pub fn step_all(&mut self) {
        let now = self.current;

        loop {
            self.step_single();

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

/// The state of the [`Program`] that can be manipulated by the [`Ip`].
///
/// [`Program`]: struct.Program.html
/// [`Ip`]: ip/struct.Ip.html
pub struct Context<'a> {
    space: Space,
    env: Environment<'a>,
    changes: Vec<ExecResult>,
}

impl<'a> Context<'a> {
    /// Returns a reference to the [`Space`] containing the [`Ip`].
    ///
    /// [`Space`]: ../data/space/struct.Space.html
    /// [`Ip`]: ip/struct.Ip.html
    pub fn space(&self) -> &Space {
        &self.space
    }

    /// Returns a mutable reference to the [`Space`] containing the [`Ip`].
    ///
    /// [`Space`]: ../data/space/struct.Space.html
    /// [`Ip`]: ip/struct.Ip.html
    pub fn space_mut(&mut self) -> &mut Space {
        &mut self.space
    }

    /// Returns a reference to the [`Environment`].
    ///
    /// [`Environment`]: config/struct.Environment.html
    pub fn env(&self) -> &Environment<'a> {
        &self.env
    }

    /// Returns a mutable reference to the [`Environment`].
    ///
    /// [`Environment`]: config/struct.Environment.html
    pub fn env_mut(&mut self) -> &mut Environment<'a> {
        &mut self.env
    }

    /// Adds an [`Ip`] to the list.
    ///
    /// This method only takes note that this operation should be performed, the
    /// [`Program`] is responsible for actually commiting this change.
    ///
    /// [`Ip`]: ip/struct.Ip.html
    /// [`Program`]: struct.Program.html
    pub fn add_ip(&mut self, ip: Ip) {
        self.changes.push(ExecResult::AddIp(ip));
    }

    /// Deletes the current [`Ip`] from the list.
    ///
    /// This method only takes note that this operation should be performed, the
    /// [`Program`] is responsible for actually commiting this change.
    ///
    /// [`Ip`]: ip/struct.Ip.html
    /// [`Program`]: struct.Program.html
    pub fn delete_ip(&mut self) {
        self.changes.push(ExecResult::DeleteIp);
    }

    /// Terminates the program, using the given [`Value`] as the exit status.
    ///
    /// This method only takes note that this operation should be performed, the
    /// [`Program`] is responsible for actually commiting this change.
    ///
    /// [`Value`]: ../data/type.Value.html
    /// [`Program`]: struct.Program.html
    pub fn terminate(&mut self, v: Value) {
        self.changes.push(ExecResult::Terminate(v));
    }
}

enum ExecResult {
    AddIp(Ip),
    DeleteIp,
    Terminate(Value),
}
