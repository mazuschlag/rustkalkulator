use std::io::{self, BufRead};
mod lexer;

fn main() {
    loop {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let string = line.unwrap();
            let chars = string.chars();
            let mut tokenizer = lexer::Tokens::new(chars);
            tokenizer.tokenize();
            println!("tokens: {:?}", tokenizer);
        }
    }
}
