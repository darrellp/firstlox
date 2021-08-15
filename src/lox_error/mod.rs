pub struct LoxError {
    line_number: i32,
    text: String,
}

impl LoxError {
    #[allow(dead_code)]
    pub fn new(line: &i32, text: String) -> LoxError {
        LoxError {
            line_number: *line,
            text,
        }
    }

    pub fn new_text_only(text: &str) -> LoxError {
        LoxError {
            line_number: -1,
            text: text.to_string(),
        }
    }

    pub fn report(&self) {
        let ln = if self.line_number < 0 {
            "".to_string()
        } else {
            format!("{}: ", self.line_number)
        };

        println!("{}{}", ln, self.text)
    }
}
