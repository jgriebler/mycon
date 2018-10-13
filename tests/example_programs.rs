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

mod util;

use util::test_output;

macro_rules! from_file {
    ($file:expr) => {
        include_str!(concat!("../test_programs/", $file))
    };
}

#[test]
fn hello() {
    let code = from_file!("hello.b98");

    test_output(code, "Hello World!\n");
}

#[test]
fn quine() {
    let code = from_file!("quine.b98");

    test_output(code, code);
}

#[test]
fn fibo() {
    let code = from_file!("fibo.b98");

    test_output(code, "6765 ");
}
