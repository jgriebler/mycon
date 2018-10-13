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

//! An interpreter and a library for the esoteric programming language [Befunge-98].
//!
//! [Befunge-98]: https://esolangs.org/wiki/Funge-98

#![warn(missing_docs)]

extern crate chrono;
extern crate rand;

mod config;
mod data;
mod program;

pub use config::Config;
pub use config::FileView;
pub use config::ExecAction;
pub use program::Program;
