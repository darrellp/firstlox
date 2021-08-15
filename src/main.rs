mod lox_error;
mod scanner;
mod setup;
extern crate ascii;
extern crate lazy_static;

fn main() {
    setup::compile();
}
