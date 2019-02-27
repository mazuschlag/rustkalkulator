use std::io::{self, BufRead};
use std::collections::HashMap;
mod lexer;
mod parser;
mod evaluator;

fn main() {
    let mut symbol_table = HashMap::new();
    println!("Welcome to Rustkalkulator!");
    println!("Press 'q' to quit");
    loop {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let string = line.unwrap();
            if string == "q" {
                println!("Goodbye!");
                return;
            }
            let chars = string.chars();
            let mut tokenizer = lexer::Tokens::new(chars);
            tokenizer.tokenize();
            let mut parser = parser::Parser::new();
            parser.parse(tokenizer.tokens);
            let (answer, symbol_update) = evaluator::evaluate(parser.tree, symbol_table);
            match answer {
                Ok(a) => println!("{}", a),
                Err(e) => println!("{}", e)
            };
            symbol_table = symbol_update;
        }
    }
}
