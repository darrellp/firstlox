mod lox_error;
mod parser;
mod scanner;
mod setup;
extern crate ascii;
extern crate colored;
extern crate lazy_static;

fn main() {
    setup::compile::compile();
}
