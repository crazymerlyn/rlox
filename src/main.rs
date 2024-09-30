#[macro_use]
extern crate lazy_static;
use std::env;

mod interpreter;
use interpreter::Interpreter;

mod errors;

#[cfg(test)]
mod tests;

mod ast;
mod builtins;
mod callable;
mod evaluable;
mod interpretable;
mod parser;
mod scanner;

fn main() {
    if env::args().len() > 2 {
        println!("Usage: rlox [script]");
    } else if let Some(script) = env::args().nth(1) {
        Interpreter::new().run_path(script).unwrap();
    } else {
        Interpreter::new().run_prompt().unwrap();
    }
}
