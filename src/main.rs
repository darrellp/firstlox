mod lox_error;
mod scanner;
mod setup;
mod parser;
extern crate ascii;
extern crate lazy_static;

fn main() {
    setup::compile();
}
