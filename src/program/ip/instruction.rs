use chrono::{Utc, Datelike, Timelike};
use rand;

use data::{Value, Point, Delta};
use program::Context;
use super::Ip;

const HANDPRINT: i32 = 0x4a474d59;
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[doc(hidden)]
impl Ip {
    // Control flow

    pub(crate) fn go_east(&mut self) {
        self.set_delta(Delta { dx: 1, dy: 0 });
    }

    pub(crate) fn go_south(&mut self) {
        self.set_delta(Delta { dx: 0, dy: 1 });
    }

    pub(crate) fn go_west(&mut self) {
        self.set_delta(Delta { dx: -1, dy: 0 });
    }

    pub(crate) fn go_north(&mut self) {
        self.set_delta(Delta { dx: 0, dy: -1 });
    }

    pub(crate) fn trampoline(&mut self, ctx: &Context) {
        if !ctx.space.is_last(self.position, self.delta) {
            self.step(&ctx.space);
        }
    }

    pub(crate) fn reflect(&mut self) {
        self.delta = self.delta.reverse();
    }

    pub(crate) fn turn_left(&mut self) {
        self.delta = self.delta.rotate_left();
    }

    pub(crate) fn turn_right(&mut self) {
        self.delta = self.delta.rotate_right();
    }

    pub(crate) fn randomize_delta(&mut self) {
        let (dx, dy) = match rand::random::<u8>() % 4 {
            0 => ( 1,  0),
            1 => ( 0,  1),
            2 => (-1,  0),
            3 => ( 0, -1),
            _ => unreachable!(),
        };

        self.set_delta(Delta { dx, dy });
    }

    pub(crate) fn absolute_delta(&mut self) {
        let dy = self.pop();
        let dx = self.pop();

        self.set_delta(Delta { dx, dy });
    }

    pub(crate) fn jump(&mut self, ctx: &Context) {
        let n = self.pop();
        let delta = self.delta;

        self.delta *= n;
        self.step(&ctx.space);

        self.delta = delta;
    }

    pub(crate) fn stop(&mut self, ctx: &mut Context) {
        ctx.control.delete_ip();
    }

    pub(crate) fn terminate(&mut self, ctx: &mut Context) {
        ctx.control.terminate(self.pop());
    }

    // Logic

    pub(crate) fn negate(&mut self) {
        let v = self.pop();

        if v == 0 {
            self.push(1);
        } else {
            self.push(0);
        }
    }

    pub(crate) fn greater_than(&mut self) {
        let b = self.pop();
        let a = self.pop();

        if a > b {
            self.push(1);
        } else {
            self.push(0);
        }
    }

    pub(crate) fn if_east_west(&mut self) {
        let v = self.pop();

        if v == 0 {
            self.go_east();
        } else {
            self.go_west();
        }
    }

    pub(crate) fn if_north_south(&mut self) {
        let v = self.pop();

        if v == 0 {
            self.go_south();
        } else {
            self.go_north();
        }
    }

    pub(crate) fn compare(&mut self) {
        let b = self.pop();
        let a = self.pop();

        if a < b {
            self.turn_left();
        } else if a > b {
            self.turn_right();
        }
    }

    // Stack manipulation

    pub(crate) fn discard(&mut self) {
        self.pop();
    }

    pub(crate) fn duplicate(&mut self) {
        let v = self.pop();

        self.push(v);
        self.push(v);
    }

    pub(crate) fn swap(&mut self) {
        let v = self.pop();
        let w = self.pop();

        self.push(v);
        self.push(w);
    }

    pub(crate) fn clear(&mut self) {
        self.stacks.clear();
    }

    // Stack stack manipulation

    pub(crate) fn begin_block(&mut self) {
        let n = self.pop();

        self.stacks.create_stack(n, self.storage);
        self.storage = self.position + self.delta;
    }

    pub(crate) fn end_block(&mut self) {
        if self.stacks.single() {
            self.reflect();
            return;
        }

        let n = self.pop();
        let storage = self.stacks.delete_stack(n);

        self.storage = storage;
    }

    pub(crate) fn dig(&mut self) {
        if self.stacks.single() {
            self.reflect();
            return;
        }

        let n = self.pop();

        self.stacks.transfer_elements(n);
    }

    // Arithmetic

    pub(crate) fn push_zero(&mut self) {
        self.push(0);
    }

