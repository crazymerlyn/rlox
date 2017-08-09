use std::path::Path;
use std::fs::File;

use std::io::Result;
use std::io::{self, Write, Read, BufRead};

use std::process;

use scanner::Scanner;
use parser::Parser;
use evaluable::Evaluable;

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
            self.had_error = false;
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
        if self.had_error { process::exit(65); };
        Ok(())
    }

    fn run<S: AsRef<str>>(&mut self, code: S) {
        let scanner = Scanner::new(code);
        let value = match scanner.scan_tokens()
            .and_then(|tokens| Parser::new(tokens).parse())
            .and_then(|expr| {
                println!("{}", expr);
                expr.evaluate()
            }) {
            Ok(x) => x,
            Err(e) => {
                self.had_error = true;
                write!(io::stderr(), "{}\n", e);
                return;
            }
        };
        println!("{}", value);
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
