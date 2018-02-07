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
