use scanner::TokenType;
use std::convert::From;

#[derive(Debug)]
pub enum UnaryOperator {
    Bang,
    Minus,
}

impl From<TokenType> for UnaryOperator {
    fn from(token: TokenType) -> UnaryOperator {
        match token {
            TokenType::Bang => UnaryOperator::Bang,
            TokenType::Minus => UnaryOperator::Minus,
            _ => panic!("Invalid unary operator {:?}", token)
        }
    }
}

#[derive(Debug)]
pub enum BinaryOperator {
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Minus,
    Plus,
    Star,
    Slash,
}

impl From<TokenType> for BinaryOperator {
    fn from(token: TokenType) -> BinaryOperator {
        match token {
            TokenType::Equal => BinaryOperator::Equal,
            TokenType::EqualEqual => BinaryOperator::EqualEqual,
            TokenType::Bang => BinaryOperator::Bang,
            TokenType::BangEqual => BinaryOperator::BangEqual,
            TokenType::Less => BinaryOperator::Less,
            TokenType::LessEqual => BinaryOperator::LessEqual,
            TokenType::Greater => BinaryOperator::Greater,
            TokenType::GreaterEqual => BinaryOperator::GreaterEqual,
            TokenType::Minus => BinaryOperator::Minus,
            TokenType::Plus => BinaryOperator::Plus,
            TokenType::Slash => BinaryOperator::Slash,
            TokenType::Star => BinaryOperator::Star,
            _ => panic!("Invalid binary operator {:?}", token)
        }
    }
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub op: UnaryOperator,
    pub expr: Expr,
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Expr,
    pub op: BinaryOperator,
    pub right: Expr,
}

#[derive(Debug)]
pub struct Grouping {
    pub expr: Expr,
}

#[derive(Debug)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
}

#[derive(Debug)]
pub enum Expr {
    Literal(Value),
    Unary(Box<UnaryExpr>),
    Binary(Box<BinaryExpr>),
    Grouping(Box<Grouping>),
}
