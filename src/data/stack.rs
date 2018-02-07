//! The stack stack of an [`Ip`].
//!
//! [`Ip`]: ../../program/ip/struct.Ip.html

use super::{Value, Point};

type Stack = Vec<Value>;

/// The stack stack of an [`Ip`].
///
/// This is simply a stack of stacks, each stack storing [`Value`]s. The stack
/// stack always contains at least a single stack, though the individual stacks
/// may be empty.
/// 
/// [`Value`]: ../type.Value.html
/// [`Ip`]: ../../program/ip/struct.Ip.html
#[derive(Clone, Debug)]
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

    fn second(&mut self) -> &mut Stack {
        let len = self.stacks.len();

        &mut self.stacks[len - 2]
    }

    /// Checks whether the `StackStack` contains only a single stack.
    pub fn single(&self) -> bool {
        self.stacks.len() == 1
    }

    /// Pushes a [`Value`] to the top stack on the `StackStack`.
    ///
    /// [`Value`]: ../type.Value.html
    pub fn push(&mut self, value: Value) {
        self.top().push(value);
    }

    /// Pushes a string to the top stack on the `StackStack`.
    ///
    /// Returns the number of cells that were pushed.
    ///
    /// Afterwards, the first character in the string will be at the top of the
    /// stack. The string is delimited by a 0.
    pub fn push_string(&mut self, s: &str) -> usize {
        let mut n = 1;

        self.push(0);
        self.top().append(&mut s.chars()
                          .rev()
                          .map(|c| {
                              n += 1;
                              c as i32
                          })
                          .collect());

        n
    }

    /// Pops a [`Value`] from the top stack on the `StackStack`.
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

    /// Returns the `n`th cell of the top stack, counted from the top.
    ///
    /// If `n` is out of bounds, 0 will be returned.
    pub fn nth(&mut self, n: usize) -> Value {
        let top = self.top();
        let len = top.len();

        if n >= len {
            0
        } else {
            top[len - n]
        }
    }

    /// Tries to pop a string from the top stack on the `StackStack`.
    ///
    /// It will be popped character by character, until a 0 is encountered.
    /// `None` will be returned if the string is not valid UTF-8.
    pub fn pop_string(&mut self) -> Option<String> {
        let mut s = String::new();

        loop {
            let v = self.pop();

            if v == 0 {
                break;
            }

            if let Some(c) = ::std::char::from_u32(v as u32) {
                s.push(c);
            } else {
                return None;
            }
        }

        Some(s)
    }

    /// Completely empties the top stack on the `StackStack`.
    pub fn clear(&mut self) {
        self.top().clear();
    }

    /// Returns a vector containing the size of each stack on the `StackStack`.
    ///
    /// The first element is the size of the bottommost stack.
    pub fn stack_sizes(&self) -> Vec<usize> {
        self.stacks.iter().map(Vec::len).collect()
    }

    /// Deletes `n` cells from the top stack, from the top down.
    ///
    /// # Panics
    ///
    /// Panics if `n` exceeds the number of elements in the top stack.
    pub fn delete_cells(&mut self, n: usize) {
        let top = self.top();
        let len = top.len();

        top.drain(len - n ..);
    }

    /// Pushes a new stack onto the `StackStack`.
    ///
    /// `n` elements from the stack previously on top will be transferred to the
    /// new stack. Then, the given [`Point`] will be pushed onto the (now)
    /// second stack.
    ///
    /// For details, consult the description of the '{'-instruction in the
    /// Funge-98 specification.
    ///
    /// [`Point`]: ../struct.Point.html
    pub fn create_stack(&mut self, n: i32, Point { x, y }: Point) {
        let mut new = Vec::new();

        {
            let top = self.top();
            let len = top.len();

            let m = n as u32 as usize;

            if n > 0 {
                if m <= len {
                    new.append(&mut top.split_off(len - m));
                } else {
                    new.append(&mut vec![0; m - len]);
                    new.append(&mut top.split_off(len));
                }
            } else if n < 0 {
                top.append(&mut vec![0; -n as usize]);
            }

            top.push(x);
            top.push(y);
        }

        self.stacks.push(new);
    }

    /// Deletes the top stack of the `StackStack`.
    ///
    /// A [`Point`] will be popped off the stack directly below the one to be
    /// deleted. `n` elements from the deleted stack will be transferred to the
    /// stack now on top.
    ///
    /// For details, consult the description of the '}'-instruction in the
    /// Funge-98 specification.
    ///
    /// # Panics
    ///
    /// Panics if there is only one stack on the `StackStack`.
    ///
    /// [`Point`]: ../struct.Point.html
    pub fn delete_stack(&mut self, n: i32) -> Point {
        use std::cmp::min;

        assert!(!self.single());

        let mut old = self.stacks.pop().unwrap();
        let len = old.len();

        let top = self.top();

        let y = top.pop().unwrap_or(0);
        let x = top.pop().unwrap_or(0);

        let m = n as u32 as usize;

        if n > 0 {
            if m <= len {
                top.append(&mut old.split_off(len - m));
            } else {
                top.append(&mut vec![0; m - len]);
                top.append(&mut old.split_off(len));
            }
        } else if n < 0 {
            let len = top.len();
            top.drain(len - min(len, -n as usize) .. len);
        }

        Point { x, y }
    }

    /// Transfers `n` elements from the second stack to the top stack.
    ///
    /// If `n` is negative, the same number of elements will be transferred in
    /// the other direction instead.
    ///
    /// The order of the elements is reversed.
    ///
    /// # Panics
    ///
    /// Panics if there is only one stack on the `StackStack`.
    pub fn transer_elements(&mut self, n: i32) {
        assert!(!self.single());

        if n > 0 {
            for _ in 0..n {
                let v = self.second().pop().unwrap_or(0);
                self.top().push(v);
            }
        } else if n < 0 {
            for _ in 0..-n {
                let v = self.top().pop().unwrap_or(0);
                self.second().push(v);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_push_pop() {
        let mut stack = StackStack::new();

        stack.push(3);
        stack.push(5);

        assert_eq!(5, stack.pop());
        assert_eq!(3, stack.pop());
        assert_eq!(0, stack.pop());
    }

    #[test]
    fn stack_clear() {
        let mut stack = StackStack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.clear();

        assert_eq!(0, stack.pop());
    }

    #[test]
    fn stack_string() {
        let mut stack = StackStack::new();

        stack.push(1);

        assert_eq!(11, stack.push_string("Befunge-98"));
        assert_eq!(Some(String::from("Befunge-98")), stack.pop_string());
        assert_eq!(1, stack.pop());
        assert_eq!(0, stack.pop());
    }
}
