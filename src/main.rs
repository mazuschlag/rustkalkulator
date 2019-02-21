use std::io::{self, BufRead};
mod lexer;

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
        }
    }
}
