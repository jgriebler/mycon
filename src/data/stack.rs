use super::Value;

type Stack = Vec<Value>;

#[derive(Clone)]
pub struct StackStack {
    stacks: Vec<Stack>,
}

impl StackStack {
    pub fn new() -> StackStack {
        StackStack {
            stacks: vec![Vec::new()],
        }
    }

    fn top(&mut self) -> &mut Stack {
        let len = self.stacks.len();

        &mut self.stacks[len - 1]
    }

    pub fn push(&mut self, value: Value) {
        self.top().push(value);
    }

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
