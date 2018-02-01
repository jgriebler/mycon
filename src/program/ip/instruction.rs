use rand;

use data::{Value, Point, Delta};
use data::space::Space;
use super::Ip;

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
        self.step(&space);
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

        self.delta *= n - 1;
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

        self.push(a / b);
    }

    pub fn rem(&mut self) {
        let b = self.pop();
        let a = self.pop();

        self.push(a % b);
    }

    // Reflection

    pub fn get(&mut self, space: &Space) {
        let y = self.pop();
        let x = self.pop();

        let v = space.get(Point { x, y } + self.storage);
        self.push(v);
    }

    pub fn put(&mut self, space: &mut Space) {
        let y = self.pop();
        let x = self.pop();
        let v = self.pop();

        space.set(Point { x, y } + self.storage, v);
    }
}
