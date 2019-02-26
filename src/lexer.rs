#[derive(Debug)]
pub struct Tokens<'a> {
    pub tokens: Vec<Token>,
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
                    c if "+-*/".contains(c) => {
                        self.tokens.push(Token::Num(num.parse::<u32>().unwrap()));
                        self.into_operator(c);
                    },
                    '=' => { 
                        self.tokens.push(Token::Num(num.parse::<u32>().unwrap()));
                        self.tokens.push(Token::Assign);
                        self.tokenize();
                    },
                    ')' => {
                        self.tokens.push(Token::Num(num.parse::<u32>().unwrap()));
                        self.tokens.push(Token::RParen);
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
                    c if "+-*/".contains(c) => {
                        self.tokens.push(Token::Ident(ident));
                        self.into_operator(c);
                    },
                    '=' => { 
                        self.tokens.push(Token::Ident(ident));
                        self.tokens.push(Token::Assign);
                        self.tokenize();
                    },
                    ')' => {
                        self.tokens.push(Token::Ident(ident));
                        self.tokens.push(Token::RParen);
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

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    LParen,
    RParen,
    Assign,
    Op(Operator),
    Ident(String),
    Num(u32),
    Error,
    End,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_operators() {
        let string = "+-*/";
        let chars = string.chars();
        let mut tokenizer = Tokens::new(chars);
        tokenizer.tokenize();
        assert_eq!(tokenizer.tokens, operator_tokens());
    }

    #[test]
    fn tokenize_parens() {
        let string = "()";
        let chars = string.chars();
        let mut tokenizer = Tokens::new(chars);
        tokenizer.tokenize();
        assert_eq!(tokenizer.tokens, paren_tokens());
    }
    #[test]
    fn tokenize_number() {
        let string = "405";
        let chars = string.chars();
        let mut tokenizer = Tokens::new(chars);
        tokenizer.tokenize();
        assert_eq!(tokenizer.tokens, number_token());
    }

    #[test]
    fn tokenize_ident() {
        let string = "foo";
        let chars = string.chars();
        let mut tokenizer = Tokens::new(chars);
        tokenizer.tokenize();
        assert_eq!(tokenizer.tokens, ident_token());
    }

    #[test]
    fn tokenize_invalid_num() {
        let string_1 = "1invalid";
        let chars_1 = string_1.chars();
        let mut tokenizer_1 = Tokens::new(chars_1);
        let string_2 = "1(invalid";
        let chars_2 = string_2.chars();
        let mut tokenizer_2 = Tokens::new(chars_2);
        tokenizer_1.tokenize();
        tokenizer_2.tokenize();
        assert_eq!(tokenizer_1.tokens, tokenizer_2.tokens);
    }

    #[test]
    fn tokenize_invalid_ident() {
        let string_1 = "b1invalid";
        let chars_1 = string_1.chars();
        let mut tokenizer_1 = Tokens::new(chars_1);
        let string_2 = "b(invalid";
        let chars_2 = string_2.chars();
        let mut tokenizer_2 = Tokens::new(chars_2);
        tokenizer_1.tokenize();
        tokenizer_2.tokenize();
        assert_eq!(tokenizer_1.tokens, tokenizer_2.tokens);
    }

    #[test]
    fn tokenize_no_spaces() {
        let string = "x=3-(42/bar)";
        let chars = string.chars();
        let mut tokenizer = Tokens::new(chars);
        tokenizer.tokenize();
        assert_eq!(tokenizer.tokens, no_spaces_tokens());
    }

    fn operator_tokens() -> Vec<Token> {
        vec![Token::Op(Operator::Plus), 
            Token::Op(Operator::Minus), 
            Token::Op(Operator::Times), 
            Token::Op(Operator::Divide), 
            Token::End]
    }

    fn paren_tokens() -> Vec<Token> {
        vec![Token::LParen, Token::RParen, Token::End]
    }

    fn number_token() -> Vec<Token> {
        vec![Token::Num(405), Token::End]
    }

    fn ident_token() -> Vec<Token> {
        vec![Token::Ident(String::from("foo")), Token::End]
    }

    fn no_spaces_tokens() -> Vec<Token> {
        vec![Token::Ident(String::from("x")), 
            Token::Assign, 
            Token::Num(3), 
            Token::Op(Operator::Minus), 
            Token::LParen, 
            Token::Num(42), 
            Token::Op(Operator::Divide), 
            Token::Ident(String::from("bar")), 
            Token::RParen, 
            Token::End]
    }
}