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

#[test]
fn hello() {
    let code = r#"a"!dlroW olleH">:#,_@"#;

    test_output(code, "Hello World!\n");
}

#[test]
fn quine() {
    let code = ":0g,:f4+-!;@,a;# _1+\n";

    test_output(code, code);
}
