use rand;

use data::Delta;
use data::space::Space;
use program::config::IoContext;
use super::{Ip, ExecResult};

#[doc(hidden)]
impl Ip {
    // Control flow

    pub fn go_east(&mut self) {
        self.set_delta(Delta { dx: 1, dy: 0 });
    }

    pub fn go_south(&mut self) {
        self.set_delta(Delta { dx: 0, dy: 1 });
    }

    pub fn go_west(&mut self) {
        self.set_delta(Delta { dx: -1, dy: 0 });
    }

    pub fn go_north(&mut self) {
        self.set_delta(Delta { dx: 0, dy: -1 });
    }

    pub fn trampoline(&mut self, space: &Space) {
        if !space.is_last(self.position, self.delta) {
            self.step(&space);
        }
    }

    pub fn reverse(&mut self) {
        self.delta = self.delta.reverse();
    }

    pub fn turn_left(&mut self) {
        self.delta = self.delta.rotate_left();
    }

    pub fn turn_right(&mut self) {
        self.delta = self.delta.rotate_right();
    }

    pub fn randomize_delta(&mut self) {
        let (dx, dy) = match rand::random::<u8>() % 4 {
            0 => ( 1,  0),
            1 => ( 0,  1),
            2 => (-1,  0),
            3 => ( 0, -1),
            _ => unreachable!(),
        };

        self.set_delta(Delta { dx, dy });
    }

    pub fn absolute_delta(&mut self) {
        let dy = self.pop();
        let dx = self.pop();

        self.set_delta(Delta { dx, dy });
    }

    pub fn jump(&mut self, space: &Space) {
        let n = self.pop();
        let delta = self.delta;

        self.delta *= n;
        self.step(space);

        self.delta = delta;
    }

    // Logic

    pub fn negate(&mut self) {
        let v = self.pop();

        if v == 0 {
            self.push(1);
        } else {
            self.push(0);
        }
    }

    pub fn greater_than(&mut self) {
        let b = self.pop();
        let a = self.pop();

        if a > b {
            self.push(1);
        } else {
            self.push(0);
        }
    }

    pub fn if_east_west(&mut self) {
        let v = self.pop();

        if v == 0 {
            self.go_east();
        } else {
            self.go_west();
        }
    }

    pub fn if_north_south(&mut self) {
        let v = self.pop();

        if v == 0 {
            self.go_south();
        } else {
            self.go_north();
        }
    }

    pub fn compare(&mut self) {
        let b = self.pop();
        let a = self.pop();

        if a < b {
            self.turn_left();
        } else if a > b {
            self.turn_right();
        }
    }

    // Stack manipulation

    pub fn discard(&mut self) {
        self.pop();
    }

    pub fn duplicate(&mut self) {
        let v = self.pop();

        self.push(v);
        self.push(v);
    }

    pub fn swap(&mut self) {
        let v = self.pop();
        let w = self.pop();

        self.push(v);
        self.push(w);
    }

    pub fn clear(&mut self) {
        self.stacks.clear();
    }

    // Stack stack manipulation

    pub fn begin_block(&mut self) {
        let n = self.pop();

        self.stacks.create_stack(n, self.storage);
        self.storage = self.position + self.delta;
    }

    pub fn end_block(&mut self) {
        if self.stacks.single() {
            self.reverse();
            return;
        }

        let n = self.pop();
        let storage = self.stacks.delete_stack(n);

        self.storage = storage;
    }

    pub fn dig(&mut self) {
        if self.stacks.single() {
            self.reverse();
            return;
        }

        let n = self.pop();

        self.stacks.transer_elements(n);
    }

    // Arithmetic

    pub fn push_zero(&mut self) {
        self.push(0);
    }

    pub fn push_one(&mut self) {
        self.push(1);
    }

    pub fn push_two(&mut self) {
        self.push(2);
    }

    pub fn push_three(&mut self) {
        self.push(3);
    }

    pub fn push_four(&mut self) {
        self.push(4);
    }

    pub fn push_five(&mut self) {
        self.push(5);
    }

    pub fn push_six(&mut self) {
        self.push(6);
    }

