use crate::lox_error;
use crate::parser;
use crate::scanner;

use parser::parser::{binary, grouping, literal, unary, Accept, ParseReturn, Visitor};
use scanner::token_type::TokenType;

// I don't really see any reason I couldn't put the types of LoxType directly into
// ParseReturn.  It would probably makes things both quicker and easier but it would
// seem like each return type of ParseReturn should correspond to it's own visitor
// class. To do otherwise would be non-orthogonal to the only current other visitor,
// the pretty printer and just go against the idea behind ParseReturn which is a
// replacement for generic parameters which I can have on trait objects sadly.
#[derive(PartialEq)]
pub enum LoxType {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
}

fn to_lox_type(tt: &TokenType) -> LoxType {
    match tt {
        TokenType::Number(s) => LoxType::Number(str::parse::<f64>(s).unwrap()),
        TokenType::String(s) => LoxType::String(s.to_string()),
        TokenType::False => LoxType::Bool(false),
        TokenType::True => LoxType::Bool(true),
        TokenType::Nil => LoxType::Nil,
        _ => panic!("Unexpected val in to_lox_type"),
    }
}

pub struct evaluator {}

impl evaluator {
    pub fn evaluate(&self, expr: &(dyn Accept + 'static)) -> ParseReturn {
        expr.accept(self)
    }

    fn get_number(&self, pr: &ParseReturn) -> f64 {
        match pr {
            ParseReturn::Val(LoxType::Number(n)) => *n,
            _ => 0f64, // Error Detection yet to be done...
        }
    }

    fn get_bool(&self, pr: &ParseReturn) -> bool {
        match pr {
            ParseReturn::Val(LoxType::Bool(f)) => *f,
            _ => false,
        }
    }

    fn get_string(&self, pr: &ParseReturn) -> String {
        match pr {
            // Now that we're actually evaluating we may have to eventually
            // mutate this string so we make a copy instead of using it
            // directly
            ParseReturn::Val(LoxType::String(s)) => s.clone(),
            _ => "".to_string(),
        }
    }

    fn is_nil(&self, pr: &ParseReturn) -> bool {
        *pr == ParseReturn::Val(LoxType::Nil)
    }

    fn is_numeric(&self, pr: &ParseReturn) -> bool {
        match pr {
            ParseReturn::Val(LoxType::Number(_)) => true,
            _ => false,
        }
    }

    fn is_string(&self, pr: &ParseReturn) -> bool {
        match pr {
            ParseReturn::Val(LoxType::String(_)) => true,
            _ => false,
        }
    }

    fn is_bool(&self, pr: &ParseReturn) -> bool {
        match pr {
            ParseReturn::Val(LoxType::Bool(_)) => true,
            _ => false,
        }
    }

    fn get_numeric_values(&self, left: &ParseReturn, right: &ParseReturn) -> (f64, f64) {
        let left_val = self.get_number(&left);
        let right_val = self.get_number(&right);
        (left_val, right_val)
    }

    fn get_string_values(&self, left: &ParseReturn, right: &ParseReturn) -> (String, String) {
        let left_val = self.get_string(&left);
        let right_val = self.get_string(&right);
        (left_val, right_val)
    }

    fn get_bool_values(&self, left: &ParseReturn, right: &ParseReturn) -> (bool, bool) {
        let left_val = self.get_bool(&left);
        let right_val = self.get_bool(&right);
        (left_val, right_val)
    }

    fn is_equal(&self, left: &ParseReturn, right: &ParseReturn) -> bool {
        if self.is_numeric(left) {
            if !self.is_numeric(right) {
                return false;
            }
            let (left_val, right_val) = self.get_numeric_values(left, right);
            return left_val == right_val;
        };

        if self.is_string(left) {
            if !self.is_string(right) {
                return false;
            }
            let (left_val, right_val) = self.get_string_values(left, right);
            return left_val == right_val;
        }

        if self.is_bool(left) {
            if !self.is_bool(right) {
                return false;
            }
            let (left_val, right_val) = self.get_bool_values(left, right);
            return left_val == right_val;
        };

        let is_nil_left = self.is_nil(left);
        let is_nil_right = self.is_nil(right);
        if is_nil_left && is_nil_right {
            return true;
        };

        if is_nil_left || is_nil_right {
            return false;
        };

        // Should never reach here...
        panic!("Equals didn't handle all cases");
    }
}

impl Visitor for evaluator {
    fn literal(&self, expr: &literal) -> ParseReturn {
        ParseReturn::Val(to_lox_type(&expr.value))
    }

