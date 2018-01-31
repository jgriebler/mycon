//! The stack stack of an [`Ip`].
//!
//! [`Ip`]: ../../program/ip/struct.Ip.html

use super::Value;

type Stack = Vec<Value>;

/// The stack stack of an [`Ip`].
///
/// This is simply a stack of stacks, each stack storing [`Value`]s. The stack
/// stack always contains at least a single stack, though the individual stacks
/// may be empty.
/// 
/// [`Value`]: ../type.Value.html
/// [`Ip`]: ../../program/ip/struct.Ip.html
#[derive(Clone)]
pub struct StackStack {
    stacks: Vec<Stack>,
}

impl StackStack {
    /// Creates a new `StackStack` containing a single empty stack.
    pub fn new() -> StackStack {
        StackStack {
            stacks: vec![Vec::new()],
        }
    }

    fn top(&mut self) -> &mut Stack {
        let len = self.stacks.len();

        &mut self.stacks[len - 1]
    }

    /// Pushes a [`Value`] to the top stack on the stack stack.
    ///
    /// [`Value`]: ../type.Value.html
    pub fn push(&mut self, value: Value) {
        self.top().push(value);
    }

    /// Pops a [`Value`] from the top stack on the stack stack.
    ///
    /// If the top stack is empty, `0` will be returned.
    ///
    /// [`Value`]: ../type.Value.html
    pub fn pop(&mut self) -> Value {
        let top = self.top();

        match top.pop() {
            Some(v) => v,
            None    => 0,
        }
    }

    pub fn clear(&mut self) {
        self.top().clear();
    }
}
