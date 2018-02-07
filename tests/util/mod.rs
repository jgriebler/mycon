use std::io;

use mycon::Program;

pub fn test_output(code: &str, output: &str) {
    let mut input = io::empty();
    let mut buffer = Vec::new();

    {
        let mut prog = Program::read_with_io(code, &mut input, &mut buffer);

        prog.run();
    }

    assert_eq!(output.as_bytes(), &*buffer);
}
