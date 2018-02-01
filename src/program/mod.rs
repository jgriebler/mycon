//! A representation of a running Befunge-98 program.

pub mod config;
pub mod ip;

use data::Value;
use data::space::Space;
use self::config::IoContext;
use self::ip::{Ip, ExecResult};

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
    io: IoContext,
}

impl Program {
    fn init(space: Space) -> Program {
        let mut ip = Ip::new();
        ip.find_command(&space);

        Program {
            space,
            ips: vec![ip],
            current: 0,
            exit: None,
            io: IoContext::stdio(),
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
        let result = self.ips[self.current].tick(&mut self.space, &mut self.io);

        let offset = match result {
            ExecResult::Done         => 1,
            ExecResult::AddIp(new)   => { self.ips.insert(self.current, new); 2 },
            ExecResult::DeleteIp     => { self.ips.remove(self.current); 0 },
            ExecResult::Terminate(v) => { self.exit = Some(v); 1 },
        };

        if self.ips.is_empty() {
            self.exit = Some(0);
            return;
        }

        self.current += offset;
        self.current %= self.ips.len();
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
