extern crate mycon;

use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

use mycon::*;

fn main() {
    let exit = {
        let path = env::args().nth(1).expect("missing file");
        let mut file = File::open(path).expect("failed to open file");
        let mut code = String::new();

        file.read_to_string(&mut code).expect("failed to read file");
        let mut prog = Program::read(&code);

        prog.run()
    };

    process::exit(exit);
}
