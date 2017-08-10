use evaluable::Evaluable;
use ast::*;
use errors::{Result, ErrorKind};
use interpreter::Environment;

pub trait Interpretable {
    fn interpret(&self, &mut Environment) -> Result<Value>;
}

impl Interpretable for Stmt {
    fn interpret(&self, env: &mut Environment) -> Result<Value> {
        match *self {
            Stmt::Expr(ref expr) => {
                let _ = expr.evaluate(env)?;
                Ok(Value::Nil)
            }
            Stmt::Print(ref expr) => {
                match expr.evaluate(env)? {
                    Value::String(s) => println!("{}", s), // Print strings without double quotes
                    x => println!("{}", x),
                }
                Ok(Value::Nil)
            }
            Stmt::Decl(ref id, ref expr) => {
                let value = expr.evaluate(env)?;
                env.insert(id.name.lexeme.clone(), value);
                Ok(Value::Nil)
            }
        }
    }
}

