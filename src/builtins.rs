use errors::Result;
use ast::Value;

use std::time::{SystemTime, UNIX_EPOCH};

pub fn clock(_args: Vec<Value>) -> Result<Value> {
    Ok(Value::Number(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as f64))
}
