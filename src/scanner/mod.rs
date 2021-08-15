use self::token::Token;
use crate::{ascii::AsciiStr, lox_error::LoxError};
pub mod token;
pub mod token_type;

pub struct Scanner<'a> {
    start: usize,
    current: usize,
    line: usize,

    tokens: Vec<Token>,
    source: &'a AsciiStr,
}

#[allow(unused)]
impl<'a> Scanner<'a> {
    pub fn new(program: &'a String) -> Result<Scanner<'a>, LoxError> {
        // Probably should check that we only have Ascii here since we
        // assume that later.
        let test = AsciiStr::from_ascii(program);
        let ascii_str = match test {
            Err(_) => return Err(LoxError::new_text_only("Program should be in ascii")),
            Ok(a) => a,
        };
        let scanner = Scanner {
            start: 0,
            current: 0,
            line: 1,
            source: ascii_str,
            tokens: vec![
                Token::new(
                    &token_type::TokenType::String("Darrell".to_string()),
                    &"Darrell".to_string(),
                    1,
                ),
                Token::new(
                    &token_type::TokenType::String("Alan".to_string()),
                    &"Alan".to_string(),
                    1,
                ),
                Token::new(
                    &token_type::TokenType::String("Plank".to_string()),
                    &"Plank".to_string(),
                    1,
                ),
            ],
        };
        Ok(scanner)
    }

    pub fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn add_token_type(&mut self, tt: &token_type::TokenType) {
        self.add_token(Token::new(
            tt,
            &token_type::tt_to_string(tt).to_string(),
            self.line,
        ))
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.add_token(Token::new(
            &token_type::TokenType::Eof,
            &"".to_string(),
            self.line,
        ));
        &self.tokens
    }

    pub fn scan_token(&mut self) -> () {
        let c = self.advance();
        match c {
            '(' => self.add_token_type(&token_type::TokenType::LeftParen),
            ')' => self.add_token_type(&token_type::TokenType::RightParen),
            '{' => self.add_token_type(&token_type::TokenType::LeftBrace),
            '}' => self.add_token_type(&token_type::TokenType::RightBrace),
            ',' => self.add_token_type(&token_type::TokenType::Comma),
            '.' => self.add_token_type(&token_type::TokenType::Dot),
            '-' => self.add_token_type(&token_type::TokenType::Minus),
            '+' => self.add_token_type(&token_type::TokenType::Plus),
            ';' => self.add_token_type(&token_type::TokenType::Semicolon),
            '*' => self.add_token_type(&token_type::TokenType::Star),
            _ => (),
        }
    }

    pub fn advance(&mut self) -> char {
        let old_index = self.current;
        self.current += 1;
        self.source[old_index].as_char()
    }
}
