use std::{
    env,
    io::{self},
    process,
};

pub mod lexer;
pub mod matcher;
pub mod parser;

// Usage: echo <input_text> | your_grep.sh -E <pattern>
fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let tokens = lexer::tokenize(&pattern);
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse().expect("Failed to parse pattern");

    if matcher::match_pattern(&ast, &input.trim()) {
        println!("{}", input);
    } else {
        println!("No match");
        process::exit(1);
    }
}
