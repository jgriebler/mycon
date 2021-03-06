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

#[macro_use]
extern crate criterion;
extern crate mycon;

mod util;

use criterion::Criterion;

use self::util::run;

macro_rules! from_file {
    ($file:expr) => {
        include_str!(concat!("../test_programs/", $file))
    };
}

fn bench_hello(c: &mut Criterion) {
    fn hello() -> i32 {
        let code = from_file!("hello.b98");

        run(code)
    }

    c.bench_function("hello", |b| b.iter(|| hello()));
}

fn bench_quine(c: &mut Criterion) {
    fn quine() -> i32 {
        let code = from_file!("quine.b98");

        run(code)
    }

    c.bench_function("quine", |b| b.iter(|| quine()));
}

fn bench_fibo(c: &mut Criterion) {
    fn fibo() -> i32 {
        let code = from_file!("fibo.b98");

        run(code)
    }

    c.bench_function("fibo", |b| b.iter(|| fibo()));
}

criterion_group!(benches, bench_hello, bench_quine, bench_fibo);
criterion_main!(benches);