    pub(crate) fn push_one(&mut self) {
        self.push(1);
    }

    pub(crate) fn push_two(&mut self) {
        self.push(2);
    }

    pub(crate) fn push_three(&mut self) {
        self.push(3);
    }

    pub(crate) fn push_four(&mut self) {
        self.push(4);
    }

    pub(crate) fn push_five(&mut self) {
        self.push(5);
    }

    pub(crate) fn push_six(&mut self) {
        self.push(6);
    }

    pub(crate) fn push_seven(&mut self) {
        self.push(7);
    }

    pub(crate) fn push_eight(&mut self) {
        self.push(8);
    }

    pub(crate) fn push_nine(&mut self) {
        self.push(9);
    }

    pub(crate) fn push_ten(&mut self) {
        self.push(10);
    }

    pub(crate) fn push_eleven(&mut self) {
        self.push(11);
    }

    pub(crate) fn push_twelve(&mut self) {
        self.push(12);
    }

    pub(crate) fn push_thirteen(&mut self) {
        self.push(13);
    }

    pub(crate) fn push_fourteen(&mut self) {
        self.push(14);
    }

    pub(crate) fn push_fifteen(&mut self) {
        self.push(15);
    }

    pub(crate) fn add(&mut self) {
        let b = self.pop();
        let a = self.pop();

        self.push(a + b);
    }

    pub(crate) fn sub(&mut self) {
        let b = self.pop();
        let a = self.pop();

        self.push(a - b);
    }

    pub(crate) fn mul(&mut self) {
        let b = self.pop();
        let a = self.pop();

        self.push(a * b);
    }

    pub(crate) fn div(&mut self) {
        let b = self.pop();
        let a = self.pop();

        if b == 0 {
            self.push(0)
        } else {
            self.push(a / b);
        }
    }

    pub(crate) fn rem(&mut self) {
        let b = self.pop();
        let a = self.pop();

        if b == 0 {
            self.push(0);
        } else {
            self.push(a % b);
        }
    }

    // Strings

    pub(crate) fn string_mode(&mut self) {
        self.string = true;
    }

    pub(crate) fn fetch_char(&mut self, ctx: &Context) {
        let v = if ctx.space.is_last(self.position, self.delta) {
            32
        } else {
            ctx.space.get(self.position + self.delta)
        };

        self.push(v);
        self.step(&ctx.space);
    }

    pub(crate) fn store_char(&mut self, ctx: &mut Context) {
        let v = self.pop();

        ctx.space.set(self.position + self.delta, v);
        self.step(&ctx.space);
    }

    // Reflection

    pub(crate) fn get(&mut self, ctx: &Context) {
        let dy = self.pop();
        let dx = self.pop();

        let v = ctx.space.get(self.storage + Delta { dx, dy });
        self.push(v);
    }

    pub(crate) fn put(&mut self, ctx: &mut Context) {
        let dy = self.pop();
        let dx = self.pop();
        let v = self.pop();

        ctx.space.set(self.storage + Delta { dx, dy }, v);
    }

    // Input/Output

    pub(crate) fn output_decimal(&mut self, ctx: &mut Context) {
        let v = self.pop();

        if !ctx.env.write_decimal(v) {
            self.reflect();
        }
    }

    pub(crate) fn output_char(&mut self, ctx: &mut Context) {
        let v = self.pop();

        if let Some(c) = ::std::char::from_u32(v as u32) {
            if !ctx.env.write_char(c) {
                self.reflect();
            }
        } else {
            self.reflect();
        }
    }

    pub(crate) fn input_decimal(&mut self, ctx: &mut Context) {
        match ctx.env.read_decimal() {
            Some(v) => self.push(v),
            None    => self.reflect(),
        }
    }

    pub(crate) fn input_char(&mut self, ctx: &mut Context) {
        match ctx.env.read_char() {
            Some(v) => self.push(v as i32),
            None    => self.reflect(),
        }
    }

