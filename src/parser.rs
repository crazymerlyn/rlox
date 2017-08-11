use scanner::{Token, TokenType};
use errors::{Result, ErrorKind};
use ast::*;

macro_rules! binary_left {
    ($self:ident, $subexpr:ident, $($op:expr),*) => {{
        let mut left = $self.$subexpr()?;
        while $self.match_any(&[$($op,)*]) {
            let op = $self.previous().ty.clone();
            let right = $self.$subexpr()?;
            left = Expr::Binary(Box::new(BinaryExpr{left, op: From::from(op), right}));
        }
        Ok(left)
    }}
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = vec![];
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt> {
        let res = if self.match_any(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };
        match res {
            Ok(stmt) => Ok(stmt),
            e => {
                self.synchronize();
                e
            }
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.".to_string())?;
        let initializer = if self.match_any(&[TokenType::Equal]) {
            self.expression()?
        } else {
            Expr::Literal(Value::Nil)
        };

        if !self.is_at_end() {
            self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.".to_string())?;
        }
        Ok(Stmt::Decl(Identifier { name }, initializer))
    }

    fn statement(&mut self) -> Result<Stmt> {
        if self.match_any(&[TokenType::Print]) {
            self.print_statement()
        } else if self.match_any(&[TokenType::LeftBrace]) {
            self.block()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        if !self.is_at_end() {
            self.consume(TokenType::Semicolon, "Expect ';' after value.".to_string())?;
        }
        Ok(Stmt::Print(expr))
    }

    fn block(&mut self) -> Result<Stmt> {
        let mut stmts = vec![];
        while !self.is_at_end() && !self.check(&TokenType::RightBrace) {
            stmts.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.".to_string())?;
        Ok(Stmt::Block(stmts))
    }

    fn expression_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        if !self.is_at_end() {
            self.consume(TokenType::Semicolon, "Expect ';' after value.".to_string())?;
        }
        Ok(Stmt::Expr(expr))
    }

    fn expression(&mut self) -> Result<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr> {
        let expr = self.equality()?;
        if self.match_any(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;
            match expr {
                Expr::Variable(id) => Ok(Expr::Assign(id, Box::new(value))),
                x => Err(ErrorKind::ParseError(equals, format!("Invalid assignment target: {}", x)).into())
            }
        } else {
            Ok(expr)
        }
    }

    fn equality(&mut self) -> Result<Expr> {
        binary_left!(self, comparison, TokenType::BangEqual, TokenType::EqualEqual)
    }

    fn comparison(&mut self) -> Result<Expr> {
        binary_left!(self, addition, TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual)
    }

    fn addition(&mut self) -> Result<Expr> {
        binary_left!(self, multiplication, TokenType::Minus, TokenType::Plus)
    }

    fn multiplication(&mut self) -> Result<Expr> {
        binary_left!(self, unary, TokenType::Slash, TokenType::Star)
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.match_any(&[TokenType::Minus, TokenType::Bang]) {
            let op = self.previous().ty.clone();
            let expr = self.unary()?;
            return Ok(Expr::Unary(Box::new(UnaryExpr { op: From::from(op), expr })));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr> {
        if self.match_any(&[TokenType::Nil]) {
            return Ok(Expr::Literal(Value::Nil));
        }
        if self.match_any(&[TokenType::False]) {
            return Ok(Expr::Literal(Value::Bool(false)));
        }
        if self.match_any(&[TokenType::True]) {
            return Ok(Expr::Literal(Value::Bool(true)));
        }
        if self.match_any(&[TokenType::Identifier]) {
            return Ok(Expr::Variable(Identifier { name: self.previous().clone() }));
        }
        let ty = self.peek().ty.clone();
        match ty {
            TokenType::Number(n) => {
                self.advance();
                Ok(Expr::Literal(Value::Number(n)))
            }
            TokenType::String(ref s) => {
                self.advance();
                Ok(Expr::Literal(Value::String(s.clone())))
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expect ')' after expression".to_string())?;
                Ok(Expr::Grouping(Box::new(Grouping { expr })))
            }
            _ => {
                Err(ErrorKind::ParseError(self.peek().clone(), "Expect expression".to_string()).into())
            }
        }
    }

    fn synchronize(&mut self) {
        while !self.is_at_end() {
            self.advance();
            if self.previous().ty == TokenType::Semicolon {
                return;
            }
            match self.peek().ty {
                TokenType::Class | TokenType::Fun | TokenType::Var | TokenType::Print |
                TokenType::For | TokenType::If | TokenType::While | TokenType::Return => {
                    return;
                }
                _ => {}
            }
        }
    }

    fn consume(&mut self, ty: TokenType, error: String) -> Result<Token> {
        if self.check(&ty) {
            let result = self.peek().clone();
            self.advance();
            Ok(result)
        } else {
            Err(ErrorKind::ParseError(self.peek().clone(), error).into())
        }
    }

    fn match_any(&mut self, tokens: &[TokenType]) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&mut self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().ty == token_type
        }
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn is_at_end(&self) -> bool {
        self.peek().ty == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