    pub fn push_seven(&mut self) {
        self.push(7);
    }

    pub fn push_eight(&mut self) {
        self.push(8);
    }

    pub fn push_nine(&mut self) {
        self.push(9);
    }

    pub fn push_ten(&mut self) {
        self.push(10);
    }

    pub fn push_eleven(&mut self) {
        self.push(11);
    }

    pub fn push_twelve(&mut self) {
        self.push(12);
    }

    pub fn push_thirteen(&mut self) {
        self.push(13);
    }

    pub fn push_fourteen(&mut self) {
        self.push(14);
    }

    pub fn push_fifteen(&mut self) {
        self.push(15);
    }

    pub fn add(&mut self) {
        let b = self.pop();
        let a = self.pop();

        self.push(a + b);
    }

    pub fn sub(&mut self) {
        let b = self.pop();
        let a = self.pop();

        self.push(a - b);
    }

    pub fn mul(&mut self) {
        let b = self.pop();
        let a = self.pop();

        self.push(a * b);
    }

    pub fn div(&mut self) {
        let b = self.pop();
        let a = self.pop();

        if b == 0 {
            self.push(0)
        } else {
            self.push(a / b);
        }
    }

    pub fn rem(&mut self) {
        let b = self.pop();
        let a = self.pop();

        if b == 0 {
            self.push(0);
        } else {
            self.push(a % b);
        }
    }

    // Strings

    pub fn string_mode(&mut self) {
        self.string = true;
    }

    pub fn fetch_char(&mut self, space: &Space) {
        let v = if space.is_last(self.position, self.delta) {
            32
        } else {
            space.get(self.position + self.delta)
        };

        self.push(v);
        self.step(space);
    }

    pub fn store_char(&mut self, space: &mut Space) {
        let v = self.pop();

        space.set(self.position + self.delta, v);
        self.step(space);
    }

    // Reflection

    pub fn get(&mut self, space: &Space) {
        let dy = self.pop();
        let dx = self.pop();

        let v = space.get(self.storage + Delta { dx, dy });
        self.push(v);
    }

    pub fn put(&mut self, space: &mut Space) {
        let dy = self.pop();
        let dx = self.pop();
        let v = self.pop();

        space.set(self.storage + Delta { dx, dy }, v);
    }

    // Input/Output

    pub fn output_decimal(&mut self, io: &mut IoContext) {
        let v = self.pop();

        if !io.write_decimal(v) {
            self.reverse();
        }
    }

    pub fn output_char(&mut self, io: &mut IoContext) {
        let v = self.pop();

        if let Some(c) = ::std::char::from_u32(v as u32) {
            if !io.write_char(c) {
                self.reverse();
            }
        } else {
            self.reverse();
        }
    }

    pub fn input_decimal(&mut self, io: &mut IoContext) {
        match io.read_decimal() {
            Some(v) => self.push(v),
            None    => self.reverse(),
        }
    }

    pub fn input_char(&mut self, io: &mut IoContext) {
        match io.read_char() {
            Some(v) => self.push(v as i32),
            None    => self.reverse(),
        }
    }

    // Concurrency

    pub fn split(&mut self) -> Ip {
        let mut ip = self.clone();

        ip.reverse();
        ip
    }

    // Other

    pub fn iterate(&mut self, space: &mut Space, io: &mut IoContext) -> ExecResult {
        let n = self.pop();

        if n <= 0 {
            if n == 0 {
                self.step(space);
                self.find_command(space);
            }
            return ExecResult::Done;
        }

        let v = self.peek_command(space);
        if let Some(c) = ::std::char::from_u32(v as u32) {
            if !is_idempotent(c) {
                for _ in 1..n {
                    self.execute(space, io, c);
                }
            }
            self.execute(space, io, c)
        } else {
            self.reverse();
            ExecResult::Done
        }
    }
}

fn is_idempotent(c: char) -> bool {
    match c {
        '<' | '>' | '?' | '@' | '^' | 'n' | 'q' | 't' | 'v' | 'z' => true,
        _                                                         => false,
    }
}
