use crate::scanner;
use scanner::token_type;
use std::fmt;

pub struct Token {
    ttype: token_type::TokenType,
    lexeme: String,
    // We can wrap up literal values in the TokenType enum
    line: usize,
}

impl Token {
    pub fn new(ttype: &token_type::TokenType, lexeme: &String, line: usize) -> Self {
        Token {
            ttype: ttype.clone(),
            lexeme: lexeme.clone(),
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(format!("{}: {} [{}]", self.ttype, self.lexeme, self.line).as_ref())
    }
}
