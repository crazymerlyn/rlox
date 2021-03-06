use std::path::Path;
use std::fs::File;
use std::fmt;

use std::io::Result;
use std::io::{self, Write, Read, BufRead};

use std::process;

use std::collections::HashMap;

use scanner::Scanner;
use parser::Parser;
use interpretable::Interpretable;
use ast::*;
use builtins::*;

pub struct Interpreter {
    had_error: bool,
    env: Environment,
}


impl Interpreter {
    pub fn new() -> Interpreter {
        let mut env = Environment::new();
        env.insert("clock".to_string(), Value::BuiltinFunc("clock".to_string(), 0, clock));
        Interpreter {
            had_error: false,
            env: env,
        }
    }

    pub fn run_prompt(&mut self) -> Result<()> {
        let stdin = io::stdin();
        print!("> ");
        io::stdout().flush()?;
        for line in stdin.lock().lines() {
            self.run(line.unwrap(), true);
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
        self.run(s, false);
        if self.had_error { process::exit(65); };
        Ok(())
    }

    fn run<S: AsRef<str>>(&mut self, code: S, print_value: bool) {
        let scanner = Scanner::new(code);
        let stmts = match scanner.scan_tokens()
            .and_then(|tokens| Parser::new(tokens).parse()) {
            Ok(x) => x,
            Err(e) => {
                self.error(e.to_string());
                return;
            }
        };

        for stmt in stmts {
            if self.had_error {
                break
            }
            match stmt.interpret(&mut self.env) {
                Ok(v) => {
                    if print_value && Value::Nil != v {
                        println!("{}", v);
                    }
                }
                Err(e) => self.error(format!("{}", e)),
            }
        }
    }

    fn error<S: AsRef<str>>(&mut self, message: S) {
        self.report("", message.as_ref())
    }

    fn report<S: AsRef<str>>(&mut self, context: S, message: S) {
        writeln!(io::stderr(), "{}{}", context.as_ref(), message.as_ref()).unwrap();
        self.had_error = true;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    maps: Vec<HashMap<String, Value>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            maps: vec![HashMap::new()],
        }
    }

    pub fn push_local_scope(&mut self) {
        self.maps.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        if !self.maps.is_empty() {
            self.maps.pop();
        } else {
            panic!("Trying to pop non-existant scope!");
        }
    }

    pub fn get(&self, s: &str) -> Option<&Value> {
        for map in self.maps.iter().rev() {
            if let Some(v) = map.get(s) {
                return Some(v)
            }
        }
        None
    }

    pub fn export_non_globals(&mut self) -> Vec<HashMap<String, Value>> {
        let non_globals = self.maps.iter().skip(1).cloned().collect::<Vec<_>>();
        self.maps.truncate(1);
        non_globals
    }

    pub fn import_non_globals(&mut self, maps: Vec<HashMap<String, Value>>) {
        self.maps.truncate(1);
        self.maps.extend(maps);
    }

    pub fn insert(&mut self, s: String, v: Value) {
        let n = self.maps.len();
        self.maps[n-1].insert(s, v);
    }

    pub fn update(&mut self, s: String, v: Value) {
        for map in self.maps.iter_mut().rev() {
            if map.get(&s).is_some() {
                map.insert(s.clone(), v.clone());
                return;
            }
        }
        panic!("Trying to update non-existant variable");
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, map) in self.maps.iter().enumerate() {
            write!(f, "Level: {}\n", i)?;
            for (name, val) in map.iter() {
                write!(f, "{} -> {}\n", name, val)?;
            }
        }
        Ok(())
    }
}

