use crate::scanner::Token;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("Error at line {0}: {1}")]
    ScanError(usize, String),
    #[error("Error at line {} at '{}': {t}", tok.line, tok.lexeme)]
    ParseError { tok: Token, t: String },
    #[error("Error: {0}")]
    EvaluateError(String),
    #[error("IO Error: {0}")]
    IO(#[from] ::std::io::Error),
}

pub type Result<T> = std::result::Result<T, ErrorKind>;
