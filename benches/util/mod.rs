// Copyright 2018 Johannes M. Griebler
//
// This file is part of mycon.
//
// mycon is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// mycon is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with mycon.  If not, see <https://www.gnu.org/licenses/>.

use std::io;

use mycon::{Config, Program};

pub fn run(code: &str) -> i32 {
    let mut empty = io::empty();
    let mut sink = io::sink();

    let config = Config::new().input(&mut empty).output(&mut sink);
    let mut prog = Program::read(code).config(config);

    prog.run()
}