    fn grouping(&self, expr: &grouping) -> ParseReturn {
        self.evaluate(&*expr.expression)
    }

    fn unary(&self, expr: &unary) -> ParseReturn {
        let right = self.evaluate(&*expr.right);
        match expr.operator.ttype {
            TokenType::Minus => {
                let right_val = self.get_number(&right);
                ParseReturn::Val(LoxType::Number(-right_val))
            }
            TokenType::Bang => {
                let right_val = self.get_bool(&right);
                ParseReturn::Val(LoxType::Bool(!right_val))
            }
            _ => ParseReturn::Val(LoxType::Number(0f64)), // Error Handling needed
        }
    }

    // copious error handling involved in here...
    fn binary(&self, expr: &binary) -> ParseReturn {
        let left = self.evaluate(&*expr.left);
        let right = self.evaluate(&*expr.right);
        match expr.operator.ttype {
            TokenType::Minus => {
                let (left_val, right_val) = self.get_numeric_values(&left, &right);
                ParseReturn::Val(LoxType::Number(left_val - right_val))
            }

            TokenType::Slash => {
                let (left_val, right_val) = self.get_numeric_values(&left, &right);
                ParseReturn::Val(LoxType::Number(left_val / right_val))
            }

            TokenType::Star => {
                let (left_val, right_val) = self.get_numeric_values(&left, &right);
                ParseReturn::Val(LoxType::Number(left_val * right_val))
            }

            TokenType::Plus => {
                if self.is_numeric(&left) && self.is_numeric(&right) {
                    let (left_val, right_val) = self.get_numeric_values(&left, &right);
                    ParseReturn::Val(LoxType::Number(left_val + right_val))
                } else if self.is_string(&left) && self.is_string(&right) {
                    let (left_val, right_val) = self.get_string_values(&left, &right);
                    let concat = format!("{}{}", left_val, right_val);
                    ParseReturn::Val(LoxType::String(concat))
                } else {
                    ParseReturn::Val(LoxType::Number(0f64))
                }
            }

            TokenType::Greater => {
                let (left_val, right_val) = self.get_numeric_values(&left, &right);
                ParseReturn::Val(LoxType::Bool(left_val > right_val))
            }

            TokenType::Less => {
                let (left_val, right_val) = self.get_numeric_values(&left, &right);
                ParseReturn::Val(LoxType::Bool(left_val < right_val))
            }

            TokenType::GreaterEqual => {
                let (left_val, right_val) = self.get_numeric_values(&left, &right);
                ParseReturn::Val(LoxType::Bool(left_val >= right_val))
            }

            TokenType::LessEqual => {
                let (left_val, right_val) = self.get_numeric_values(&left, &right);
                ParseReturn::Val(LoxType::Bool(left_val <= right_val))
            }

            // We do follow IEEE 754 for NaN here.  The book does not.  Not going to "fix" this.
            TokenType::Equal => ParseReturn::Val(LoxType::Bool(self.is_equal(&left, &right))),

            TokenType::BangEqual => ParseReturn::Val(LoxType::Bool(!self.is_equal(&left, &right))),

            _ => ParseReturn::Val(LoxType::Number(0f64)), // Error Handling needed
        }
    }
}
