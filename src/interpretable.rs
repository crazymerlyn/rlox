use crate::ast::*;
use crate::errors::Result;
use crate::evaluable::Evaluable;
use crate::interpreter::{EnvRef, Environment};
use std::cell::RefCell;
use std::rc::Rc;

pub trait Interpretable {
    fn interpret(&self, env: EnvRef) -> Result<Value>;
}

impl Interpretable for Stmt {
    fn interpret(&self, env: EnvRef) -> Result<Value> {
        match *self {
            Stmt::Expr(ref expr) => expr.evaluate(env),
            Stmt::Print(ref expr) => {
                match expr.evaluate(env)? {
                    Value::String(s) => println!("{}", s), // Print strings without double quotes
                    x => println!("{}", x),
                }
                Ok(Value::Nil)
            }
            Stmt::Decl(ref id, ref expr) => {
                let value = expr.evaluate(env.clone())?;
                RefCell::borrow_mut(&env).insert(&id.name.lexeme, value);
                Ok(Value::Nil)
            }
            Stmt::Block(ref stmts) => {
                let new_env = Rc::new(RefCell::new(Environment::wrap(env.clone())));
                let mut res = Value::Nil;
                for stmt in stmts {
                    res = stmt.interpret(new_env.clone())?;
                    if let Value::Return(_) = res {
                        return Ok(res);
                    }
                }
                Ok(res)
            }
            Stmt::If(ref cond, ref if_stmt, ref else_stmt) => {
                let value = cond.evaluate(env.clone())?;
                if value.is_truthy() {
                    if_stmt.interpret(env.clone())
                } else if let Some(ref else_stmt) = *else_stmt {
                    else_stmt.interpret(env.clone())
                } else {
                    Ok(Value::Nil)
                }
            }
            Stmt::While(ref cond, ref stmt) => {
                let mut res = Value::Nil;
                while cond.evaluate(env.clone())?.is_truthy() {
                    res = stmt.interpret(env.clone())?;
                    if let Value::Return(_) = res {
                        return Ok(res);
                    }
                }
                Ok(res)
            }
            Stmt::Func(ref name, ref params, ref body) => {
                RefCell::borrow_mut(&env).insert(
                    &name.lexeme,
                    Value::Func(
                        name.to_owned(),
                        env.clone(),
                        params.to_owned(),
                        body.to_owned(),
                    ),
                );
                Ok(Value::Nil)
            }
            Stmt::Return(ref expr) => Ok(Value::Return(Box::new(expr.evaluate(env)?))),
        }
    }
}
