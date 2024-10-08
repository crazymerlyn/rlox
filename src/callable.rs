use crate::ast::*;
use crate::errors::{ErrorKind, Result};
use crate::interpretable::Interpretable;
use crate::interpreter::Environment;

pub trait Callable {
    fn call(&self, env: &mut Environment, args: Vec<Value>) -> Result<Value>;
}

impl Callable for Value {
    fn call(&self, env: &mut Environment, args: Vec<Value>) -> Result<Value> {
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
            Value::Func(_, ref params, ref block) => {
                if params.len() != args.len() {
                    Err(ErrorKind::EvaluateError(format!(
                        "Wrong number of arguments: Expected {}, got {}",
                        params.len(),
                        args.len()
                    ))
                    .into())
                } else {
                    let non_globals = env.export_non_globals();
                    env.push_local_scope();
                    for (param, value) in params.iter().zip(args.into_iter()) {
                        env.insert(&param.lexeme, value);
                    }
                    let res = block.interpret(env);
                    env.import_non_globals(non_globals);

                    match res {
                        Ok(Value::Return(x)) => Ok(*x),
                        _ => res,
                    }
                }
            }
            _ => Err(ErrorKind::EvaluateError(format!("{} is not a valid function", self)).into()),
        }
    }
}
