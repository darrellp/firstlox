use crate::scanner::token::Token;
use crate::scanner::token_type::TokenType;

/// Produces visitor structs for parser productions
macro_rules! build_struct {
    ($struct_name:ident : $($type:ident $name:ident),*) => (
        #[allow(unused)]
        #[allow(non_camel_case_types)]
        pub struct $struct_name {
            $(
                $name: $type,
            )*
        }

        #[allow(unused)]
        impl $struct_name {
            pub fn new(
                $(
                    $name: $type
                ),*
            ) -> Self {
                $struct_name {
                    $(
                        $name
                    ),*
                }
            }
        }

        impl<T> Accept<T> for $struct_name {
            fn accept(&self, visitor: &impl Visitor<T>) -> T {
                visitor.$struct_name(self)
            }
        }
    )
}

macro_rules! build_structs {
    ( $( $rhs_name:ident : $($lhs_name:ident $lhs_type:ident),* ;)+ )
    => {
        pub trait Accept<T> {
            fn accept(&self, visitor: &impl Visitor<T>) -> T;
        }
        // Member functions of this trait are actually visitors which I'd
        // like to name something like visit-assign but rust macros won't
        // allow string concatenation in identifiers so I just have to
        // leave them with the same names as the classes they visit.
        pub trait Visitor<T> {
            $(
                fn $rhs_name(&self, expr: &$rhs_name) -> T;
            )*
        }

        // Build the production structures
        $(
            build_struct!($rhs_name : $($lhs_name $lhs_type),*);
        )*
    };
}

pub struct expr {}

build_structs! {
    binary : expr left, Token operator, expr right;
    grouping : expr expression;
    literal : TokenType value;
    unary : Token operator, expr right;
}
