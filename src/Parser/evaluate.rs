use crate::lox_error;
use crate::parser;
use crate::scanner;

use lox_error::lox_error::LoxError;
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

fn to_lox_name(val: &LoxType) -> &'static str {
    match val {
        LoxType::Nil => "nil",
        LoxType::Bool(_) => "bool",
        LoxType::Number(_) => "number",
        LoxType::String(_) => "string",
    }
}

pub struct Evaluator {}

impl Evaluator {
    pub fn evaluate(&self, expr: &(dyn Accept + 'static)) -> Result<ParseReturn, LoxError> {
        expr.accept(self)
    }

    fn get_number(&self, pr: &ParseReturn) -> Result<f64, LoxError> {
        match pr {
            ParseReturn::Val(LoxType::Number(n)) => Ok(*n),
            ParseReturn::Val(val) => {
                let err_msg = format!("Expected number but found {}", to_lox_name(&val));
                Err(LoxError::new_text_only(None, &err_msg))
            }
            _ => panic!("No LoxType in eval"),
        }
    }

    fn get_bool(&self, pr: &ParseReturn) -> Result<bool, LoxError> {
        match pr {
            ParseReturn::Val(LoxType::Bool(f)) => Ok(*f),
            ParseReturn::Val(val) => {
                let err_msg = format!("Expected bool but found {}", to_lox_name(&val));
                Err(LoxError::new_text_only(None, &err_msg))
            }
            _ => panic!("No LoxType in eval"),
        }
    }

    fn get_string(&self, pr: &ParseReturn) -> Result<String, LoxError> {
        match pr {
            // Now that we're actually evaluating we may have to eventually
            // mutate this string so we make a copy instead of using it
            // directly
            ParseReturn::Val(LoxType::String(s)) => Ok(s.clone()),
            ParseReturn::Val(val) => {
                let err_msg = format!("Expected string but found {}", to_lox_name(&val));
                Err(LoxError::new_text_only(None, &err_msg))
            }
            _ => panic!("No LoxType in eval"),
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

    fn get_numeric_values(
        &self,
        left: &ParseReturn,
        right: &ParseReturn,
    ) -> Result<(f64, f64), LoxError> {
        let left_val = self.get_number(&left)?;
        let right_val = self.get_number(&right)?;
        Ok((left_val, right_val))
    }

    fn get_string_values(
        &self,
        left: &ParseReturn,
        right: &ParseReturn,
    ) -> Result<(String, String), LoxError> {
        let left_val = self.get_string(&left)?;
        let right_val = self.get_string(&right)?;
        Ok((left_val, right_val))
    }

    fn get_bool_values(
        &self,
        left: &ParseReturn,
        right: &ParseReturn,
    ) -> Result<(bool, bool), LoxError> {
        let left_val = self.get_bool(&left)?;
        let right_val = self.get_bool(&right)?;
        Ok((left_val, right_val))
    }

    fn is_equal(&self, left: &ParseReturn, right: &ParseReturn) -> Result<bool, LoxError> {
        if self.is_numeric(left) {
            if !self.is_numeric(right) {
                return Ok(false);
            }
            let (left_val, right_val) = self.get_numeric_values(left, right)?;
            return Ok(left_val == right_val);
        };

        if self.is_string(left) {
            if !self.is_string(right) {
                return Ok(false);
            }
            let (left_val, right_val) = self.get_string_values(left, right)?;
            return Ok(left_val == right_val);
        }

        if self.is_bool(left) {
            if !self.is_bool(right) {
                return Ok(false);
            }
            let (left_val, right_val) = self.get_bool_values(left, right)?;
            return Ok(left_val == right_val);
        };

        let is_nil_left = self.is_nil(left);
        let is_nil_right = self.is_nil(right);
        if is_nil_left && is_nil_right {
            return Ok(true);
        };

        if is_nil_left || is_nil_right {
            return Ok(false);
        };

        // Should never reach here...
        panic!("Equals didn't handle all cases");
    }
}

impl Visitor for Evaluator {
    fn literal(&self, expr: &literal) -> Result<ParseReturn, LoxError> {
        Ok(ParseReturn::Val(to_lox_type(&expr.value)))
    }

    fn grouping(&self, expr: &grouping) -> Result<ParseReturn, LoxError> {
        Ok(self.evaluate(&*expr.expression)?)
    }

    fn unary(&self, expr: &unary) -> Result<ParseReturn, LoxError> {
        let right = self.evaluate(&*expr.right)?;
        match expr.operator.ttype {
            TokenType::Minus => {
                let right_val = self.get_number(&right)?;
                Ok(ParseReturn::Val(LoxType::Number(-right_val)))
            }
            TokenType::Bang => {
                let right_val = self.get_bool(&right)?;
                Ok(ParseReturn::Val(LoxType::Bool(!right_val)))
            }
            // Don't think the parser will allow this case to happen
            _ => panic!("Unary with invalid operation in Eval"),
        }
    }

    // copious error handling involved in here...
    fn binary(&self, expr: &binary) -> Result<ParseReturn, LoxError> {
        let left = self.evaluate(&*expr.left)?;
        let right = self.evaluate(&*expr.right)?;
        match expr.operator.ttype {
            TokenType::Minus => {
                let (left_val, right_val) = self.get_numeric_values(&left, &right)?;
                Ok(ParseReturn::Val(LoxType::Number(left_val - right_val)))
            }

            TokenType::Slash => {
                let (left_val, right_val) = self.get_numeric_values(&left, &right)?;
                Ok(ParseReturn::Val(LoxType::Number(left_val / right_val)))
            }

            TokenType::Star => {
                let (left_val, right_val) = self.get_numeric_values(&left, &right)?;
                Ok(ParseReturn::Val(LoxType::Number(left_val * right_val)))
            }

            TokenType::Plus => {
                if self.is_numeric(&left) && self.is_numeric(&right) {
                    let (left_val, right_val) = self.get_numeric_values(&left, &right)?;
                    Ok(ParseReturn::Val(LoxType::Number(left_val + right_val)))
                } else if self.is_string(&left) && self.is_string(&right) {
                    let (left_val, right_val) = self.get_string_values(&left, &right)?;
                    let concat = format!("{}{}", left_val, right_val);
                    Ok(ParseReturn::Val(LoxType::String(concat)))
                } else {
                    Ok(ParseReturn::Val(LoxType::Number(0f64)))
                }
            }

            TokenType::Greater => {
                let (left_val, right_val) = self.get_numeric_values(&left, &right)?;
                Ok(ParseReturn::Val(LoxType::Bool(left_val > right_val)))
            }

            TokenType::Less => {
                let (left_val, right_val) = self.get_numeric_values(&left, &right)?;
                Ok(ParseReturn::Val(LoxType::Bool(left_val < right_val)))
            }

            TokenType::GreaterEqual => {
                let (left_val, right_val) = self.get_numeric_values(&left, &right)?;
                Ok(ParseReturn::Val(LoxType::Bool(left_val >= right_val)))
            }

            TokenType::LessEqual => {
                let (left_val, right_val) = self.get_numeric_values(&left, &right)?;
                Ok(ParseReturn::Val(LoxType::Bool(left_val <= right_val)))
            }

            // We do follow IEEE 754 for NaN here.  The book does not.  Not going to "fix" this.
            TokenType::Equal => Ok(ParseReturn::Val(LoxType::Bool(
                self.is_equal(&left, &right)?,
            ))),

            TokenType::BangEqual => Ok(ParseReturn::Val(LoxType::Bool(
                !self.is_equal(&left, &right)?,
            ))),

            _ => panic!("Unhandled operator in binary"),
        }
    }
}
