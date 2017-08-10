use evaluable::Evaluable;
use ast::*;
use errors::{Result, ErrorKind};

pub trait Interpretable {
    fn interpret(&self) -> Result<Value>;
}

impl Interpretable for Stmt {
    fn interpret(&self) -> Result<Value> {
        match *self {
            Stmt::Expr(ref expr) => {
                let _ = expr.evaluate()?;
                Ok(Value::Nil)
            }
            Stmt::Print(ref expr) => {
                match expr.evaluate()? {
                    Value::String(s) => println!("{}", s), // Print strings without double quotes
                    x => println!("{}", x),
                }
                Ok(Value::Nil)
            }
        }
    }
}

