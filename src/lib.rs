//! An interpreter and a library for the esoteric programming language [Befunge-98].
//!
//! [Befunge-98]: https://esolangs.org/wiki/Funge-98

#![warn(missing_docs)]

extern crate chrono;
extern crate rand;

mod data;
mod program;

pub use program::Program;
pub use program::config::Environment;
pub use program::config::FileView;
pub use program::config::ExecAction;
