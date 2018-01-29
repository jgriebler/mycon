pub mod data;
pub mod program;

pub use data::{Value, Point, Delta};
pub use data::space::Space;
pub use data::stack::StackStack;
pub use program::Program;
pub use program::ip::Ip;
