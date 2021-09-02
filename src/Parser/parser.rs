use crate::lox_error;
use crate::parser;
use crate::scanner;
use lox_error::lox_error::LoxError;
use parser::struct_macros::{binary, grouping, literal, unary, Accept};
use scanner::{token::Token, token_type::TokenType};

// An AST always owns the entire tree below it so when the AST goes
// out of scope the entire tree is destroyed
type AST = Box<dyn Accept + 'static>;

#[allow(dead_code)]
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[allow(unused)]
macro_rules! match_one_of {
    ($parser: ident, $($ttype:expr),*) => (
        {
            let mut ret = false;
            $(if $parser.check ($ttype) {
                $parser.advance();
                ret = true;
            })*
            ret
        }
    );
}

#[allow(unused)]
impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn expression(&mut self) -> AST {
        self.equality()
    }

    fn equality(&mut self) -> AST {
        let mut expr = self.comparison();

        while match_one_of!(self, &TokenType::BangEqual, &TokenType::EqualEqual) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Box::new(binary::new(expr, operator, right));
        }
        expr
    }

    fn comparison(&mut self) -> AST {
        let mut expr = self.term();

        while match_one_of!(
            self,
            &TokenType::Greater,
            &TokenType::GreaterEqual,
            &TokenType::Less,
            &TokenType::LessEqual
        ) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Box::new(binary::new(expr, operator, right));
        }
        expr
    }

    fn term(&mut self) -> AST {
        let mut expr = self.factor();

        while match_one_of!(self, &TokenType::Minus, &TokenType::Plus) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Box::new(binary::new(expr, operator, right));
        }
        expr
    }

    fn factor(&mut self) -> AST {
        let mut expr = self.unary();

        while match_one_of!(self, &TokenType::Slash, &TokenType::Star) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Box::new(binary::new(expr, operator, right));
        }
        expr
    }

    fn unary(&mut self) -> AST {
        if match_one_of!(self, &TokenType::Bang, &TokenType::Minus) {
            let operator = self.previous().clone();
            let right = self.unary();
            Box::new(unary::new(operator, right))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> AST {
        if match_one_of!(
            self,
            &TokenType::False,
            &TokenType::True,
            &TokenType::Nil,
            &TokenType::Number("".to_string()),
            &TokenType::String("".to_string())
        ) {
            return Box::new(literal::new(self.previous().ttype.clone()));
        }

        if match_one_of!(self, &TokenType::LeftParen) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Box::new(grouping::new(expr));
        }
        Box::new(literal::new(TokenType::Eof))
    }

    fn check(&self, tt: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.peek().ttype) == std::mem::discriminant(tt)
        }
    }

    fn consume(&mut self, tt: TokenType, msg: &str) -> Result<TokenType, LoxError> {
        if self.check(&tt) {
            Ok(self.advance().unwrap().ttype)
        } else {
            Err(LoxError::new_text_only(msg))
        }
    }

    fn is_at_end(&self) -> bool {
        return self.peek().ttype == TokenType::Eof;
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> Option<Token> {
        if !self.is_at_end() {
            self.current += 1;
            Some(self.previous().clone())
        } else {
            None
        }
    }
}
