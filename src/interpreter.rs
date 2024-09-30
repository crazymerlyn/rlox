use std::fmt;
use std::fs::File;
use std::path::Path;

use std::io::{self, BufRead, Read, Write};

use anyhow::Result;

use std::collections::HashMap;

use crate::ast::*;
use crate::builtins::*;
use crate::interpretable::Interpretable;
use crate::parser::Parser;
use crate::scanner::Scanner;

pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut env = Environment::new();
        env.insert("clock", Value::BuiltinFunc("clock".to_string(), 0, clock));
        Interpreter { env }
    }

    pub fn run_prompt(&mut self) -> io::Result<()> {
        let stdin = io::stdin();
        print!("> ");
        io::stdout().flush()?;
        for line in stdin.lock().lines() {
            match self.run(&line.unwrap()) {
                Ok(v) => println!("{v}"),
                Err(e) => eprintln!("{e}"),
            }
            print!("> ");
            io::stdout().flush()?;
        }
        println!();
        Ok(())
    }

    pub fn run_path<P: AsRef<Path>>(&mut self, path: P) -> Result<Value> {
        let mut s = String::new();
        let mut file = File::open(path)?;
        file.read_to_string(&mut s)?;
        self.run(&s)
    }

    pub fn run(&mut self, code: &str) -> Result<Value> {
        let scanner = Scanner::new(code);
        let stmts = scanner
            .scan_tokens()
            .and_then(|tokens| Parser::new(tokens).parse())?;

        let mut last_val = Value::Nil;
        for stmt in stmts {
            last_val = stmt.interpret(&mut self.env)?;
        }
        Ok(last_val)
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
                return Some(v);
            }
        }
        None
    }

    pub fn export_non_globals(&mut self) -> Vec<HashMap<String, Value>> {
        self.maps.split_off(1)
    }

    pub fn import_non_globals(&mut self, maps: Vec<HashMap<String, Value>>) {
        self.maps.truncate(1);
        self.maps.extend(maps);
    }

    pub fn insert(&mut self, s: &str, v: Value) {
        let n = self.maps.len();
        self.maps[n - 1].insert(s.to_owned(), v);
    }

    pub fn update(&mut self, s: &str, v: Value) -> Option<Value> {
        self.maps
            .iter_mut()
            .rev()
            .find_map(|map| map.get_mut(s))
            .map(|val| {
                *val = v.clone();
                v
            })
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, map) in self.maps.iter().enumerate() {
            writeln!(f, "Level: {}", i)?;
            for (name, val) in map.iter() {
                writeln!(f, "{} -> {}", name, val)?;
            }
        }
        Ok(())
    }
}
