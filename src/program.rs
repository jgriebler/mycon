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

//! A representation of a running Befunge-98 program.

mod ip;

use crate::config::Config;
use crate::data::Value;
use crate::data::space::Space;
use self::ip::Ip;

/// An instance of a Befunge-98 program.
///
/// This manages all data associated to the running program, like the
/// addressable space, all currently active instruction pointers and program
/// configuration.
pub struct Program<'env> {
    context: Context<'env>,
    ip_data: IpData,
}

impl<'env> Program<'env> {
    fn init(space: Space, config: Config<'env>) -> Self {
        let ip = Ip::new();

        let context = Context {
            space,
            config,
            control: Control(Vec::new()),
        };

        let ip_data = IpData {
            ips: vec![ip],
            current: 0,
            exit: None,
            new_id: 1,
        };

        Program {
            context,
            ip_data,
        }
    }

    /// Creates a new empty `Program`.
    pub fn new() -> Self {
        Program::init(Space::new(), Config::new())
    }

    /// Initializes a `Program` with the given source code.
    pub fn read(code: &str) -> Self {
        Program::init(Space::read(code), Config::new())
    }

    /// Sets the `Program`'s [`Config`].
    ///
    /// [`Config`]: struct.Config.html
    pub fn config(mut self, config: Config<'env>) -> Self {
        self.context.config = config;
        self
    }

    /// Returns the exit status if the `Program` has finished.
    ///
    /// If the `Program` is not yet done, `None` is returned.
    pub fn exit_status(&self) -> Option<Value> {
        self.ip_data.exit
    }

    /// Executes the current instruction of a single instruction pointer.
    ///
    /// The IP will execute a single 'tick' as defined by the Funge-98
    /// specification and then advance its position up to the next command,
    /// skipping any intermediate spaces and areas delimited by semicolons and
    /// wrapping around to the other side of the program if it steps out of the
    /// program area.
    pub fn step_single(&mut self) {
        self.ip_data.ips[self.ip_data.current].tick(&mut self.context);
        self.context.commit_changes(&mut self.ip_data);
    }

    /// Executes the current instruction of every active instruction pointer.
    ///
    /// Similarly to [`step_single`], each IP will be advanced to its next
    /// command.
    ///
    /// [`step_single`]: #method.step_single
    pub fn step_all(&mut self) {
        let now = self.ip_data.current;

        loop {
            self.step_single();

            if self.ip_data.current == now || self.ip_data.exit.is_some() {
                break;
            }
        }
    }

    /// Runs the program to completion.
    ///
    /// Instructions will continuously be executed until the program encounters
    /// an error, all instruction pointers stop by encountering an
    /// `@` instruction or the program is stopped with a `q` instruction.
    pub fn run(&mut self) -> Value {
        loop {
            self.step_all();

            if let Some(value) = self.ip_data.exit {
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
struct Control(Vec<ExecResult>);

impl Control {
    /// Adds an [`Ip`] to the list.
    ///
    /// This method only takes note that this operation should be performed, the
    /// [`Program`] is responsible for actually commiting this change.
    ///
    /// [`Ip`]: ip/struct.Ip.html
    /// [`Program`]: struct.Program.html
    fn add_ip(&mut self, ip: Ip) {
        self.0.push(ExecResult::AddIp(ip));
    }

    /// Deletes the current [`Ip`] from the list.
    ///
    /// This method only takes note that this operation should be performed, the
    /// [`Program`] is responsible for actually commiting this change.
    ///
    /// [`Ip`]: ip/struct.Ip.html
    /// [`Program`]: struct.Program.html
    fn delete_ip(&mut self) {
        self.0.push(ExecResult::DeleteIp);
    }

    /// Terminates the program, using the given [`Value`] as the exit status.
    ///
    /// This method only takes note that this operation should be performed, the
    /// [`Program`] is responsible for actually commiting this change.
    ///
    /// [`Value`]: ../data/type.Value.html
    /// [`Program`]: struct.Program.html
    fn terminate(&mut self, v: Value) {
        self.0.push(ExecResult::Terminate(v));
    }
}

/// The state of the [`Program`] that can be manipulated by the [`Ip`].
///
/// [`Program`]: struct.Program.html
/// [`Ip`]: ip/struct.Ip.html
pub(crate) struct Context<'env> {
    control: Control,
    space: Space,
    config: Config<'env>,
}

impl<'env> Context<'env> {
    /// Commits all changes registered on this `Context`.
    ///
    /// This method needs to be called exactly once after an instruction has
    /// been executed.
    fn commit_changes(&mut self, ip_data: &mut IpData) {
        let mut offset = 1;

        for result in self.control.0.drain(..) {
            match result {
                ExecResult::AddIp(mut new) => {
                    new.set_id(ip_data.new_id);
                    ip_data.new_id += 1;
                    ip_data.ips.insert(ip_data.current, new);
                    offset += 1;
                },
                ExecResult::DeleteIp => {
                    ip_data.ips.remove(ip_data.current);
                    offset -= 1;
                },
                ExecResult::Terminate(v) => {
                    ip_data.exit = Some(v);
                },
            }
        }

        if ip_data.ips.is_empty() && ip_data.exit.is_none() {
            ip_data.exit = Some(0);
            return;
        }

        ip_data.current += (ip_data.ips.len() as isize + offset) as usize;
        ip_data.current %= ip_data.ips.len();
    }
}

/// A structure that tracks the active [`Ip`]s of a [`Program`].
///
/// [`Program`]: struct.Program.html
/// [`Ip`]: ip/struct.Ip.html
struct IpData {
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
