pub mod sstructs {
    use crate::lox_error::lox_error::LoxError;
    use crate::parser::parser::ParseReturn;
    use crate::{build_struct, build_structs, exprType};

    build_structs! {
        expression : expr expression;
        print : expr expression;
    }

    pub trait Accept {
        fn accept(&self, visitor: &mut dyn Visitor) -> Result<ParseReturn, LoxError>;
    }
}
