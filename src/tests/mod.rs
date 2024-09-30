use std::{fs::File, io::Read};

use glob::glob;

use crate::Interpreter;

fn test_loxfile(path: std::path::PathBuf) {
    println!("Testing: {}", path.display());
    let mut s = String::new();
    let mut file = File::open(path).unwrap();
    file.read_to_string(&mut s).unwrap();
    Interpreter::new().run(&s).unwrap();
}

#[test]
fn test_loxfiles() {
    for entry in glob("./src/tests/loxfiles/**/*.lox").unwrap() {
        test_loxfile(entry.unwrap());
    }
}
