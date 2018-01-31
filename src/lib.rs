//! An interpreter and a library for the esoteric programming language [Befunge-98].
//!
//! [Befunge-98]: https://esolangs.org/wiki/Funge-98

#![warn(missing_docs)]

extern crate rand;

pub mod data;
pub mod program;

pub use data::{Value, Point, Delta};
pub use data::space::Space;
pub use data::stack::StackStack;
pub use program::Program;
pub use program::ip::Ip;
