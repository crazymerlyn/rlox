use std::collections::HashMap;

use errors::{ErrorKind, Result};


lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut k = HashMap::new();
        k.insert("and".to_string(), TokenType::And);
        k.insert("class".to_string(), TokenType::Class);
        k.insert("else".to_string(), TokenType::Else);
        k.insert("false".to_string(), TokenType::False);
        k.insert("for".to_string(), TokenType::For);
        k.insert("fun".to_string(), TokenType::Fun);
        k.insert("if".to_string(), TokenType::If);
        k.insert("nil".to_string(), TokenType::Nil);
        k.insert("or".to_string(), TokenType::Or);
        k.insert("print".to_string(), TokenType::Print);
        k.insert("return".to_string(), TokenType::Return);
        k.insert("super".to_string(), TokenType::Super);
        k.insert("this".to_string(), TokenType::This);
        k.insert("true".to_string(), TokenType::True);
        k.insert("var".to_string(), TokenType::Var);
        k.insert("while".to_string(), TokenType::While);
        k
    };
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    
    // One or two character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals
    Identifier, String(String), Number(f64),

    // Keywords
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Eof

}

#[derive(Debug, Clone)]
pub struct Token {
    pub ty: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new<S: AsRef<str>>(ty: TokenType, lexeme: S, line: usize) -> Token {
        Token { ty, lexeme: lexeme.as_ref().to_string(), line }
    }
}

pub struct Scanner {
    src: String,
    tokens: Vec<Token>,
    line: usize,
    start: usize,
    current: usize,
}

impl Scanner {
    pub fn new<S: AsRef<str>>(s: S) -> Scanner {
        Scanner { src: s.as_ref().to_owned(), tokens: vec![], line: 1, start: 0, current: 0 }
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        self.tokens.push(Token::new(TokenType::Eof, "Eof", self.line));
        Ok(self.tokens)
    }

    fn scan_token(&mut self) -> Result<()> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let token = if self.match_next('=') { TokenType::BangEqual } else { TokenType::Bang };
                self.add_token(token);
            }
            '=' => {
                let token = if self.match_next('=') { TokenType::EqualEqual } else { TokenType::Equal };
                self.add_token(token);
            }
            '<' => {
                let token = if self.match_next('=') { TokenType::LessEqual } else { TokenType::Less };
                self.add_token(token);
            }
            '>' => {
                let token = if self.match_next('=') { TokenType::GreaterEqual } else { TokenType::Greater };
                self.add_token(token);
            }
            '/' => {
                if self.match_next('/') {
                    while self.peek() != Some('\n') && !self.is_at_end() { self.advance(); }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            '"' => self.string()?,
            c if is_digit(c) => self.number()?,
            c if is_alpha(c) => self.identifier(),
            ' ' | '\t' | '\r' => {},
            '\n' => self.line += 1,
            _ => return Err(ErrorKind::ScanError(self.line, format!("Unexpected character: {}", c)).into()),
        }
        Ok(())
    }

    fn string(&mut self) -> Result<()> {
        let mut string = "".to_string();
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') { self.line += 1; }
            string.push(self.advance());
        }

        if self.is_at_end() {
            self.error("Unterminated string.".to_string())?;
        }

        self.advance(); // Closing "

        self.add_token(TokenType::String(string));
        Ok(())
    }

    fn number(&mut self) -> Result<()> {
        while is_digit(self.peek().unwrap_or(' ')) {
            self.advance();
        }
        if self.peek() == Some('.') && is_digit(self.peek_next().unwrap_or(' ')) {
            self.advance();
            while is_digit(self.peek().unwrap_or(' ')) {
                self.advance();
            }
        }
        let s = self.src[self.start..self.current].to_string();
        self.add_token(TokenType::Number(s.parse().unwrap()));
        Ok(())
    }

    fn identifier(&mut self) {
        while is_alpha_numeric(self.peek().unwrap_or(' ')) { self.advance(); }

        let text = self.src[self.start..self.current].to_string();
        self.add_token(KEYWORDS.get(&text).cloned().unwrap_or(TokenType::Identifier));
    }

    fn error(&self, s: String) -> Result<()> {
        Err(From::from(ErrorKind::ScanError(self.line, s)))
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.src.chars().nth(self.current - 1).unwrap()
    }

    fn peek(&self) -> Option<char> {
        self.src.chars().nth(self.current)
    }

    fn peek_next(&self) -> Option<char> {
        self.src.chars().nth(self.current+1)
    }

    fn is_at_end(&self) -> bool {
        match self.peek() {
            Some(_) => false,
            _ => true
        }
    }

    fn match_next(&mut self, expected: char) -> bool {
        let next_char = self.peek();
        if Some(expected) == next_char {
            self.current += 1;
            true
        } else {
            false
        }
    }

    fn add_token(&mut self, ty: TokenType) {
        let s = self.src[self.start..self.current].to_string();
        self.tokens.push(Token::new(ty, s, self.line));
    }
}

fn is_digit(c: char) -> bool {
    '0' <= c && c <= '9'
}

fn is_alpha(c: char) -> bool {
    'a' <= c && c <= 'z' || 'A' <= c && c <= 'Z' || c == '_'
}

fn is_alpha_numeric(c: char) -> bool {
    is_digit(c) || is_alpha(c)
}

