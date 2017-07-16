use std::env;
use std::path::Path;
use std::fs::File;

use std::io::Result;
use std::io::{self, Write, Read, BufRead};

fn run<S: AsRef<str>>(code: S) {
    for word in code.as_ref().split_whitespace() {
        println!("{}", word);
    }
}

fn run_path<P: AsRef<Path>>(path: P) -> Result<()> {
    let mut s = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut s)?;
    run(s);
    Ok(())
}

fn run_prompt() -> Result<()> {
    let stdin = io::stdin();
    print!("> ");
    io::stdout().flush()?;
    for line in stdin.lock().lines() {
        run(line.unwrap());
        print!("> ");
        io::stdout().flush()?;
    }
    println!();
    Ok(())
}

fn main() {
    if env::args().len() > 1 {
        println!("Usage: rlox [script]");
    } else if let Some(script) = env::args().nth(1) {
        run_path(script).unwrap();
    } else {
        run_prompt().unwrap();
    }
}
