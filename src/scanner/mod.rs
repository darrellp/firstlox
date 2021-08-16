use self::token::Token;
use self::token_type::TokenType;
use crate::{ascii::AsciiStr, lox_error::LoxError, lox_error::LoxErrorList};
pub mod token;
pub mod token_type;

pub struct Scanner<'a> {
    start: usize,
    current: usize,
    line: usize,

    tokens: Vec<Token>,
    errors: LoxErrorList,
    source: &'a AsciiStr,
}

#[allow(unused)]
impl<'a> Scanner<'a> {
    pub fn new(program: &'a String) -> Result<Scanner<'a>, LoxError> {
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
            tokens: vec![],
            errors: LoxErrorList::new(),
        };
        Ok(scanner)
    }

    pub fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn add_token_type(&mut self, tt: &TokenType) {
        self.add_token(Token::new(tt, &tt.to_stringslice().to_string(), self.line))
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    pub fn get_errors(&self) -> LoxErrorList {
        self.errors.clone()
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_tokens(&mut self) {
        let mut errors = LoxErrorList::new();

        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.add_token(Token::new(&TokenType::Eof, &"".to_string(), self.line));
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            // unambiguous single characters
            '(' => self.add_token_type(&TokenType::LeftParen),
            ')' => self.add_token_type(&TokenType::RightParen),
            '{' => self.add_token_type(&TokenType::LeftBrace),
            '}' => self.add_token_type(&TokenType::RightBrace),
            ',' => self.add_token_type(&TokenType::Comma),
            '.' => self.add_token_type(&TokenType::Dot),
            '-' => self.add_token_type(&TokenType::Minus),
            '+' => self.add_token_type(&TokenType::Plus),
            ';' => self.add_token_type(&TokenType::Semicolon),
            '*' => self.add_token_type(&TokenType::Star),

            // Two letter combos ending with '='
            '!' => {
                let tt = if self.match_ch('=') {
                    &TokenType::BangEqual
                } else {
                    &TokenType::Bang
                };
                self.add_token_type(tt);
            }
            '=' => {
                let tt = if self.match_ch('=') {
                    &TokenType::EqualEqual
                } else {
                    &TokenType::Equal
                };
                self.add_token_type(tt);
            }
            '<' => {
                let tt = if self.match_ch('=') {
                    &TokenType::LessEqual
                } else {
                    &TokenType::Less
                };
                self.add_token_type(tt);
            }
            '>' => {
                let tt = if self.match_ch('=') {
                    &TokenType::GreaterEqual
                } else {
                    &TokenType::Greater
                };
                self.add_token_type(tt);
            }
            '/' => {
                if self.match_ch('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token_type(&TokenType::Slash);
                }
            }

            // White Space
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,

            // Strings
            '"' => self.scan_string(),

            // Numbers
            '0'..='9' => self.scan_number(c),

            // Everything else
            _ => {
                self.errors.push(LoxError::new(
                    self.line,
                    "Unexpected character.".to_string(),
                ));
            }
        };
    }

    fn scan_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }

        if self.is_at_end() {
            self.errors
                .push(LoxError::new_text_only("Unterminated string."));
            return;
        }

        // Terminating double quote
        self.advance();

        let text = &self.source[self.start + 1..self.current - 1].to_string();
        let token = Token::new(
            &TokenType::String(text.clone()),
            &format!("{}{}{}", '"', text, '"'),
            self.line,
        );
        self.add_token(token)
    }

    fn scan_number(&mut self, init: char) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            //consume the decimal point
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        let text = self.source[self.start..self.current].to_string();
        let token = Token::new(&TokenType::Number(text.clone()), &text, self.line);
        self.add_token(token);
    }

    fn advance(&mut self) -> char {
        let old_index = self.current;
        self.current += 1;
        self.source[old_index].as_char()
    }

    fn match_ch(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let ch = self.source[self.current].as_char();
        if (ch != expected) {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current].as_char()
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1].as_char()
        }
    }
}
