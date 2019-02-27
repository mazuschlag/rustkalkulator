use std::io::{self, BufRead};
use std::collections::HashMap;
mod lexer;
mod parser;
mod evaluator;

fn main() {
    let mut symbol_table = HashMap::new();
    println!("Welcom to Rustkalkulator!");
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
            let mut evaluator = evaluator::Evaluator::new(symbol_table);
            let answer = evaluator.evaluate(*parser.tree.unwrap());
            println!("{}", answer.unwrap());
            symbol_table = evaluator.symbols;
        }
    }
}
