use chrono::{Utc, Datelike, Timelike};
use rand;

use data::{Value, Point, Delta};
use data::space::Space;
use program::config::IoContext;
use super::{Ip, ExecResult};

const HANDPRINT: i32 = 0x4a474d59;
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

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

    pub fn split(&mut self, space: &Space, new_id: i32) -> Ip {
        let mut ip = self.clone();

        ip.id = new_id;
        ip.reverse();
        ip.step(space);
        ip.find_command(space);

        ip
    }

    // Fingerprints

    pub fn load_semantics(&mut self) {
        let v = self.pop();

        if v <= 0 {
            self.reverse();
        } else {
            let mut fp = 0;

            for _ in 0..v {
                let n = self.pop();

                fp <<= 8;
                fp += n;
            }

            self.reverse(); // TODO implement
        }
    }

    pub fn unload_semantics(&mut self) {
        let v = self.pop();

        if v <= 0 {
            self.reverse();
        } else {
            let mut fp = 0;

            for _ in 0..v {
                let n = self.pop();

                fp <<= 8;
                fp += n;
            }

            self.reverse(); // TODO implement
        }
    }

    // Other

    pub fn iterate(&mut self, space: &mut Space, io: &mut IoContext, new_id: Value) -> ExecResult {
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
                    self.execute(space, io, new_id, c);
                }
            }
            self.execute(space, io, new_id, c)
        } else {
            self.reverse();
            ExecResult::Done
        }
    }

    pub fn get_sysinfo(&mut self, space: &Space, io: &mut IoContext) {
        let n = self.pop();
        let mut num_cells = 0;

        let sizes = self.stacks.stack_sizes();

        // Environment variables
        num_cells += 1;
        self.push(0);
        for (k, v) in io.env_vars() {
            num_cells += self.push_string(&format!("{}={}", k, v));
        }

        // Command line arguments
        num_cells += 2;
        self.push(0);
        self.push(0);
        for a in io.cmd_args() {
            num_cells += self.push_string(&a);
        }

        // Size of each stack
        num_cells += sizes.len();
        for &l in sizes.iter() {
            self.push(l as i32);
        }

        // Total number of stacks
        num_cells += 1;
        self.push(sizes.len() as i32);

        let dt = Utc::now();

        // Time
        num_cells += 1;
        self.push(((dt.hour() << 16) + (dt.minute() << 8) + dt.second()) as i32);

        // Date
        num_cells += 1;
        self.push((dt.year() - 1900 << 16) + ((dt.month() << 8) + dt.day()) as i32);

        let (x0, y0) = space.min();
        let (x1, y1) = space.max();

        // Program size
        num_cells += 2;
        self.push(x1 - x0);
        self.push(y1 - y0);

        // Program start
        num_cells += 2;
        self.push(x0);
        self.push(y0);

        let Point { x, y } = self.storage;

        // Storage offset
        num_cells += 2;
        self.push(x);
        self.push(y);

        let Delta { dx, dy } = self.delta;

        // Delta
        num_cells += 2;
        self.push(dx);
        self.push(dy);

        let Point { x, y } = self.position;

        // Position
        num_cells += 2;
        self.push(x);
        self.push(y);

        // Team number
        num_cells += 1;
        self.push(0);

        // ID
        num_cells += 1;
        let id = self.id;
        self.push(id);

        // Dimension
        num_cells += 1;
        self.push(2);

        // Path separator
        num_cells += 1;
        self.push('/' as i32);

        // Operating paradigm
        num_cells += 1;
        self.push(io.operating_paradigm());

        // Interpreter version
        num_cells += 1;
        self.push(version_number(VERSION));

        // Interpreter handprint
        num_cells += 1;
        self.push(HANDPRINT);

        // Cell size
        num_cells += 1;
        self.push(4);

        // Flags
        num_cells += 1;
        self.push(io.flags());

        if n > 0 {
            let v = self.stacks.nth(n as usize);

            self.stacks.delete_cells(num_cells);
            self.push(v);
        }
    }
}

fn is_idempotent(c: char) -> bool {
    match c {
        '<' | '>' | '?' | '@' | '^' | 'n' | 'q' | 't' | 'v' | 'z' => true,
        _                                                         => false,
    }
}

fn version_number(s: &str) -> Value {
    let mut r = 0;

    for p in s.split('.') {
        let n: i32 = p.parse().unwrap();

        r <<= 8;
        r += n;
    }

    r
}
