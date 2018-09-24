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
pub struct Program<'env> {
    context: Context<'env>,
    ip_meta: IpMeta,
}

impl<'env> Program<'env> {
    fn init(space: Space, env: Environment<'env>) -> Self {
        let mut ip = Ip::new();
        ip.find_command(&space);

        let context = Context {
            space,
            env,
            control: Control(Vec::new()),
        };

        let ip_meta = IpMeta {
            ips: vec![ip],
            current: 0,
            exit: None,
            new_id: 1,
        };

        Program {
            context,
            ip_meta,
        }
    }

    /// Creates a new empty `Program`.
    pub fn new() -> Self {
        Program::init(Space::new(), Environment::new())
    }

    /// Initializes a `Program` with the given source code.
    pub fn read(code: &str) -> Self {
        Program::init(Space::from(code), Environment::new())
    }

    pub fn read_with_io<R, W>(code: &str, input: &'env mut R, output: &'env mut W) -> Self
        where R: BufRead,
              W: Write,
    {
        Program::init(Space::from(code), Environment::new().input(input).output(output))
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
        self.ip_meta.ips[self.ip_meta.current].tick(&mut self.context);
        self.context.commit_changes(&mut self.ip_meta);
    }

    /// Executes the current instruction of every active [`Ip`].
    ///
    /// Similarly to [`step_single`], each [`Ip`] will be advanced to its next
    /// command.
    ///
    /// [`Ip`]: ip/struct.Ip.html
    /// [`step_single`]: #method.step_single
    pub fn step_all(&mut self) {
        let now = self.ip_meta.current;

        loop {
            self.step_single();

            if self.ip_meta.current == now || self.ip_meta.exit.is_some() {
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

            if let Some(value) = self.ip_meta.exit {
                return value;
            }
        }
    }
}

/// A structure to track changes done to the control state of a [`Program`] by
/// an [`Ip`].
///
/// Methods are provided to add a new [`Ip`], delete the current [`Ip`] and to
/// terminate the [`Program`].
///
/// [`Ip`]: ip/struct.Ip.html
/// [`Program`]: struct.Program.html
pub struct Control(Vec<ExecResult>);

impl Control {
    /// Adds an [`Ip`] to the list.
    ///
    /// This method only takes note that this operation should be performed, the
    /// [`Program`] is responsible for actually commiting this change.
    ///
    /// [`Ip`]: ip/struct.Ip.html
    /// [`Program`]: struct.Program.html
    pub fn add_ip(&mut self, ip: Ip) {
        self.0.push(ExecResult::AddIp(ip));
    }

    /// Deletes the current [`Ip`] from the list.
    ///
    /// This method only takes note that this operation should be performed, the
    /// [`Program`] is responsible for actually commiting this change.
    ///
    /// [`Ip`]: ip/struct.Ip.html
    /// [`Program`]: struct.Program.html
    pub fn delete_ip(&mut self) {
        self.0.push(ExecResult::DeleteIp);
    }

    /// Terminates the program, using the given [`Value`] as the exit status.
    ///
    /// This method only takes note that this operation should be performed, the
    /// [`Program`] is responsible for actually commiting this change.
    ///
    /// [`Value`]: ../data/type.Value.html
    /// [`Program`]: struct.Program.html
    pub fn terminate(&mut self, v: Value) {
        self.0.push(ExecResult::Terminate(v));
    }
}

/// The state of the [`Program`] that can be manipulated by the [`Ip`].
///
/// [`Program`]: struct.Program.html
/// [`Ip`]: ip/struct.Ip.html
pub struct Context<'env> {
    control: Control,
    space: Space,
    env: Environment<'env>,
}

impl<'env> Context<'env> {
    /// Commits all changes registered on this `Context`.
    ///
    /// This method needs to be called exactly once after an instruction has
    /// been executed.
    pub fn commit_changes(&mut self, ip_meta: &mut IpMeta) {
        let mut offset = 1;

        for result in self.control.0.drain(..) {
            match result {
                ExecResult::AddIp(mut new) => {
                    new.set_id(ip_meta.new_id);
                    ip_meta.new_id += 1;
                    ip_meta.ips.insert(ip_meta.current, new);
                    offset += 1;
                },
                ExecResult::DeleteIp => {
                    ip_meta.ips.remove(ip_meta.current);
                    offset -= 1;
                },
                ExecResult::Terminate(v) => {
                    ip_meta.exit = Some(v);
                },
            }
        }

        if ip_meta.ips.is_empty() && ip_meta.exit.is_none() {
            ip_meta.exit = Some(0);
            return;
        }

        ip_meta.current += (ip_meta.ips.len() as isize + offset) as usize;
        ip_meta.current %= ip_meta.ips.len();
    }
}

pub struct IpMeta {
    ips: Vec<Ip>,
    current: usize,
    exit: Option<Value>,
    new_id: Value,
}

enum ExecResult {
    AddIp(Ip),
    DeleteIp,
    Terminate(Value),
}
