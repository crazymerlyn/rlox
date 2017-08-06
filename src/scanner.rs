use std::collections::HashMap;

use errors::{ErrorKind, Result};


lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut k = HashMap::new();
        k.insert("and".to_string(), TokenType::AND);
        k.insert("class".to_string(), TokenType::CLASS);
        k.insert("else".to_string(), TokenType::ELSE);
        k.insert("false".to_string(), TokenType::FALSE);
        k.insert("for".to_string(), TokenType::FOR);
        k.insert("fun".to_string(), TokenType::FUN);
        k.insert("if".to_string(), TokenType::IF);
        k.insert("nil".to_string(), TokenType::NIL);
        k.insert("or".to_string(), TokenType::OR);
        k.insert("print".to_string(), TokenType::PRINT);
        k.insert("return".to_string(), TokenType::RETURN);
        k.insert("super".to_string(), TokenType::SUPER);
        k.insert("this".to_string(), TokenType::THIS);
        k.insert("true".to_string(), TokenType::TRUE);
        k.insert("var".to_string(), TokenType::VAR);
        k.insert("while".to_string(), TokenType::WHILE);
        k
    };
}

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single character tokens
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,
    
    // One or two character tokens
    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,

    // Literals
    IDENTIFIER, STRING(String), NUMBER(f64),

    // Keywords
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF

}

#[derive(Debug)]
pub struct Token {
    ty: TokenType,
    lexeme: String,
    line: usize,
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
        self.tokens.push(Token::new(TokenType::EOF, "", self.line));
        Ok(self.tokens)
    }

    fn scan_token(&mut self) -> Result<()> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                let token = if self.match_next('=') { TokenType::BANG_EQUAL } else { TokenType::BANG };
                self.add_token(token);
            }
            '=' => {
                let token = if self.match_next('=') { TokenType::EQUAL_EQUAL } else { TokenType::EQUAL };
                self.add_token(token);
            }
            '<' => {
                let token = if self.match_next('=') { TokenType::LESS_EQUAL } else { TokenType::LESS };
                self.add_token(token);
            }
            '>' => {
                let token = if self.match_next('=') { TokenType::GREATER_EQUAL } else { TokenType::GREATER };
                self.add_token(token);
            }
            '/' => {
                if self.match_next('/') {
                    while self.peek() != Some('\n') && !self.is_at_end() { self.advance(); }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }
            '"' => self.string()?,
            c if is_digit(c) => self.number()?,
            c if is_alpha(c) => self.identifier(),
            ' ' | '\t' | '\r' => {},
            '\n' => self.line += 1,
            _ => return Err(ErrorKind::ScanError(self.line, "").into()),
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
            self.error("Unterminated string.")?;
        }

        self.advance(); // Closing "

        self.add_token(TokenType::STRING(string));
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
        self.add_token(TokenType::NUMBER(s.parse().unwrap()));
        Ok(())
    }

    fn identifier(&mut self) {
        while is_alpha_numeric(self.peek().unwrap_or(' ')) { self.advance(); }

        let text = self.src[self.start..self.current].to_string();
        self.add_token(KEYWORDS.get(&text).cloned().unwrap_or(TokenType::IDENTIFIER));
    }

    fn error(&self, s: &'static str) -> Result<()> {
        return Err(From::from(ErrorKind::ScanError(self.line, s)));
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

    fn is_at_end(&mut self) -> bool {
        match self.peek() {
            Some(_) => false,
            _ => true
        }
    }

    fn match_next(&mut self, expected: char) -> bool {
        let next_char = self.peek();
        if Some(expected) == next_char {
            self.current += 1;
            return true;
        }
        return false;
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

