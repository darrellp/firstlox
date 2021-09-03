use crate::lox_error;
use crate::scanner;
use crate::{build_struct, build_structs, exprType};
use lox_error::lox_error::{LoxError, LoxErrorList};
use scanner::{token::Token, token_type::TokenType};

// An AST always owns the entire tree below it so when the AST goes
// out of scope the entire tree is destroyed
type AST = Box<dyn Accept + 'static>;

// ParseReturn is an enumeration to allow us to use Accept without generic
// parameters which in turn would keep cause rustc to disallow dyn Accept.  Instead of
// using a generic parameter to indicate our return type we always return
// a ParseReturn and use the different enumerations to contain our various
// return types.  Not quite as convenient but it has the advantage of working.
#[allow(dead_code)]
pub enum ParseReturn {
    PP(String),
    AST,
}

build_structs! {
    binary : expr left, Token operator, expr right;
    grouping : expr expression;
    literal : TokenType value;
    unary : Token operator, expr right;
}

pub trait Accept {
    fn accept(&self, visitor: &dyn Visitor) -> ParseReturn;
}

#[allow(dead_code)]
struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: LoxErrorList,
}

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
        Parser {
            tokens,
            current: 0,
            errors: LoxErrorList::new(),
        }
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

    fn consume(&mut self, tt: TokenType, msg: &str) -> TokenType {
        if self.check(&tt) {
            self.advance().unwrap().ttype
        } else {
            // Advance or don't advance?  Book throws.
            self.errors
                .push(LoxError::new_text_only(Some(self.peek().line), msg));
            TokenType::Error
        }
    }

    fn err_on_token(&mut self, token: &Token, msg: &str) {
        self.errors
            .push(LoxError::new(token.clone(), msg.to_string()))
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