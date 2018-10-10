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

extern crate mycon;

use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

use mycon::*;

fn main() {
    let exit = {
        let path = env::args().nth(1).expect("missing file");
        let mut code;

        {
            let mut file = File::open(path).expect("failed to open file");
            code = String::new();

            file.read_to_string(&mut code).expect("failed to read file");
        }

        let mut prog = Program::read(&code);

        prog.run()
    };

    process::exit(exit);
}
