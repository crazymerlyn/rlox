use ast::*;
use errors::{Result, ErrorKind};

pub trait Evaluable {
    fn evaluate(&self) -> Result<Value>;
}

impl Evaluable for Expr {
    fn evaluate(&self) -> Result<Value> {
        match *self {
            Expr::Literal(ref v) => Ok(v.clone()),
            Expr::Unary(ref u) => u.evaluate(),
            Expr::Binary(ref b) => b.evaluate(),
            Expr::Grouping(ref g) => g.evaluate(),
        }
    }
}

impl Evaluable for UnaryExpr {
    fn evaluate(&self) -> Result<Value> {
        match self.op {
            UnaryOperator::Bang => Ok(Value::Bool(!is_truthy(&self.expr.evaluate()?))),
            UnaryOperator::Minus => match self.expr.evaluate()? {
                Value::Number(n) => Ok(Value::Number(-n)),
                x => Err(ErrorKind::EvaluateError(format!("Can't negate {}", x)).into()),
            }
        }
    }
}

impl Evaluable for BinaryExpr {
    fn evaluate(&self) -> Result<Value> {
        let left = self.left.evaluate()?;
        let right = self.right.evaluate()?;

        match self.op {
            BinaryOperator::Minus | BinaryOperator::Slash | BinaryOperator::Star |
            BinaryOperator::Less | BinaryOperator::Greater | BinaryOperator::LessEqual | BinaryOperator::GreaterEqual => {
                let left = number(left)?;
                let right = number(right)?;
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

fn number(value: Value) -> Result<f64> {
    match value {
        Value::Number(n) => Ok(n),
        _ => Err(ErrorKind::EvaluateError(format!("Expected a number, instead got: {}", value)).into())
    }
}

fn is_truthy(value: &Value) -> bool {
    match *value {
        Value::Nil => false,
        Value::Bool(b) => b,
        _ => true,
    }
}

impl Evaluable for Grouping {
    fn evaluate(&self) -> Result<Value> {
        self.expr.evaluate()
    }
}

