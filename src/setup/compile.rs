use crate::colored::*;
use crate::lox_error;
use crate::parser;
use crate::parser::evaluate::Evaluator;
use crate::scanner::scanner;

use lox_error::{lox_error::LoxError, lox_error::LoxErrorList};
use parser::{parser::Parser, pretty_print::AstPrinter};
use std::env;
use std::fs;
use std::io::{self, stdout, BufRead, Write};

pub fn compile() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        LoxError::new_text_only(None, "Syntax: lox [file]").report();
    } else if args.len() == 2 {
        run_file(&args[1])
    } else {
        run_prompt();
    }
}

fn run_file(file: &String) {
    let program_val = fs::read_to_string(file);
    match program_val {
        Err(_) => {
            let error = LoxError::new_text_only(None, &format!("Couldn't read {}", file));
            error.report()
        }
        Ok(program) => {
            run(&program).report();
        }
    }
}

fn run_prompt() {
    let reader = io::stdin();
    println!("^c to end...\n");
    loop {
        print!("> ");
        match stdout().flush() {
            Err(err) => {
                let error = LoxError::new_text_only(
                    None,
                    &format!("Flushing problem: {:?}", err.to_string()),
                );
                error.report()
            }
            Ok(_) => (),
        }
        let mut line = String::new();
        let read_stat = reader.lock().read_line(&mut line);
        match read_stat {
            Ok(0) => break,
            Err(err) => {
                let error =
                    LoxError::new_text_only(None, &format!("Input problem: {:?}", err.to_string()));
                error.report();
                continue;
            }
            Ok(_) => line = line.trim().to_string(),
        };
        run(&line).report()
    }
}

// run() should take care of all running (duh).  The only thing it's callers get is
// a list of the errors.  The buck stops here.
fn run(program: &String) -> LoxErrorList {
    let scanner_test = scanner::Scanner::new(&program);
    let mut scanner = match scanner_test {
        Err(e) => return LoxErrorList::single(e.clone()),
        Ok(s) => s,
    };

    scanner.scan_tokens();

    // This kills scanner as it moves all the tokens out of it - they now belong to
    // the parser
    let mut parser = Parser::new(scanner.get_tokens());
    let expr_opt = parser.parse();
    match expr_opt {
        None => parser.errors,
        Some(ast) => {
            let mut printer = AstPrinter {};
            println!("{}", printer.pretty_print_value(&*ast).yellow());
            let mut errors = parser.errors;
            if errors.len() == 0 {
                errors = Evaluator {}.interpret(&*ast);
            }
            errors
        }
    }
}
