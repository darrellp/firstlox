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
    Number(i32),
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
}

const TYPE_STRING: &'static [(TokenType, &str)] = &[
    (TokenType::LeftParen, "("),
    (TokenType::RightParen, ")"),
    (TokenType::LeftBrace, "["),
    (TokenType::RightBrace, "]"),
    (TokenType::Comma, ","),
    (TokenType::Dot, "."),
    (TokenType::Minus, "-"),
    (TokenType::Plus, "+"),
    (TokenType::Semicolon, ";"),
    (TokenType::Slash, "/"),
    (TokenType::Star, "*"),
    (TokenType::Bang, "!"),
    (TokenType::BangEqual, "!="),
    (TokenType::Equal, "="),
    (TokenType::EqualEqual, "=="),
    (TokenType::Greater, ">"),
    (TokenType::GreaterEqual, ">="),
    (TokenType::Less, "<"),
    (TokenType::LessEqual, "<="),
    (TokenType::And, "and"),
    (TokenType::Class, "class"),
    (TokenType::Else, "else"),
    (TokenType::False, "false"),
    (TokenType::Fun, "fun"),
    (TokenType::For, "for"),
    (TokenType::If, "if"),
    (TokenType::Nil, "nil"),
    (TokenType::Or, "or"),
    (TokenType::Print, "print"),
    (TokenType::Return, "return"),
    (TokenType::Super, "super"),
    (TokenType::This, "this"),
    (TokenType::True, "true"),
    (TokenType::Var, "var"),
    (TokenType::While, "while"),
    (TokenType::Eof, "eof"),
];

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
pub fn tt_to_string(tt: &TokenType) -> &str {
    MAP_TYPE_TO_STRING[tt]
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
