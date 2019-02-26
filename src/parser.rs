use super::lexer;

#[derive(Debug)]
pub enum ParseTree {
    Sum(Op, Box<ParseTree>, Box<ParseTree>),
    Prod(Op, Box<ParseTree>, Box<ParseTree>),
    Assign(String, Box<ParseTree>),
    Unary(Op, Box<ParseTree>),
    Num(u32),
    Var(String)
}

#[derive(Debug)]
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
        return match term_result { 
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
        return match factor_result {
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
                    }
                    _ => (Ok(factor_tree), tokens, token)
                }
            }
        }
    }

    fn factor(mut tokens: std::vec::IntoIter<lexer::Token>, mut token: Option<lexer::Token>) -> (Result<Box<ParseTree>, &'a str>, std::vec::IntoIter<lexer::Token>, Option<lexer::Token>) {
        if token == None {
            token = tokens.next();
        }
        return match token {
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