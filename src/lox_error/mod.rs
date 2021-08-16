pub struct LoxError {
    line_number: Option<usize>,
    text: String,
}

impl LoxError {
    #[allow(dead_code)]
    pub fn new(line: usize, text: String) -> LoxError {
        LoxError {
            line_number: Some(line),
            text,
        }
    }

    pub fn new_text_only(text: &str) -> LoxError {
        LoxError {
            line_number: None,
            text: text.to_string(),
        }
    }

    pub fn report(&self) {
        let ln = if self.line_number == None {
            "".to_string()
        } else {
            format!("{}: ", self.line_number.unwrap())
        };

        println!("{}{}", ln, self.text)
    }
}

pub struct LoxErrorList {
    errors: Vec<LoxError>,
}

impl LoxErrorList {
    pub fn new() -> Self {
        LoxErrorList { errors: vec![] }
    }

    pub fn single(err: LoxError) -> Self {
        LoxErrorList { errors: vec![err] }
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
