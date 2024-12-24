use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::*;
use crate::errors::{ErrorKind, Result};
use crate::interpretable::Interpretable;
use crate::interpreter::{EnvRef, Environment};

pub trait Callable {
    fn call(&self, env: EnvRef, args: Vec<Value>) -> Result<Value>;
}

impl Callable for Value {
    fn call(&self, _env: EnvRef, args: Vec<Value>) -> Result<Value> {
        match *self {
            Value::BuiltinFunc(_, ref arity, ref func) => {
                if *arity != args.len() {
                    Err(ErrorKind::EvaluateError(format!(
                        "Wrong number of arguments: Expected {}, got {}",
                        arity,
                        args.len()
                    )))
                } else {
                    func(args)
                }
            }
            Value::Func(_, ref closure, ref params, ref block) => {
                if params.len() != args.len() {
                    Err(ErrorKind::EvaluateError(format!(
                        "Wrong number of arguments: Expected {}, got {}",
                        params.len(),
                        args.len()
                    )))
                } else {
                    let funcenv = Rc::new(RefCell::new(Environment::wrap(closure.clone())));
                    for (param, value) in params.iter().zip(args.into_iter()) {
                        funcenv.borrow_mut().insert(&param.lexeme, value);
                    }
                    let res = block.interpret(funcenv);

                    match res {
                        Ok(Value::Return(x)) => Ok(*x),
                        _ => res,
                    }
                }
            }
            _ => Err(ErrorKind::EvaluateError(format!(
                "{} is not a valid function",
                self
            ))),
        }
    }
}
