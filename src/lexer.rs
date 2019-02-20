#[derive(Debug)]
pub struct Tokens {
    tokens: Vec<Token>,
}

impl Tokens {
    pub fn new() -> Tokens {
        Tokens {
            tokens: Vec::new()
        }
    }
    pub fn tokenize(&mut self, mut input: std::str::Chars) {
        match input.next() {
            Some(c) => { 
                match c {
                    c if "+-*/".contains(c) => {
                        self.into_operator(c, input);
                    },
                    '=' => { 
                        self.tokens.push(Token::Assign);
                        self.tokenize(input);
                    },
                    '(' => {
                        self.tokens.push(Token::LParen);
                        self.tokenize(input);
                    }
                    ')' => {
                        self.tokens.push(Token::RParen);
                        self.tokenize(input);
                    },
                    c if c.is_digit(10) => {
                        let num = String::new();
                        self.into_number(c, num, input)
                    },
                    c if c.is_alphabetic() => {
                        let ident = String::new();
                        self.into_identifier(c, ident, input);
                    },
                    c if c.is_whitespace() => self.tokenize(input),
                    _ => self.tokens.push(Token::Error),
                };
            },
            None => self.tokens.push(Token::End)
        }
    }

    fn into_operator(&mut self, o: char, input: std::str::Chars) {
        let op = match o {
            '-' => Operator::Minus,
            '*' => Operator::Times,
            '/' => Operator::Divide,
            _ => Operator::Plus,
        };
        self.tokens.push(Token::Op(op));
        self.tokenize(input);
    }

    fn into_number(&mut self, n: char, mut num: String, mut input: std::str::Chars) {
        num.push(n);
        match input.next() {
            Some(c) => {
                match c {
                    c if c.is_digit(10) => self.into_number(c, num, input),
                    c if c.is_whitespace() => {
                        self.tokens.push(Token::Num(num.parse::<u32>().unwrap()));
                        self.tokenize(input);
                    },
                    _ => self.tokens.push(Token::Error)
                };
            },
            None => {
                self.tokens.push(Token::Num(num.parse::<u32>().unwrap()));
                self.tokens.push(Token::End);
            }
        };
    }

    fn into_identifier(&mut self, i: char, mut ident: String, mut input: std::str::Chars) {
        ident.push(i);
        match input.next() {
            Some(c) => {
                match c {
                    c if c.is_alphabetic() => self.into_identifier(c, ident, input),
                    c if c.is_whitespace() => {
                        self.tokens.push(Token::Ident(ident));
                        self.tokenize(input);
                    },
                    _ => self.tokens.push(Token::Error)
                };
            },
            None => { 
                self.tokens.push(Token::Ident(ident));
                self.tokens.push(Token::End);
            }
        };
    }
}

#[derive(Debug)]
enum Token {
    LParen,
    RParen,
    Assign,
    Op(Operator),
    Ident(String),
    Num(u32),
    Error,
    End,
}

#[derive(Debug)]
enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
}