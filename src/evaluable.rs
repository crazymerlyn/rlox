use crate::ast::*;
use crate::callable::Callable;
use crate::errors::{ErrorKind, Result};
use crate::interpreter::EnvRef;

pub trait Evaluable {
    fn evaluate(&self, env: EnvRef) -> Result<Value>;
}

impl Evaluable for Expr {
    fn evaluate(&self, env: EnvRef) -> Result<Value> {
        match *self {
            Expr::Literal(ref v) => Ok(v.clone()),
            Expr::Unary(ref u) => u.evaluate(env),
            Expr::Binary(ref b) => b.evaluate(env),
            Expr::Logical(ref l) => l.evaluate(env),
            Expr::Grouping(ref g) => g.evaluate(env),
            Expr::Variable(ref id) => env
                .borrow()
                .get(&id.name.lexeme)
                .map(|v| v.borrow().clone())
                .ok_or(ErrorKind::EvaluateError(format!(
                    "Undefined variable: {}",
                    id.name.lexeme
                ))),
            Expr::Assign(ref id, ref e) => {
                let value = e.evaluate(env.clone())?;
                env.borrow_mut()
                    .update(&id.name.lexeme, value)
                    .ok_or(ErrorKind::EvaluateError(format!(
                        "Undefined variable: {}",
                        id.name.lexeme
                    )))
            }
            Expr::Call(ref expr, ref args) => {
                let func = expr.evaluate(env.clone())?;
                let mut values = vec![];
                for arg in args {
                    values.push(arg.evaluate(env.clone())?)
                }
                func.call(env, values)
            }
        }
    }
}

impl Evaluable for UnaryExpr {
    fn evaluate(&self, env: EnvRef) -> Result<Value> {
        match self.op {
            UnaryOperator::Bang => Ok(Value::Bool(!self.expr.evaluate(env)?.is_truthy())),
            UnaryOperator::Minus => match self.expr.evaluate(env)? {
                Value::Number(n) => Ok(Value::Number(-n)),
                x => Err(ErrorKind::EvaluateError(format!("Can't negate {}", x))),
            },
        }
    }
}

impl Evaluable for BinaryExpr {
    fn evaluate(&self, env: EnvRef) -> Result<Value> {
        let left = self.left.evaluate(env.clone())?;
        let right = self.right.evaluate(env.clone())?;

        match self.op {
            BinaryOperator::Minus
            | BinaryOperator::Slash
            | BinaryOperator::Star
            | BinaryOperator::Less
            | BinaryOperator::Greater
            | BinaryOperator::LessEqual
            | BinaryOperator::GreaterEqual => {
                let left = number(&left)?;
                let right = number(&right)?;
                let value = match self.op {
                    BinaryOperator::Minus => Value::Number(left - right),
                    BinaryOperator::Star => Value::Number(left * right),
                    BinaryOperator::Slash => Value::Number(left / right),
                    BinaryOperator::Less => Value::Bool(left < right),
                    BinaryOperator::LessEqual => Value::Bool(left <= right),
                    BinaryOperator::Greater => Value::Bool(left > right),
                    BinaryOperator::GreaterEqual => Value::Bool(left >= right),
                    _ => Value::Nil,
                };
                Ok(value)
            }
            BinaryOperator::Plus => {
                if let Value::Number(l) = left {
                    if let Value::Number(r) = right {
                        Ok(Value::Number(l + r))
                    } else if let Value::String(r) = right {
                        Ok(Value::String(format!("{}{}", l, r)))
                    } else {
                        Err(ErrorKind::EvaluateError(format!(
                            "Can't add {} to a number",
                            right
                        )))
                    }
                } else if let Value::String(l) = left {
                    if let Value::String(r) = right {
                        Ok(Value::String(l + &r))
                    } else if let Value::Number(r) = right {
                        Ok(Value::String(format!("{}{}", l, r)))
                    } else {
                        Ok(Value::String(format!("{}{}", l, right)))
                    }
                } else if let Value::String(r) = right {
                    Ok(Value::String(format!("{}{}", left, r)))
                } else {
                    Err(ErrorKind::EvaluateError(format!(
                        "Can't add {} and {}",
                        left, right
                    )))
                }
            }
            BinaryOperator::EqualEqual => Ok(Value::Bool(left == right)),
            BinaryOperator::BangEqual => Ok(Value::Bool(left != right)),
            BinaryOperator::Equal => Ok(Value::Nil),
        }
    }
}

impl Evaluable for LogicalExpr {
    fn evaluate(&self, env: EnvRef) -> Result<Value> {
        let left = self.left.evaluate(env.clone())?;
        if self.op == LogicalOperator::Or {
            if left.is_truthy() {
                return Ok(left);
            }
        } else if !left.is_truthy() {
            return Ok(left);
        }
        self.right.evaluate(env.clone())
    }
}

fn number(value: &Value) -> Result<f64> {
    match *value {
        Value::Number(n) => Ok(n),
        _ => Err(ErrorKind::EvaluateError(format!(
            "Expected a number, instead got: {}",
            value
        ))),
    }
}

impl Evaluable for Grouping {
    fn evaluate(&self, env: EnvRef) -> Result<Value> {
        self.expr.evaluate(env)
    }
}
