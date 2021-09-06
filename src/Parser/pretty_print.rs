use crate::lox_error;
use crate::parser;
use crate::scanner;

use lox_error::lox_error::LoxError;
use parser::parser::{binary, grouping, literal, unary, Accept, ParseReturn, Visitor};
// Without the "unused" exemption rustc claims that token::Token is unused
// although it is most certainly is used and will give an unresolved error if I remove
// it from the "use" statement.  Confusing.
#[allow(unused)]
use scanner::{token::Token, token_type::TokenType};

pub struct AstPrinter {}

#[allow(unused)]
impl AstPrinter {
    pub fn pretty_print_value(&self, expr: &dyn Accept) -> String {
        if let Result::Ok(ParseReturn::PP(value)) = expr.accept(self) {
            value
        } else {
            "Not PP in ParseResult".to_string()
        }
    }
}

macro_rules! parenthesize {
    ($printer: expr, $name:expr => $($args: expr),*) => ( {
        let mut result = "(".to_string();
        result += $name;

        $(
            result += " ";
            result += &(if let Ok(ParseReturn::PP(val)) = $args.accept($printer) { val } else {"ERROR".to_string()});
        )*
        Ok(ParseReturn::PP(result + ")"))
    }
    );
}

impl Visitor for AstPrinter {
    fn binary(&self, expr: &binary) -> Result<ParseReturn, LoxError> {
        parenthesize!(self, &expr.operator.lexeme => expr.left, expr.right)
    }
    fn grouping(&self, expr: &grouping) -> Result<ParseReturn, LoxError> {
        parenthesize!(self, "group" => expr.expression)
    }
    fn literal(&self, expr: &literal) -> Result<ParseReturn, LoxError> {
        match &expr.value {
            TokenType::Number(n) => Ok(ParseReturn::PP(format!(
                "{}",
                str::parse::<f64>(n).unwrap()
            ))),
            TokenType::String(s) => Ok(ParseReturn::PP(format!("\"{}\"", s))),
            _ => Ok(ParseReturn::PP(
                "Non-Literal TokenType in Pretty Print".to_string(),
            )),
        }
    }
    fn unary(&self, expr: &unary) -> Result<ParseReturn, LoxError> {
        parenthesize!(self, &expr.operator.lexeme => expr.right)
    }
}

#[test]
pub fn pretty_print_test() {
    let num1_lit = literal::new(TokenType::Number("123".to_string()));
    let num2_lit = literal::new(TokenType::Number("45.67".to_string()));
    let grouping_expr = grouping::new(Box::new(num2_lit));
    let unary_expr = unary::new(
        Token::new(&TokenType::Minus, &"-".to_string(), 1),
        Box::new(num1_lit),
    );
    let expr = binary::new(
        Box::new(unary_expr),
        Token::new(&TokenType::Star, &"*".to_string(), 1),
        Box::new(grouping_expr),
    );

    assert_eq!(
        "(* (- 123) (group 45.67))".to_string(),
        AstPrinter {}.pretty_print_value(&expr)
    );
}
