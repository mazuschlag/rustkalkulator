#[derive(Debug)]
pub struct Tokens<'a> {
    tokens: Vec<Token>,
    input: std::str::Chars<'a>
}

impl<'a> Tokens<'a> {
    pub fn new(characters: std::str::Chars) -> Tokens {
        Tokens {
            tokens: Vec::new(),
            input: characters
        }
    }
    pub fn tokenize(&mut self) {
        match self.input.next() {
            Some(c) => { 
                match c {
                    c if "+-*/".contains(c) => {
                        self.into_operator(c);
                    },
                    '=' => { 
                        self.tokens.push(Token::Assign);
                        self.tokenize();
                    },
                    '(' => {
                        self.tokens.push(Token::LParen);
                        self.tokenize();
                    }
                    ')' => {
                        self.tokens.push(Token::RParen);
                        self.tokenize();
                    },
                    c if c.is_digit(10) => {
                        let num = String::new();
                        self.into_number(c, num)
                    },
                    c if c.is_alphabetic() => {
                        let ident = String::new();
                        self.into_identifier(c, ident);
                    },
                    c if c.is_whitespace() => self.tokenize(),
                    _ => self.tokens.push(Token::Error),
                };
            },
            None => self.tokens.push(Token::End)
        }
    }

    fn into_operator(&mut self, o: char) {
        let op = match o {
            '-' => Operator::Minus,
            '*' => Operator::Times,
            '/' => Operator::Divide,
            _ => Operator::Plus,
        };
        self.tokens.push(Token::Op(op));
        self.tokenize();
    }

    fn into_number(&mut self, n: char, mut num: String) {
        num.push(n);
        match self.input.next() {
            Some(c) => {
                match c {
                    c if c.is_digit(10) => self.into_number(c, num),
                    c if c.is_whitespace() => {
                        self.tokens.push(Token::Num(num.parse::<u32>().unwrap()));
                        self.tokenize();
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

    fn into_identifier(&mut self, i: char, mut ident: String) {
        ident.push(i);
        match self.input.next() {
            Some(c) => {
                match c {
                    c if c.is_alphabetic() => self.into_identifier(c, ident),
                    c if c.is_whitespace() => {
                        self.tokens.push(Token::Ident(ident));
                        self.tokenize();
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