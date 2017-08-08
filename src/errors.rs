#![allow(unknown_lints)]

use scanner::Token;

error_chain! {
    errors {
        ScanError(line: usize, t: String) {
            description("Invalid syntax")
            display("Error at line {}: {}", line, t)
        }
        ParseError(tok: Token, t: String) {
            description("Invalid program")
            display("Error at line {} at '{}': {}", tok.line, tok.lexeme, t)
        }
    }
    foreign_links {
        IO(::std::io::Error);
    }
}
