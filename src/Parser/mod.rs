use crate::scanner::token::Token;
use crate::scanner::token_type::TokenType;

#[allow(unused)]
pub enum ParseReturn {
    PP(String),
    AST,
}

pub trait Accept {
    fn accept(&self, visitor: &dyn Visitor) -> ParseReturn;
}

/// Produces visitor structs for parser productions
macro_rules! build_struct {
    ($struct_name:ident : $($type:ident $name:ident),*) => (
        #[allow(unused)]
        #[allow(non_camel_case_types)]
        pub struct $struct_name {
            $(
                $name: exprType!($type),
            )*
        }

        #[allow(unused)]
        impl $struct_name {
            fn new(
                $(
                    $name: exprType!($type)
                ),*
            ) -> Self {
                $struct_name {
                    $(
                        $name
                    ),*
                }
            }
        }

        impl Accept for $struct_name {
            fn accept(&self, visitor: &dyn Visitor) -> ParseReturn {
                visitor.$struct_name(self)
            }
        }
    )
}

macro_rules! exprType {
    (expr) => (Box<dyn Accept>);
    ($type: ident) => ($type);
}

macro_rules! build_structs {
    ( $( $rhs_name:ident : $($lhs_name:ident $lhs_type:ident),* ;)+ )
    => {
        // Member functions of this trait are actually visitors which I'd
        // like to name something like visit-assign but rust macros won't
        // allow string concatenation in identifiers so I just have to
        // leave them with the same names as the classes they visit.
        pub trait Visitor {
            $(
                fn $rhs_name(&self, expr: &$rhs_name) -> ParseReturn;
            )*
        }

        // Build the production structures
        $(
            build_struct!($rhs_name : $($lhs_name $lhs_type),*);
        )*
    };
}

build_structs! {
    binary : expr left, Token operator, expr right;
    grouping : expr expression;
    literal : TokenType value;
    unary : Token operator, expr right;
}
macro_rules! parenthesize {
    ($printer: expr, $name:expr => $($args: expr),*) => ( {
        let mut result = "(".to_string();
        result += $name;

        $(
            result += " ";
            result += &(if let ParseReturn::PP(val) = $args.accept($printer) { val } else {"ERROR".to_string()});
        )*
        ParseReturn::PP(result + ")")
    }
    );
}

pub struct AstPrinter {}

#[allow(unused)]
impl AstPrinter {
    pub fn pretty_print_value(&self, expr: &dyn Accept) -> String {
        if let ParseReturn::PP(value) = expr.accept(self) {
            value
        } else {
            "Not PP in ParseResult".to_string()
        }
    }
}

impl Visitor for AstPrinter {
    fn binary(&self, expr: &binary) -> ParseReturn {
        parenthesize!(self, &expr.operator.lexeme => expr.left, expr.right)
    }
    fn grouping(&self, expr: &grouping) -> ParseReturn {
        parenthesize!(self, "group" => expr.expression)
    }
    fn literal(&self, expr: &literal) -> ParseReturn {
        match &expr.value {
            TokenType::Number(n) => {
                ParseReturn::PP(format!("{:.2}", str::parse::<f64>(n).unwrap()))
            }
            TokenType::String(s) => ParseReturn::PP(format!("{}", s)),
            _ => ParseReturn::PP("Non-Literal TokenType in Pretty Print".to_string()),
        }
    }
    fn unary(&self, expr: &unary) -> ParseReturn {
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
        "(* (- 123.00) (group 45.67))".to_string(),
        AstPrinter {}.pretty_print_value(&expr)
    );
}
