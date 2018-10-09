//! An interpreter and a library for the esoteric programming language [Befunge-98].
//!
//! [Befunge-98]: https://esolangs.org/wiki/Funge-98

#![warn(missing_docs)]

extern crate chrono;
extern crate rand;

mod config;
mod data;
mod program;

pub use config::Environment;
pub use config::FileView;
pub use config::ExecAction;
pub use program::Program;
