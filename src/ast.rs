use scanner::{TokenType, Token};
use std::convert::From;
use std::fmt;

#[derive(Debug, Clone)]
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

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnaryOperator::Bang => write!(f, "!"),
            UnaryOperator::Minus => write!(f, "-"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Equal,
    EqualEqual,
    //Bang,
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
            //TokenType::Bang => BinaryOperator::Bang,
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

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BinaryOperator::Equal => write!(f, "="),
            BinaryOperator::EqualEqual => write!(f, "=="),
            //BinaryOperator::Bang => write!(f, "!"),
            BinaryOperator::BangEqual => write!(f, "!="),
            BinaryOperator::Less => write!(f, "<"),
            BinaryOperator::LessEqual => write!(f, "<="),
            BinaryOperator::Greater => write!(f, ">"),
            BinaryOperator::GreaterEqual => write!(f, ">="),
            BinaryOperator::Minus => write!(f, "-"),
            BinaryOperator::Plus => write!(f, "+"),
            BinaryOperator::Slash => write!(f, "/"),
            BinaryOperator::Star => write!(f, "*"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub op: UnaryOperator,
    pub expr: Expr,
}

impl fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.op, self.expr)
    }
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub left: Expr,
    pub op: BinaryOperator,
    pub right: Expr,
}

impl fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {} {})", self.op, self.left, self.right)
    }
}

#[derive(Debug, Clone)]
pub struct Grouping {
    pub expr: Expr,
}

impl fmt::Display for Grouping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.expr)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(ref s) => write!(f, "\"{}\"", s),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: Token,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Value),
    Unary(Box<UnaryExpr>),
    Binary(Box<BinaryExpr>),
    Grouping(Box<Grouping>),
    Variable(Identifier),
    Assign(Identifier, Box<Expr>),
}


impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expr::Literal(ref v) => write!(f, "{}", v),
            Expr::Unary(ref v) => write!(f, "{}", v),
            Expr::Binary(ref v) => write!(f, "{}", v),
            Expr::Grouping(ref v) => write!(f, "{}", v),
            Expr::Variable(ref v) => write!(f, "{}", v.name.lexeme),
            Expr::Assign(ref id, ref v) => write!(f, "{} = {}", id.name.lexeme, v),
        }
    }
}

pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Decl(Identifier, Expr),
}

