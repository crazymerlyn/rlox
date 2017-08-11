use evaluable::Evaluable;
use ast::*;
use errors::Result;
use interpreter::Environment;

pub trait Interpretable {
    fn interpret(&self, &mut Environment) -> Result<Value>;
}

impl Interpretable for Stmt {
    fn interpret(&self, env: &mut Environment) -> Result<Value> {
        match *self {
            Stmt::Expr(ref expr) => {
                expr.evaluate(env)
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
            Stmt::Block(ref stmts) => {
                env.push_local_scope();
                for stmt in stmts {
                    stmt.interpret(env)?;
                }
                env.pop_scope();
                Ok(Value::Nil)
            }
            Stmt::If(ref cond, ref if_stmt, ref else_stmt) => {
                let value = cond.evaluate(env)?;
                if value.is_truthy() {
                    if_stmt.interpret(env)
                } else {
                    if let Some(ref else_stmt) = *else_stmt {
                        else_stmt.interpret(env)
                    } else {
                        Ok(Value::Nil)
                    }
                }
            }
            Stmt::While(ref cond, ref stmt) => {
                while cond.evaluate(env)?.is_truthy() {
                    stmt.interpret(env)?;
                }
                Ok(Value::Nil)
            }
        }
    }
}

