use super::lexer;

#[derive(PartialEq, Debug)]
pub enum ParseTree {
    Sum(Op, Box<ParseTree>, Box<ParseTree>),
    Prod(Op, Box<ParseTree>, Box<ParseTree>),
    Assign(String, Box<ParseTree>),
    Unary(Op, Box<ParseTree>),
    Num(u32),
    Var(String)
}

#[derive(PartialEq, Debug)]
pub enum Op {
    Plus,
    Minus,
    Times,
    Divide
}

#[derive(Debug)]
pub struct Parser<'a> {
    tree: Result<Box<ParseTree>, &'a str>,
}

impl <'a> Parser<'a> {
    pub fn new() -> Parser<'a> {
        Parser { 
            tree: Err("Nothing to parse"),
        }
    }

    pub fn parse(&mut self, tokens: Vec<lexer::Token>) {
        self.tree = match Parser::expression(tokens.into_iter(), None) {
            (tree, _, _) => tree
        }   
    }

    fn expression(tokens: std::vec::IntoIter<lexer::Token>, token: Option<lexer::Token>) -> (Result<Box<ParseTree>, &'a str>, std::vec::IntoIter<lexer::Token>, Option<lexer::Token>) {
        let (term_result, mut tokens, mut token) = Parser::term(tokens, token);
        if token == None {
            token = tokens.next();
        }
        match term_result { 
            Err(_) => (term_result, tokens, None),
            Ok(term_tree) => {
                match token {
                    Some(lexer::Token::Op(op)) => {
                        match op {
                            lexer::Operator::Plus | lexer::Operator::Minus => {
                                let node_op = if op == lexer::Operator::Plus { Op::Plus } else { Op::Minus };
                                match Parser::expression(tokens, None) { 
                                    (Err(e), tokens, _) => (Err(e), tokens, None), 
                                    (Ok(expression_tree), tokens, token) => {
                                        (Ok(Box::new(ParseTree::Sum(node_op, term_tree, expression_tree))), tokens, token)
                                    }
                                }
                            },
                            _ => {
                                (Ok(term_tree), tokens, token)
                            }
                        }
                    },
                    Some(lexer::Token::Assign) => {
                        match *term_tree {
                            ParseTree::Var(s) => {
                                match Parser::expression(tokens, None) {
                                    (Err(e), tokens, _) => (Err(e), tokens, None),
                                    (Ok(expression_tree), tokens, token) => {
                                        (Ok(Box::new(ParseTree::Assign(s, expression_tree))), tokens, token)
                                    } 
                                }
                            },
                            _ => (Err("Only variables can be assigned to"), tokens, None)
                        }
                    },
                    Some(lexer::Token::Error) => (Err("Unexpected end of input"), tokens, None),
                    _ => {
                        (Ok(term_tree), tokens, token)
                    }
                }
            }
        }
    }

    fn term(tokens: std::vec::IntoIter<lexer::Token>, token: Option<lexer::Token>) -> (Result<Box<ParseTree>, &'a str>, std::vec::IntoIter<lexer::Token>, Option<lexer::Token>) {
        let (factor_result, mut tokens, mut token) = Parser::factor(tokens, token);
        if token == None {
            token = tokens.next();
        }
        match factor_result {
            Err(_) => (factor_result, tokens, None),
            Ok(factor_tree) => {
                match token {
                    Some(lexer::Token::Op(op)) => {
                        match op {
                            lexer::Operator::Times | lexer::Operator::Divide => {
                                let tree_op = if op == lexer::Operator::Times { Op::Times } else { Op::Divide };
                                match Parser::term(tokens, None) {
                                    (Err(e), tokens, _) => (Err(e), tokens, None),
                                    (Ok(term_tree), tokens, token) => {
                                        (Ok(Box::new(ParseTree::Prod(tree_op, factor_tree, term_tree))), tokens, token)
                                    }
                                }
                            },
                            _ => (Ok(factor_tree), tokens, token)
                        }
                    },
                    Some(lexer::Token::Error) => (Err("Unexpected end of input"), tokens, None),
                    _ => (Ok(factor_tree), tokens, token)
                }
            }
        }
    }

    fn factor(mut tokens: std::vec::IntoIter<lexer::Token>, mut token: Option<lexer::Token>) -> (Result<Box<ParseTree>, &'a str>, std::vec::IntoIter<lexer::Token>, Option<lexer::Token>) {
        if token == None {
            token = tokens.next();
        }
        match token {
            Some(lexer::Token::Num(n)) => {
                (Ok(Box::new(ParseTree::Num(n))), tokens, None)
            },
            Some(lexer::Token::Ident(i)) => {
                (Ok(Box::new(ParseTree::Var(i))), tokens, None)
            },
            Some(lexer::Token::Op(op)) => {
                match op {
                    lexer::Operator::Plus | lexer::Operator::Minus => {
                        let tree_op = if op == lexer::Operator::Plus { Op::Plus } else { Op::Minus };
                        match Parser::factor(tokens, None) {
                            (Err(e), tokens, _) => (Err(e), tokens, None),
                            (Ok(factor_tree), tokens, token) => (Ok(Box::new(ParseTree::Unary(tree_op, factor_tree))), tokens, token)
                        }
                    },
                    _ => (Err("Parse error on token"), tokens, None)
                }
            },
            Some(lexer::Token::LParen) => {
                match Parser::expression(tokens, None) {
                    (Ok(expression_tree), tokens, Some(lexer::Token::RParen)) => (Ok(expression_tree), tokens, None),
                    (_, tokens, _)=> (Err("Missing right parenthesis"), tokens, None)
                }
            },
            Some(lexer::Token::Error) => (Err("Unexpected end of input"), tokens, None),
            _ => (Err("Parse error on token"), tokens, None)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn valid_assign() {
        let valid_tokens = valid_assign_tokens();
        let mut valid_parser = Parser::new();
        valid_parser.parse(valid_tokens);
        assert_eq!(valid_parser.tree.unwrap(), valid_assign_tree());
    }

    #[test]
    fn invalid_assign() {
        let invalid_tokens = invalid_assign_tokens();
        let mut invalid_parser = Parser::new();
        invalid_parser.parse(invalid_tokens);
        assert!(invalid_parser.tree.is_err(), "Only variables can be assigned to");
    }

    #[test]
    fn valid_sum() {
        let valid_tokens = valid_expression_tokens(lexer::Operator::Plus);
        let mut valid_parser = Parser::new();
        valid_parser.parse(valid_tokens);
        assert_eq!(valid_parser.tree.unwrap(), valid_expression_tree(Op::Plus));
    }

    #[test]
    fn invalid_sum() {
        let invalid_tokens = invalid_expression_tokens(lexer::Operator::Plus);
        let mut invalid_parser = Parser::new();
        invalid_parser.parse(invalid_tokens);
        assert!(invalid_parser.tree.is_err(), "Parse error on token");
    }

    #[test]
    fn valid_product() {
        let valid_tokens = valid_expression_tokens(lexer::Operator::Times);
        let mut valid_parser = Parser::new();
        valid_parser.parse(valid_tokens);
        assert_eq!(valid_parser.tree.unwrap(), valid_expression_tree(Op::Times));
    }

    #[test]
    fn invalid_product() {
        let invalid_tokens = invalid_expression_tokens(lexer::Operator::Times);
        let mut invalid_parser = Parser::new();
        invalid_parser.parse(invalid_tokens);
        assert!(invalid_parser.tree.is_err(), "Parse error on token");
    }

    #[test]
    fn order_of_ops() {
        let mut order_tokens = invalid_expression_tokens(lexer::Operator::Minus);
        order_tokens.append(&mut valid_expression_tokens(lexer::Operator::Divide));
        let mut valid_parser = Parser::new();
        valid_parser.parse(order_tokens);
        assert_eq!(valid_parser.tree.unwrap(), order_tree());
    }

    #[test]
    fn valid_parens() {
        let valid_tokens = valid_parens_tokens();
        let mut valid_parser = Parser::new();
        valid_parser.parse(valid_tokens);
        assert_eq!(valid_parser.tree.unwrap(), valid_parens_tree());
    }

    #[test]
    fn invalid_parens() {
        let invalid_tokens = invalid_parens_tokens();
        let mut invalid_parser = Parser::new();
        invalid_parser.parse(invalid_tokens);
        assert!(invalid_parser.tree.is_err(), "Missing right parenthesis");
    }

    #[test]
    fn catch_err_tokens() {
        let invalid_tokens = error_tokens();
        let mut invalid_parser = Parser::new();
        invalid_parser.parse(invalid_tokens);
        assert!(invalid_parser.tree.is_err(), "Unexpected end of input");
    }

    fn valid_assign_tokens() -> Vec<lexer::Token> {
        vec![lexer::Token::Ident(String::from("x")), 
            lexer::Token::Assign, 
            lexer::Token::Num(1)
        ]
    }

    fn invalid_assign_tokens() -> Vec<lexer::Token> {
        vec![lexer::Token::Num(3),
            lexer::Token::Assign, 
            lexer::Token::Ident(String::from("x"))]
    }

    fn valid_assign_tree() -> Box<ParseTree> {
        Box::new(ParseTree::Assign(String::from("x"), Box::new(ParseTree::Num(1))))
    }

    fn valid_expression_tokens(op: lexer::Operator) -> Vec<lexer::Token> {
        vec![lexer::Token::Num(1),
            lexer::Token::Op(op),
            lexer::Token::Num(2)
        ]
    }

    fn invalid_expression_tokens(op: lexer::Operator) -> Vec<lexer::Token> {
        vec![lexer::Token::Num(3), lexer::Token::Op(op)]
    }

    fn valid_expression_tree(op: Op) -> Box<ParseTree> {
        match op {
            Op::Plus | Op::Minus => Box::new(ParseTree::Sum(op, Box::new(ParseTree::Num(1)), Box::new(ParseTree::Num(2)))),
            _ => Box::new(ParseTree::Prod(op, Box::new(ParseTree::Num(1)), Box::new(ParseTree::Num(2))))
        }
    }

    fn order_tree() -> Box<ParseTree> {
        Box::new(ParseTree::Sum(Op::Minus, Box::new(ParseTree::Num(3)), Box::new(ParseTree::Prod(Op::Divide, Box::new(ParseTree::Num(1)), Box::new(ParseTree::Num(2))))))
    }

    fn valid_parens_tokens() -> Vec<lexer::Token> {
        vec![lexer::Token::Num(3),
            lexer::Token::Op(lexer::Operator::Times),
            lexer::Token::LParen,
            lexer::Token::Num(1),
            lexer::Token::Op(lexer::Operator::Plus),
            lexer::Token::Num(2),
            lexer::Token::RParen
        ]
    }

    fn invalid_parens_tokens() -> Vec<lexer::Token> {
        vec![lexer::Token::LParen,
            lexer::Token::Num(1),
            lexer::Token::Op(lexer::Operator::Plus),
            lexer::Token::Num(2)
        ]
    }

    fn valid_parens_tree() -> Box<ParseTree> {
        Box::new(ParseTree::Prod(Op::Times, Box::new(ParseTree::Num(3)), Box::new(ParseTree::Sum(Op::Plus, Box::new(ParseTree::Num(1)), Box::new(ParseTree::Num(2))))))
    }

    fn error_tokens() -> Vec<lexer::Token> {
        vec![lexer::Token::Num(3),
            lexer::Token::Error,
            lexer::Token::Ident(String::from("x"))
        ]
    }
}