use crate::lox_error::LoxError;
use crate::lox_error::LoxErrorList;
use crate::scanner;
use std::env;
use std::fs;
use std::io::{self, stdout, BufRead, Write};

pub fn compile() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        LoxError::new_text_only("Syntax: lox [file]").report();
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
            let error = LoxError::new_text_only(&format!("Couldn't read {}", file));
            error.report()
        }
        Ok(program) => match run(&program) {
            Err(erlst) => erlst.report(),
            Ok(_) => {}
        },
    }
}

fn run_prompt() {
    let reader = io::stdin();
    println!("^c to end...\n");
    loop {
        print!("> ");
        match stdout().flush() {
            Err(err) => {
                let error =
                    LoxError::new_text_only(&format!("Flushing problem: {:?}", err.to_string()));
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
                    LoxError::new_text_only(&format!("Input problem: {:?}", err.to_string()));
                error.report();
                continue;
            }
            Ok(_) => line = line.trim().to_string(),
        };
        match run(&line) {
            Err(v) => v.report(),
            Ok(_) => {}
        }
    }
}

fn run(program: &String) -> Result<(), LoxErrorList> {
    let scanner_test = scanner::Scanner::new(&program);
    let mut scanner = match scanner_test {
        Err(e) => return Err(LoxErrorList::single(e)),
        Ok(s) => s,
    };

    let ret = scanner.scan_tokens();
    for token in scanner.get_tokens() {
        println!("{}", token)
    }
    ret
}
