use std::io;

use mycon::{Environment, Program};

pub fn test_output(code: &str, output: &str) {
    let mut empty = io::empty();
    let mut buffer = Vec::new();

    {
        let env = Environment::new().input(&mut empty).output(&mut buffer);
        let mut prog = Program::read(code).env(env);

        prog.run();
    }

    assert_eq!(output.as_bytes(), &*buffer);
}
