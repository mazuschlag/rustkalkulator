use std::io::{self, BufRead};
mod lexer;
mod parser;

fn main() {
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
            println!("tokens: {:?}", tokenizer);
            let mut parser = parser::Parser::new();
            parser.parse(tokenizer.tokens);
            println!("parse tree: {:?}", parser);
        }
    }
}
