// rlox - Interpreter in Rust for Lox Language
// 1. Takes input the source code for Lox Language, read, parse and execute it
// 2. Or provide a REPL interface where user provides a Lox language code on command line,
// read, parse and execute it. Ctrl+D  signals EOF to terminate the REPL loop.

// Scanning
// Read source code and extract tokens

mod errors;
mod lexer;
mod token_type;
use clap::Parser;
use std::{error::Error, fs, io};

use crate::lexer::tokenize;

#[derive(Parser, Debug)]
struct CliArgs {
    // Path to lox source code
    file_path: Option<String>,
    // Enable REPL mode
    #[arg(short, long, default_value_t = false)]
    repl: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::parse();

    if args.repl {
        loop {
            let mut input = String::new();

            // Ctrl+D signals EOF
            if io::stdin().read_line(&mut input)? == 0 {
                break;
            }

            match tokenize(&input) {
                Ok(tokens) => println!("{tokens:#?}"),
                Err(err) => eprintln!("Error encountered: {err}"),
            }
        }
    } else if let Some(file_path) = args.file_path {
        let input = fs::read_to_string(file_path)?;

        match tokenize(&input) {
            Ok(tokens) => println!("{tokens:#?}"),
            Err(err) => eprintln!("Error encountered: {err}"),
        }
    }

    Ok(())
}
