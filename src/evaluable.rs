use ast::*;
use errors::{Result, ErrorKind};
use interpreter::Environment;
use callable::Callable;

pub trait Evaluable {
    fn evaluate(&self, env: &mut Environment) -> Result<Value>;
}

impl Evaluable for Expr {
    fn evaluate(&self, env: &mut Environment) -> Result<Value> {
        match *self {
            Expr::Literal(ref v) => Ok(v.clone()),
            Expr::Unary(ref u) => u.evaluate(env),
            Expr::Binary(ref b) => b.evaluate(env),
            Expr::Logical(ref l) => l.evaluate(env),
            Expr::Grouping(ref g) => g.evaluate(env),
            Expr::Variable(ref id) => {
                match env.get(&id.name.lexeme) {
                    Some(v) => Ok(v.clone()),
                    None => Err(ErrorKind::EvaluateError(format!("Undefined variable: {}", id.name.lexeme)).into()),
                }
            },
            Expr::Assign(ref id, ref e) => {
                if env.get(&id.name.lexeme) == None {
                    Err(ErrorKind::EvaluateError(format!("Undefined variable: {}", id.name.lexeme)).into())
                } else {
                    let value = e.evaluate(env)?;
                    env.update(id.name.lexeme.clone(), value.clone());
                    Ok(value)
                }
            }
            Expr::Call(ref expr, ref args) => {
                let func = expr.evaluate(env)?;
                let mut values = vec![];
                for arg in args {
                    values.push(arg.evaluate(env)?)
                }
                func.call(env, values)
            }
        }
    }
}

impl Evaluable for UnaryExpr {
    fn evaluate(&self, env: &mut Environment) -> Result<Value> {
        match self.op {
            UnaryOperator::Bang => Ok(Value::Bool(!self.expr.evaluate(env)?.is_truthy())),
            UnaryOperator::Minus => match self.expr.evaluate(env)? {
                Value::Number(n) => Ok(Value::Number(-n)),
                x => Err(ErrorKind::EvaluateError(format!("Can't negate {}", x)).into()),
            }
        }
    }
}

impl Evaluable for BinaryExpr {
    fn evaluate(&self, env: &mut Environment) -> Result<Value> {
        let left = self.left.evaluate(env)?;
        let right = self.right.evaluate(env)?;

        match self.op {
            BinaryOperator::Minus | BinaryOperator::Slash | BinaryOperator::Star |
            BinaryOperator::Less | BinaryOperator::Greater | BinaryOperator::LessEqual | BinaryOperator::GreaterEqual => {
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
                        Err(ErrorKind::EvaluateError(format!("Can't add {} to a number", right)).into())
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
                    Err(ErrorKind::EvaluateError(format!("Can't add {} and {}", left, right)).into())
                }
            }
            BinaryOperator::EqualEqual => Ok(Value::Bool(left == right)),
            BinaryOperator::BangEqual => Ok(Value::Bool(left != right)),
            BinaryOperator::Equal => Ok(Value::Nil),
        }
    }
}

impl Evaluable for LogicalExpr {
    fn evaluate(&self, env: &mut Environment) -> Result<Value> {
        let left = self.left.evaluate(env)?;
        if self.op == LogicalOperator::Or {
            if left.is_truthy() { return Ok(left); }
        } else if !left.is_truthy() {
            return Ok(left); 
        }
        self.right.evaluate(env)
    }
}

fn number(value: &Value) -> Result<f64> {
    match *value {
        Value::Number(n) => Ok(n),
        _ => Err(ErrorKind::EvaluateError(format!("Expected a number, instead got: {}", value)).into())
    }
}

impl Evaluable for Grouping {
    fn evaluate(&self, env: &mut Environment) -> Result<Value> {
        self.expr.evaluate(env)
    }
}

