use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash)]
#[allow(unused)]
pub enum TokenType {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    String(String),
    // We store the string for the float in Number - converting to an f64
    // causes TokenType to be unhashable which means we can't create our
    // hash tables below.  I tried using the enum discriminant but get the
    // error that they're not stable in consts so that doesn't work.
    // Another idea would be to make a dummy hashing function for floats.
    // There's no actual floats being hashed here anyway so it would just
    // be to allow hashing on this data type.  In the end, using an f64
    // would mean I'd lose the actual lexeme that led to the number so it's
    // a bit of a disadvantage and I decided to stick with the string.
    Number(String),
    Identifier(String),

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,

    Error,
}

macro_rules! tt_entry {
    ($($type: tt: $val: expr) *) => (&[
        $((TokenType::$type, $val),)*
    ])
}

const TYPE_STRING: &'static [(TokenType, &str)] = tt_entry! {
    LeftParen: "("
    RightParen: ")"
    LeftBrace: "{"
    RightBrace: "}"
    Comma: ","
    Dot: "."
    Minus: "-"
    Plus: "+"
    Semicolon: ";"
    Slash: "/"
    Star: "*"
    Bang: "!"
    BangEqual: "!="
    Equal: "="
    EqualEqual: "=="
    Greater: ">"
    GreaterEqual: ">="
    Less: "<"
    LessEqual: "<="
    And: "and"
    Class: "class"
    Else: "else"
    False: "false"
    Fun: "fun"
    For: "for"
    If: "if"
    Nil: "nil"
    Or: "or"
    Return: "return"
    Super: "super"
    This: "this"
    True: "true"
    Var: "var"
    While: "while"
    Eof: "eof"
};

lazy_static! {
    static ref MAP_TYPE_TO_STRING: HashMap<TokenType, &'static str> = {
        let mut hm = HashMap::new();
        for pair in TYPE_STRING {
            let tt = pair.0.clone();
            let rep = &pair.1;

            hm.insert(tt.clone(), rep.clone());
        }
        hm
    };
    static ref MAP_STRING_TO_TYPE: HashMap<&'static str, TokenType> = {
        let mut hm = HashMap::new();
        for pair in TYPE_STRING {
            let tt = &pair.0;
            let rep = &pair.1;

            hm.insert(rep.clone(), tt.clone());
        }
        hm
    };
}

#[allow(dead_code)]
impl TokenType {
    // We have to handle numeric value specially since including an f32 as an associated
    // value in the enum renders it unhashable.
    pub fn num_value(&self) -> f32 {
        match self {
            Self::Number(text) => text.parse::<f32>().unwrap(),
            _ => {
                assert!(false, "Trying to retrieve num value from non-num token");
                0.0
            }
        }
    }

    pub fn to_stringslice(&self) -> &str {
        MAP_TYPE_TO_STRING[self]
    }

    pub fn to_keyword(text: &str) -> Option<TokenType> {
        if text != "eof" && MAP_STRING_TO_TYPE.contains_key(text) {
            Some(MAP_STRING_TO_TYPE[text].clone())
        } else {
            None
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::String(s) => f.write_str(format!("\"{}\"", s).as_ref()),
            TokenType::Identifier(s) => f.write_str(format!("id[\"{}\"]", s).as_ref()),
            TokenType::Number(n) => f.write_str(format!("{}", n).as_ref()),
            // Everything else...
            _tt => f.write_str(MAP_TYPE_TO_STRING[_tt]),
        }
    }
}
