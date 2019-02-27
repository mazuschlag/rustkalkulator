use super::lexer::Token;
use super::lexer::Operator;

#[derive(PartialEq, Debug)]
pub enum ParseTree {
    Sum(SumOp, Box<ParseTree>, Box<ParseTree>),
    Prod(ProdOp, Box<ParseTree>, Box<ParseTree>),
    Assign(String, Box<ParseTree>),
    Unary(SumOp, Box<ParseTree>),
    Num(i32),
    Var(String)
}

#[derive(PartialEq, Debug)]
pub enum SumOp {
    Plus,
    Minus
}

#[derive(PartialEq, Debug)]
pub enum ProdOp {
    Times,
    Divide
}

#[derive(Debug)]
pub struct Parser {
    pub tree: Result<Box<ParseTree>, String>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser { 
            tree: Err(String::from("Nothing to parse")),
        }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) {
        self.tree = match Parser::expression(tokens.into_iter(), None) {
            (tree, _, _) => tree
        }   
    }

    fn expression(tokens: std::vec::IntoIter<Token>, token: Option<Token>) -> (Result<Box<ParseTree>, String>, std::vec::IntoIter<Token>, Option<Token>) {
        let (term_result, mut tokens, mut token) = Parser::term(tokens, token);
        if token == None {
            token = tokens.next();
        }
        match term_result { 
            Err(_) => (term_result, tokens, None),
            Ok(term_tree) => {
                match token {
                    Some(Token::Op(op)) => {
                        match op {
                            Operator::Plus | Operator::Minus => {
                                let node_op = if op == Operator::Plus { SumOp::Plus } else { SumOp::Minus };
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
                    Some(Token::Assign) => {
                        match *term_tree {
                            ParseTree::Var(s) => {
                                match Parser::expression(tokens, None) {
                                    (Err(e), tokens, _) => (Err(e), tokens, None),
                                    (Ok(expression_tree), tokens, token) => {
                                        (Ok(Box::new(ParseTree::Assign(s, expression_tree))), tokens, token)
                                    } 
                                }
                            },
                            _ => (Err(String::from("Only variables can be assigned to")), tokens, None)
                        }
                    },
                    Some(Token::Error(s)) => (Err(format!("Unexpected end of input: {}", s)), tokens, None),
                    _ => {
                        (Ok(term_tree), tokens, token)
                    }
                }
            }
        }
    }

    fn term(tokens: std::vec::IntoIter<Token>, token: Option<Token>) -> (Result<Box<ParseTree>, String>, std::vec::IntoIter<Token>, Option<Token>) {
        let (factor_result, mut tokens, mut token) = Parser::factor(tokens, token);
        if token == None {
            token = tokens.next();
        }
        match factor_result {
            Err(_) => (factor_result, tokens, None),
            Ok(factor_tree) => {
                match token {
                    Some(Token::Op(op)) => {
                        match op {
                            Operator::Times | Operator::Divide => {
                                let tree_op = if op == Operator::Times { ProdOp::Times } else { ProdOp::Divide };
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
                    Some(Token::Error(s)) => (Err(format!("Parse error on token: {}", s)), tokens, None),
                    _ => (Ok(factor_tree), tokens, token)
                }
            }
        }
    }

    fn factor(mut tokens: std::vec::IntoIter<Token>, mut token: Option<Token>) -> (Result<Box<ParseTree>, String>, std::vec::IntoIter<Token>, Option<Token>) {
        if token == None {
            token = tokens.next();
        }
        match token {
            Some(Token::Num(n)) => {
                (Ok(Box::new(ParseTree::Num(n))), tokens, None)
            },
            Some(Token::Ident(i)) => {
                (Ok(Box::new(ParseTree::Var(i))), tokens, None)
            },
            Some(Token::Op(op)) => {
                match op {
                    Operator::Plus | Operator::Minus => {
                        let tree_op = if op == Operator::Plus { SumOp::Plus } else { SumOp::Minus };
                        match Parser::factor(tokens, None) {
                            (Err(e), tokens, _) => (Err(e), tokens, None),
                            (Ok(factor_tree), tokens, token) => (Ok(Box::new(ParseTree::Unary(tree_op, factor_tree))), tokens, token)
                        }
                    },
                    _ => (Err(String::from("Invalid unary operator")), tokens, None)
                }
            },
            Some(Token::LParen) => {
                match Parser::expression(tokens, None) {
                    (Ok(expression_tree), tokens, Some(Token::RParen)) => (Ok(expression_tree), tokens, None),
                    (_, tokens, _)=> (Err(String::from("Missing right parenthesis")), tokens, None)
                }
            },
            Some(Token::Error(s)) => (Err(format!("Parse error on token: {}", s)), tokens, None),
            _ => (Err(String::from("Unexpected end of input")), tokens, None)
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
        let valid_tokens = valid_expression_tokens(Operator::Plus);
        let mut valid_parser = Parser::new();
        valid_parser.parse(valid_tokens);
        assert_eq!(valid_parser.tree.unwrap(), valid_expression_tree(SumOp::Plus));
    }

    #[test]
    fn invalid_sum() {
        let invalid_tokens = invalid_expression_tokens(Operator::Plus);
        let mut invalid_parser = Parser::new();
        invalid_parser.parse(invalid_tokens);
        assert!(invalid_parser.tree.is_err(), "Unexpected end of input");
    }

    #[test]
    fn valid_product() {
        let valid_tokens = valid_expression_tokens(Operator::Times);
        let mut valid_parser = Parser::new();
        valid_parser.parse(valid_tokens);
        assert_eq!(valid_parser.tree.unwrap(), valid_term_tree(ProdOp::Times));
    }

    #[test]
    fn invalid_product() {
        let invalid_tokens = invalid_expression_tokens(Operator::Times);
        let mut invalid_parser = Parser::new();
        invalid_parser.parse(invalid_tokens);
        assert!(invalid_parser.tree.is_err(), "Unexpected end of input");
    }

    #[test]
    fn order_of_ops() {
        let mut order_tokens = invalid_expression_tokens(Operator::Minus);
        order_tokens.append(&mut valid_expression_tokens(Operator::Divide));
        let mut valid_parser = Parser::new();
        valid_parser.parse(order_tokens);
        assert_eq!(valid_parser.tree.unwrap(), order_tree());
    }

    #[test]
    fn valid_unary_ops() {
        let valid_tokens = unary_tokens();
        let mut valid_parser = Parser::new();
        valid_parser.parse(valid_tokens);
        assert_eq!(valid_parser.tree.unwrap(), Box::new(ParseTree::Unary(SumOp::Minus, Box::new(ParseTree::Num(1)))));
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

    fn valid_assign_tokens() -> Vec<Token> {
        vec![Token::Ident(String::from("x")), 
            Token::Assign, 
            Token::Num(1)
        ]
    }

    fn invalid_assign_tokens() -> Vec<Token> {
        vec![Token::Num(3),
            Token::Assign, 
            Token::Ident(String::from("x"))]
    }

    fn valid_assign_tree() -> Box<ParseTree> {
        Box::new(ParseTree::Assign(String::from("x"), Box::new(ParseTree::Num(1))))
    }

    fn valid_expression_tokens(op: Operator) -> Vec<Token> {
        vec![Token::Num(1),
            Token::Op(op),
            Token::Num(2)
        ]
    }

    fn invalid_expression_tokens(op: Operator) -> Vec<Token> {
        vec![Token::Num(3), Token::Op(op)]
    }

    fn valid_expression_tree(op: SumOp) -> Box<ParseTree> {
        Box::new(ParseTree::Sum(op, Box::new(ParseTree::Num(1)), Box::new(ParseTree::Num(2))))
    }
    fn valid_term_tree(op: ProdOp) -> Box<ParseTree> {
        Box::new(ParseTree::Prod(op, Box::new(ParseTree::Num(1)), Box::new(ParseTree::Num(2))))
    }

    fn order_tree() -> Box<ParseTree> {
        Box::new(ParseTree::Sum(SumOp::Minus, Box::new(ParseTree::Num(3)), Box::new(ParseTree::Prod(ProdOp::Divide, Box::new(ParseTree::Num(1)), Box::new(ParseTree::Num(2))))))
    }

    fn unary_tokens() -> Vec<Token> {
        vec![Token::Op(Operator::Minus), Token::Num(1)]
    }

    fn valid_parens_tokens() -> Vec<Token> {
        vec![Token::Num(3),
            Token::Op(Operator::Times),
            Token::LParen,
            Token::Num(1),
            Token::Op(Operator::Plus),
            Token::Num(2),
            Token::RParen
        ]
    }

    fn invalid_parens_tokens() -> Vec<Token> {
        vec![Token::LParen,
            Token::Num(1),
            Token::Op(Operator::Plus),
            Token::Num(2)
        ]
    }

    fn valid_parens_tree() -> Box<ParseTree> {
        Box::new(ParseTree::Prod(ProdOp::Times, Box::new(ParseTree::Num(3)), Box::new(ParseTree::Sum(SumOp::Plus, Box::new(ParseTree::Num(1)), Box::new(ParseTree::Num(2))))))
    }

    fn error_tokens() -> Vec<Token> {
        vec![Token::Num(3),
            Token::Error(String::from("$")),
            Token::Ident(String::from("x"))
        ]
    }
}