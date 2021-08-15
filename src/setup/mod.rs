use crate::lox_error;
use crate::scanner;
use std::env;
use std::fs;
use std::io::{self, stdout, BufRead, Write};

pub fn compile() {
    let args: Vec<String> = env::args().collect();
    let mut result: Result<(), Vec<lox_error::LoxError>> = Ok(());

    if args.len() > 2 {
        lox_error::LoxError::new_text_only("Syntax: lox [file]").report();
    } else if args.len() == 2 {
        result = run_file(&args[1])
    } else {
        result = run_prompt();
    }
    match result {
        Err(v) => {
            for error in v {
                error.report();
            }
        }
        Ok(_) => (),
    }
}

fn run_file(file: &String) -> Result<(), Vec<lox_error::LoxError>> {
    let program_val = fs::read_to_string(file);
    match program_val {
        Err(_) => {
            let error = lox_error::LoxError::new_text_only(&format!("Couldn't read {}", file));
            return Err(vec![error]);
        }
        Ok(program) => run(&program),
    }
}

fn run_prompt() -> Result<(), Vec<lox_error::LoxError>> {
    let reader = io::stdin();
    println!("^c to end...\n");
    loop {
        print!("> ");
        match stdout().flush() {
            Err(err) => {
                let error = lox_error::LoxError::new_text_only(&format!(
                    "Flushing problem: {:?}",
                    err.to_string()
                ));
                return Err(vec![error]);
            }
            Ok(_) => (),
        }
        let mut line = String::new();
        let read_stat = reader.lock().read_line(&mut line);
        match read_stat {
            Ok(0) => break,
            Err(err) => {
                let error = lox_error::LoxError::new_text_only(&format!(
                    "Input problem: {:?}",
                    err.to_string()
                ));
                return Err(vec![error]);
            }
            Ok(_) => (),
        };
        match run(&line) {
            Err(v) => {
                return Err(v);
            }
            Ok(_) => continue,
        }
    }
    Ok(())
}

fn run(program: &String) -> Result<(), Vec<lox_error::LoxError>> {
    let scanner_test = scanner::Scanner::new(&program);
    let scanner = match scanner_test {
        Err(e) => return Err(vec![e]),
        Ok(s) => s,
    };

    for token in scanner.get_tokens() {
        println!("{}", token)
    }
    Ok(())
}
