// rlox - Interpreter in Rust for Lox Language
// 1. Takes input the source code for Lox Language, read, parse and execute it 
// 2. Or provide a REPL interface where user provides a Lox language code on command line,
// read, parse and execute it. Ctrl+D  signals EOF to terminate the REPL loop.

// Scanning
// Read source code and extract tokens according to the grammar
// Questions:
// 1. How to handle invalid tokens?

mod lexer;
mod errors;
mod token_type;

fn main() {
    println!("Hello, world!");
}


