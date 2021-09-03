/// Produces visitor structs for parser productions
#[macro_export]
macro_rules! build_struct {
    ($struct_name:ident : $($type:ident $name:ident),*) => (
        #[allow(unused)]
        #[allow(non_camel_case_types)]
        pub struct $struct_name {
            $(
                pub $name: exprType!($type),
            )*
        }

        #[allow(unused)]
        impl $struct_name {
            pub fn new(
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

#[macro_export]
macro_rules! exprType {
    (expr) => (Box<dyn Accept>);
    ($type: ident) => ($type);
}

#[macro_export]
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