    pub(crate) fn write_file(&mut self, ctx: &mut Context) {
        if let Some(path) = self.pop_string() {
            let v = self.pop();
            let y = self.pop();
            let x = self.pop();
            let h = self.pop();
            let w = self.pop();

            let trim_right = v & 1 == 1;

            let mut i;
            let mut j = y;
            let mut s = String::new();
            let mut spaces = 0;
            let mut newlines = 0;

            while j - y < h {
                i = x;

                while i - x < w {
                    let Point { x: sx, y: sy } = self.storage;
                    let v = ctx.space.get(Point { x: i + sx, y: j + sy });

                    if v == ' ' as i32 {
                        spaces += 1;
                    } else {
                        for _ in 0..spaces {
                            s.push(' ');
                        }

                        spaces = 0;

                        if i == x {
                            for _ in 0..newlines {
                                s.push('\n');
                            }
                            newlines = 0;
                        }

                        if let Some(c) = ::std::char::from_u32(v as u32) {
                            s.push(c);
                        } else {
                            self.reflect();
                            return;
                        }
                    }

                    i += 1;
                }

                if !trim_right {
                    for _ in 0..spaces {
                        s.push(' ');
                    }
                }

                j += 1;
                newlines += 1;
                spaces = 0;
            }

            if !trim_right {
                for _ in 1..newlines {
                    s.push('\n');
                }
            }

            s.push('\n');

            if !ctx.env.write_file(&path, &s) {
                self.reflect();
            }
        } else {
            self.reflect();
        }
    }

    pub(crate) fn read_file(&mut self, ctx: &mut Context) {
        if let Some(path) = self.pop_string() {
            let v = self.pop();
            let y = self.pop();
            let x = self.pop();

            let linear = v & 1 == 1;

            let mut i = x;
            let mut j = y;

            let mut w = 0;

            if let Some(s) = ctx.env.read_file(&path) {
                for c in s.chars() {
                    if c == '\n' && !linear {
                        i = x;
                        j += 1;
                    } else if linear || c != '\r' {
                        if c != ' ' {
                            let Point { x: sx, y: sy } = self.storage;
                            ctx.space.set(Point { x: i + sx, y: j + sy }, c as i32);
                        }
                        i += 1;
                        if i - x > w {
                            w = i - x;
                        }
                    }
                }

                self.push(w);
                self.push(j - y);
                self.push(x);
                self.push(y);
            } else {
                self.reflect();
            }
        } else {
            self.reflect();
        }
    }

    // Concurrency

    pub(crate) fn split(&mut self, ctx: &mut Context) {
        let mut ip = self.clone();

        ip.reflect();
        ip.step(&ctx.space);
        ip.find_command(&ctx.space);

        ctx.control.add_ip(ip);
    }

    // Fingerprints

    pub(crate) fn load_semantics(&mut self) {
        let v = self.pop();

        if v <= 0 {
            self.reflect();
        } else {
            let mut fp = 0;

            for _ in 0..v {
                let n = self.pop();

                fp <<= 8;
                fp += n;
            }

            self.reflect(); // TODO implement
        }
    }

    pub(crate) fn unload_semantics(&mut self) {
        let v = self.pop();

        if v <= 0 {
            self.reflect();
        } else {
            let mut fp = 0;

            for _ in 0..v {
                let n = self.pop();

                fp <<= 8;
                fp += n;
            }

            self.reflect(); // TODO implement
        }
    }

    // Other

    pub(crate) fn iterate(&mut self, ctx: &mut Context) {
        let n = self.pop();

        if n <= 0 {
            if n == 0 {
                self.step(&ctx.space);
                self.find_command(&ctx.space);
            }
            return;
        }

        let v = self.peek_command(&ctx.space);
        if let Some(c) = ::std::char::from_u32(v as u32) {
            if !is_idempotent(c) {
                for _ in 1..n {
                    self.execute(ctx, c);
                }
            }
            self.execute(ctx, c);
        } else {
            self.reflect();
        }
    }

    pub(crate) fn system_execute(&mut self, ctx: &mut Context) {
        if let Some(cmd) = self.pop_string() {
            match ctx.env.execute(&cmd) {
                Some(v) => self.push(v),
                None    => self.reflect(),
            }
        } else {
            self.reflect();
        }
    }

    pub(crate) fn get_sysinfo(&mut self, ctx: &mut Context) {
        let n = self.pop();
        let mut num_cells = 0;

        let space = &ctx.space;
        let io = &ctx.env;

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
        for ref a in io.cmd_args() {
            num_cells += self.push_string(a);
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
        '<' | '>' | '?' | '@' | '^' | 'n' | 'q' | 'v' | 'z' => true,
        _                                                   => false,
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
