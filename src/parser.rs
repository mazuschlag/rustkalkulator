use super::lexer;

#[derive(Debug)]
pub enum Node {
    Start,
    Sum(Op),
    Prod(Op),
    Assign(String),
    Unary(Op),
    Num(u32),
    Var(String),
    End
}

#[derive(Debug)]
pub enum Op {
    Plus,
    Minus,
    Times,
    Divide
}

#[derive(Debug)]
pub struct ParseTree<'a> {
    node: Node,
    left: Option<Result<Box<ParseTree<'a>>, &'a str>>,
    right: Option<Result<Box<ParseTree<'a>>, &'a str>>
}

impl<'a> ParseTree<'a> {
    pub fn new(n: Node) -> Box<ParseTree<'a>> {
        Box::new(ParseTree {
            node: n,
            left: None,
            right: None,
        })
    }
}

#[derive(Debug)]
pub struct Parser<'a> {
    tree: Result<Box<ParseTree<'a>>, &'a str>,
    tokens: std::vec::IntoIter<lexer::Token>,
    token: Option<lexer::Token>
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<lexer::Token>) -> Parser<'a> {
        Parser { 
            tree: Ok(ParseTree::new(Node::Start)),
            tokens: tokens.into_iter(),
            token: None
        }
    }

    pub fn parse(&mut self) {
        self.tree = self.expression();
    }

    fn expression(&mut self) -> Result<Box<ParseTree<'a>>, &'a str> {
        let term_tree = self.term();
        println!("expression term_tree: {:?}", term_tree);
        match &self.token {
            Some(lexer::Token::Op(op)) => {
                match op {
                    lexer::Operator::Plus | lexer::Operator::Minus => {
                        let node_op = if op == &lexer::Operator::Plus { Op::Plus } else { Op::Minus };
                        let mut expression_tree = ParseTree::new(Node::Sum(node_op));
                        expression_tree.left = Some(term_tree);
                        expression_tree.right = Some(self.expression());
                        return Ok(expression_tree)
                    },
                    _ => {
                        return term_tree;
                    }
                }
            },
            Some(lexer::Token::Assign) => {
                match &term_tree {
                    Ok(tree) => match &tree.node {
                        Node::Var(s) => {
                            self.token = self.tokens.next();
                            let mut assign_tree = ParseTree::new(Node::Assign(s.clone()));
                            assign_tree.left = Some(term_tree);
                            assign_tree.right = Some(self.expression());
                            return Ok(assign_tree)
                        },
                        _ => return Err("Only variables can be assigned to")
                    }
                    _ => return term_tree
                }
            },
            _ => return term_tree
        };
    }

    fn term(&mut self) -> Result<Box<ParseTree<'a>>, &'a str> {
        let factor_tree = self.factor();
        println!("term factor_tree: {:?}", factor_tree);
        let tree = match &self.token {
            Some(lexer::Token::Op(op)) => {
                match op {
                    lexer::Operator::Times | lexer::Operator::Divide => {
                        let node_op = if op == &lexer::Operator::Times { Op::Times } else { Op::Divide };
                        let mut term_tree = ParseTree::new(Node::Prod(node_op));
                        term_tree.left = Some(factor_tree);
                        term_tree.right = Some(self.term());
                        Ok(term_tree)
                    },
                    _ => factor_tree
                }
            }
            _ => factor_tree
        };
        return tree
    }

    fn factor(&mut self) -> Result<Box<ParseTree<'a>>, &'a str> {
        println!("factor before token: {:?}", self.token);
        self.token = self.tokens.next();
        println!("factor after token: {:?}", self.token);
        let tree = match &self.token {
            Some(lexer::Token::Num(n)) => {
             Ok(ParseTree::new(Node::Num(*n)))
            },
            Some(lexer::Token::Ident(i)) => Ok(ParseTree::new(Node::Var(i.clone()))),
            Some(lexer::Token::Op(op)) => {
                match op {
                    lexer::Operator::Plus | lexer::Operator::Minus => {
                        let node_op = if *op == lexer::Operator::Plus { Op::Plus } else { Op::Minus };
                     Ok(ParseTree::new(Node::Unary(node_op)))
                    },
                    _ => Err("Parse error on token")
                }
            },
            Some(lexer::Token::LParen) => {
                let paren_tree = self.expression();
                match self.token {
                    Some(lexer::Token::RParen) => paren_tree,
                    _ => Err("Missing right parenthesis")
                }
            },
            Some(lexer::Token::Error) => Err("Unexpected end of input"),
            Some(lexer::Token::End) => Ok(ParseTree::new(Node::End)),
            _ => Err("Parse error on token")
        };
        self.token = self.tokens.next();
        println!("factor tree: {:?}", tree);
        return tree;
    }
}