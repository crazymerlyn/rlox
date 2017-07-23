use std::env;

mod interpreter;
use interpreter::Interpreter;

fn main() {
    if env::args().len() > 1 {
        println!("Usage: rlox [script]");
    } else if let Some(script) = env::args().nth(1) {
        Interpreter::new().run_path(script).unwrap();
    } else {
        Interpreter::new().run_prompt().unwrap();
    }
}
