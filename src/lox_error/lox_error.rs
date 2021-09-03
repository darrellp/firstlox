use crate::scanner;
use scanner::token::Token;
use scanner::token_type::TokenType;

#[derive(Clone)]
pub struct LoxError {
    token_option: Option<Token>,
    line_option: Option<usize>,
    text: String,
}

impl LoxError {
    #[allow(dead_code)]
    pub fn new(token: Token, text: String) -> LoxError {
        LoxError {
            line_option: Some(token.line),
            token_option: Some(token),
            text,
        }
    }

    pub fn new_text_only(line_number: Option<usize>, text: &str) -> LoxError {
        LoxError {
            line_option: line_number,
            token_option: None,
            text: text.to_string(),
        }
    }

    pub fn report_msg(&self) -> String {
        let msg = match &self.token_option {
            Some(tt) => match tt.ttype {
                TokenType::Eof => format!("at end - {}", self.text),
                _ => format!("at '{}' - {}", tt.lexeme, self.text),
            },
            _ => self.text.clone(),
        };
        match self.line_option {
            Some(ln) => format!("{}: {}", ln, msg),
            None => format!("{}", msg),
        }
    }

    pub fn report(&self) {
        println!("{}", self.report_msg());
    }
}

#[test]
pub fn error_test() {
    let token = Token::new(&TokenType::And, &"&".to_string(), 10);
    let err = LoxError::new(token, "Test with normal token".to_string());
    let text = err.report_msg();

    assert_eq!("10: at '&' - Test with normal token", text);

    let token = Token::new(&TokenType::Eof, &"".to_string(), 20);
    let err = LoxError::new(token, "Test with EOF token".to_string());
    let text = err.report_msg();

    assert_eq!("20: at end - Test with EOF token", text);

    let err = LoxError::new_text_only(None, "Test with only this text");
    let text = err.report_msg();

    assert_eq!("Test with only this text", text);

    let err = LoxError::new_text_only(Some(30), "Test with only text and line number");
    let text = err.report_msg();

    assert_eq!("30: Test with only text and line number", text);
}

#[derive(Clone)]
pub struct LoxErrorList {
    errors: Vec<LoxError>,
}

#[allow(unused)]
impl LoxErrorList {
    pub fn new() -> Self {
        LoxErrorList { errors: vec![] }
    }

    pub fn single(err: LoxError) -> Self {
        LoxErrorList { errors: vec![err] }
    }

    pub fn push(&mut self, error: LoxError) {
        self.errors.push(error);
    }

    pub fn append(&mut self, mut elst: LoxErrorList) {
        self.errors.append(&mut elst.errors);
    }

    pub fn len(&self) -> usize {
        self.errors.len()
    }

    pub fn report(&self) -> () {
        for error in self.errors.iter() {
            error.report();
        }
    }
}
