use std::cell::RefCell;
use std::fmt;
use std::fs::File;
use std::path::Path;

use std::io::{self, BufRead, Read, Write};
use std::rc::Rc;

use anyhow::Result;

use std::collections::HashMap;

use crate::ast::*;
use crate::builtins::*;
use crate::interpretable::Interpretable;
use crate::parser::Parser;
use crate::scanner::Scanner;

pub struct Interpreter {
    env: EnvRef,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut env = Environment::new();
        env.insert("clock", Value::BuiltinFunc("clock".to_string(), 0, clock));
        Interpreter {
            env: Rc::new(RefCell::new(env)),
        }
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
            last_val = stmt.interpret(self.env.clone())?;
        }
        Ok(last_val)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    map: HashMap<String, ValRef>,
    parent: Option<Rc<RefCell<Environment>>>,
}

pub type EnvRef = Rc<RefCell<Environment>>;

impl Environment {
    pub fn new() -> Environment {
        Environment {
            map: HashMap::new(),
            parent: None,
        }
    }

    pub fn wrap(parent: EnvRef) -> Environment {
        Environment {
            map: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn get(&self, s: &str) -> Option<ValRef> {
        if let Some(v) = self.map.get(s) {
            return Some(v.clone());
        }
        if let Some(ref parent) = self.parent {
            return parent.borrow().get(s);
        }
        None
    }

    pub fn insert(&mut self, s: &str, v: Value) {
        self.map.insert(s.to_owned(), Rc::new(RefCell::new(v)));
    }

    pub fn update(&mut self, s: &str, v: Value) -> Option<Value> {
        self.map.get_mut(s).map(|val| {
            *val = Rc::new(RefCell::new(v.clone()));
            v
        })
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut i = 0;
        let mut env = Some(Rc::new(RefCell::new(self.clone())));
        while let Some(e) = env {
            writeln!(f, "Level: {}", i)?;
            for (name, val) in e.borrow().map.iter() {
                writeln!(f, "{} -> {}", name, val.borrow_mut())?;
            }
            i += 1;
            env = e.borrow().parent.clone();
        }
        Ok(())
    }
}
