use crate::ast::*;
use crate::errors::{ErrorKind, Result};
use crate::scanner::{Token, TokenType};

macro_rules! binary_left {
    ($self:ident, $subexpr:ident, $($op:expr),*) => {{
        let mut left = $self.$subexpr()?;
        while $self.match_any(&[$($op,)*]) {
            let op = $self.previous().ty;
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
        } else if self.match_any(&[TokenType::Fun]) {
            self.fun_declaration()
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
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;
        let initializer = if self.match_any(&[TokenType::Equal]) {
            self.expression()?
        } else {
            Expr::Literal(Value::Nil)
        };

        if !self.is_at_end() {
            self.consume(
                TokenType::Semicolon,
                "Expect ';' after variable declaration.",
            )?;
        }
        Ok(Stmt::Decl(Identifier { name }, initializer))
    }

    fn fun_declaration(&mut self) -> Result<Stmt> {
        let name = self.consume(TokenType::Identifier, "Expect function name.")?;
        self.consume(TokenType::LeftParen, "Expect '(' after function name.")?;
        let mut params = vec![];
        if !self.check(&TokenType::RightParen) {
            params.push(self.consume(TokenType::Identifier, "Expect identifier name.")?);
            while self.match_any(&[TokenType::Comma]) {
                params.push(self.consume(TokenType::Identifier, "Expect identifier name.")?);
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters")?;

        self.consume(TokenType::LeftBrace, "Expect '{' before function body")?;
        let block = self.block()?;
        Ok(Stmt::Func(name, params, Box::new(block)))
    }

    fn statement(&mut self) -> Result<Stmt> {
        if self.match_any(&[TokenType::Print]) {
            self.print_statement()
        } else if self.match_any(&[TokenType::LeftBrace]) {
            self.block()
        } else if self.match_any(&[TokenType::While]) {
            self.while_stmt()
        } else if self.match_any(&[TokenType::Return]) {
            self.return_stmt()
        } else if self.match_any(&[TokenType::For]) {
            self.for_stmt()
        } else if self.match_any(&[TokenType::If]) {
            self.if_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        if !self.is_at_end() {
            self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        }
        Ok(Stmt::Print(expr))
    }

    fn block(&mut self) -> Result<Stmt> {
        let mut stmts = vec![];
        while !self.is_at_end() && !self.check(&TokenType::RightBrace) {
            stmts.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(Stmt::Block(stmts))
    }

    fn while_stmt(&mut self) -> Result<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let cond = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after while condition.")?;
        let stmt = self.statement()?;
        Ok(Stmt::While(cond, Box::new(stmt)))
    }

    fn return_stmt(&mut self) -> Result<Stmt> {
        let expr = if self.check(&TokenType::Semicolon) {
            Expr::Literal(Value::Nil)
        } else {
            self.expression()?
        };

        self.consume(TokenType::Semicolon, "Exprect ';' after return statement.")?;
        Ok(Stmt::Return(expr))
    }

    fn for_stmt(&mut self) -> Result<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;
        let initializer = if self.match_any(&[TokenType::Semicolon]) {
            None
        } else if self.match_any(&[TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };
        let cond = if !self.check(&TokenType::Semicolon) {
            self.expression()?
        } else {
            Expr::Literal(Value::Bool(true))
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let inc = if !self.check(&TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;
        let mut body = self.statement()?;

        if let Some(inc) = inc {
            body = Stmt::Block(vec![body, Stmt::Expr(inc)]);
        }

        body = Stmt::While(cond, Box::new(body));

        if let Some(init) = initializer {
            body = Stmt::Block(vec![init, body]);
        }

        Ok(body)
    }

    fn if_statement(&mut self) -> Result<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let cond = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let if_branch = self.statement()?;
        let else_branch = if self.match_any(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        Ok(Stmt::If(cond, Box::new(if_branch), else_branch))
    }

    fn expression_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        if !self.is_at_end() {
            self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        }
        Ok(Stmt::Expr(expr))
    }

    fn expression(&mut self) -> Result<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr> {
        let expr = self.or_expr()?;
        if self.match_any(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;
            match expr {
                Expr::Variable(id) => Ok(Expr::Assign(id, Box::new(value))),
                x => Err(ErrorKind::ParseError {
                    tok: equals,
                    t: format!("Invalid assignment target: {}", x),
                }),
            }
        } else {
            Ok(expr)
        }
    }

    fn or_expr(&mut self) -> Result<Expr> {
        let mut left = self.and_expr()?;

        while self.match_any(&[TokenType::Or]) {
            let op = self.previous().ty;
            let right = self.and_expr()?;
            left = Expr::Logical(Box::new(LogicalExpr {
                left,
                op: op.into(),
                right,
            }));
        }
        Ok(left)
    }

    fn and_expr(&mut self) -> Result<Expr> {
        let mut left = self.equality()?;

        while self.match_any(&[TokenType::And]) {
            let op = self.previous().ty;
            let right = self.equality()?;
            left = Expr::Logical(Box::new(LogicalExpr {
                left,
                op: op.into(),
                right,
            }));
        }
        Ok(left)
    }

    fn equality(&mut self) -> Result<Expr> {
        binary_left!(
            self,
            comparison,
            TokenType::BangEqual,
            TokenType::EqualEqual
        )
    }

    fn comparison(&mut self) -> Result<Expr> {
        binary_left!(
            self,
            addition,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual
        )
    }

    fn addition(&mut self) -> Result<Expr> {
        binary_left!(self, multiplication, TokenType::Minus, TokenType::Plus)
    }

    fn multiplication(&mut self) -> Result<Expr> {
        binary_left!(self, unary, TokenType::Slash, TokenType::Star)
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.match_any(&[TokenType::Minus, TokenType::Bang]) {
            let op = self.previous().ty;
            let expr = self.unary()?;
            return Ok(Expr::Unary(Box::new(UnaryExpr {
                op: From::from(op),
                expr,
            })));
        }

        self.call()
    }

    fn call(&mut self) -> Result<Expr> {
        let mut expr = self.primary()?;

        loop {
            if self.match_any(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, expr: Expr) -> Result<Expr> {
        let mut args = vec![];
        if !self.check(&TokenType::RightParen) {
            args.push(self.expression()?);
            while self.match_any(&[TokenType::Comma]) {
                args.push(self.expression()?);
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after arguments")?;

        Ok(Expr::Call(Box::new(expr), args))
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
            return Ok(Expr::Variable(Identifier {
                name: self.previous().clone(),
            }));
        }
        let token = self.peek();
        match token.ty {
            TokenType::Number(n) => {
                self.advance();
                Ok(Expr::Literal(Value::Number(n)))
            }
            TokenType::String => {
                // Ignore double quotes at start and end
                let s = token.lexeme[1..token.lexeme.len() - 1].to_string();
                self.advance();
                Ok(Expr::Literal(Value::String(s)))
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expect ')' after expression")?;
                Ok(Expr::Grouping(Box::new(Grouping { expr })))
            }
            _ => Err(ErrorKind::ParseError {
                tok: self.peek().clone(),
                t: "Expect expression".to_string(),
            }),
        }
    }

    fn synchronize(&mut self) {
        while !self.is_at_end() {
            self.advance();
            if self.previous().ty == TokenType::Semicolon {
                return;
            }
            match self.peek().ty {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::Print
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Return => {
                    return;
                }
                _ => {}
            }
        }
    }

    fn consume(&mut self, ty: TokenType, error: &str) -> Result<Token> {
        if self.check(&ty) {
            let result = self.peek().clone();
            self.advance();
            Ok(result)
        } else {
            Err(ErrorKind::ParseError {
                tok: self.peek().clone(),
                t: error.to_string(),
            })
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
        &self.peek().ty == token_type
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
