use std::path::Path;
use std::fs::File;

use std::io::Result;
use std::io::{self, Write, Read, BufRead};

pub struct Interpreter {
    had_error: bool,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            had_error: false,
        }
    }

    pub fn run_prompt(&mut self) -> Result<()> {
        let stdin = io::stdin();
        print!("> ");
        io::stdout().flush()?;
        for line in stdin.lock().lines() {
            self.run(line.unwrap());
            print!("> ");
            io::stdout().flush()?;
        }
        println!();
        Ok(())
    }

    pub fn run_path<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let mut s = String::new();
        let mut file = File::open(path)?;
        file.read_to_string(&mut s)?;
        self.run(s);
        Ok(())
    }

    fn run<S: AsRef<str>>(&mut self, code: S) {
        for word in code.as_ref().split_whitespace() {
            println!("{}", word);
        }
    }

    fn error<S: AsRef<str>>(&mut self, line: usize, message: S) -> Result<()> {
        self.report(line, "", message.as_ref())
    }

    fn report<S: AsRef<str>>(&mut self, line: usize, context: S, message: S) -> Result<()> {
        write!(io::stderr(), "[line {}] Error{}: {}", line, context.as_ref(), message.as_ref())?;
        self.had_error = true;
        Ok(())
    }
}
