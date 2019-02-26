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
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<lexer::Token>) -> Parser<'a> {
        Parser { 
            tree: Ok(ParseTree::new(Node::Start)),
            tokens: tokens.into_iter(),
        }
    }

    pub fn parse(&mut self) {
        let token = self.tokens.next();
        let (final_tree, _) = self.expression(token);
        self.tree = final_tree;
    }

    fn expression(&mut self, token: Option<lexer::Token>) -> (Result<Box<ParseTree<'a>>, &'a str>, Option<lexer::Token>) {
        let (term_tree, mut token) = self.term(token);
        if token == None {
            token = self.tokens.next();
        }
        let tree = match token {
            Some(lexer::Token::Op(op)) => {
                match op {
                    lexer::Operator::Plus | lexer::Operator::Minus => {
                        token = self.tokens.next();
                        let node_op = if op == lexer::Operator::Plus { Op::Plus } else { Op::Minus };
                        let mut expression_tree = ParseTree::new(Node::Sum(node_op));
                        let (expression_tree_right, token) = self.expression(token);
                        expression_tree.left = Some(term_tree);
                        expression_tree.right = Some(expression_tree_right);
                        (Ok(expression_tree), token)
                    },
                    _ => {
                        (term_tree, token)
                    }
                }
            },
            Some(lexer::Token::Assign) => {
                match &term_tree {
                    Ok(tree) => match &tree.node {
                        Node::Var(s) => {
                            token = self.tokens.next();
                            let mut assign_tree = ParseTree::new(Node::Assign(s.clone()));
                            let (assign_tree_right, token) = self.expression(token);
                            assign_tree.left = Some(term_tree);
                            assign_tree.right = Some(assign_tree_right);
                            (Ok(assign_tree), token)
                        },
                        _ => (Err("Only variables can be assigned to"), token)
                    }
                    _ => (term_tree, token)
                }
            },
            _ => (term_tree, token)
        };
        return tree
    }

    fn term(&mut self, token: Option<lexer::Token>) -> (Result<Box<ParseTree<'a>>, &'a str>, Option<lexer::Token>) {
        let (factor_tree, mut token) = self.factor(token);
        if token == None {
            token = self.tokens.next();
        }
        let tree_and_token = match token {
            Some(lexer::Token::Op(op)) => {
                match op {
                    lexer::Operator::Times | lexer::Operator::Divide => {
                        token = self.tokens.next();
                        let node_op = if op == lexer::Operator::Times { Op::Times } else { Op::Divide };
                        let mut term_tree = ParseTree::new(Node::Prod(node_op));
                        let (right_term_tree, token) = self.term(token);
                        term_tree.left = Some(factor_tree);
                        term_tree.right = Some(right_term_tree);
                        (Ok(term_tree), token)
                    },
                    _ => (factor_tree, token)
                }
            }
            _ => (factor_tree, token)
        };
        return tree_and_token
    }

    fn factor(&mut self, mut token: Option<lexer::Token>) -> (Result<Box<ParseTree<'a>>, &'a str>, Option<lexer::Token>) {
        let tree_and_token = match token {
            Some(lexer::Token::Num(n)) => {
             (Ok(ParseTree::new(Node::Num(n))), None)
            },
            Some(lexer::Token::Ident(i)) => {
                (Ok(ParseTree::new(Node::Var(i.clone()))), None)
            },
            Some(lexer::Token::Op(op)) => {
                match op {
                    lexer::Operator::Plus | lexer::Operator::Minus => {
                        token = self.tokens.next();
                        let node_op = if op == lexer::Operator::Plus { Op::Plus } else { Op::Minus };
                        let mut unary_tree = ParseTree::new(Node::Unary(node_op));
                        let (fac_tree, t) = self.factor(token);
                        unary_tree.left = Some(fac_tree);
                        (Ok(unary_tree), t)
                    },
                    _ => (Err("Parse error on token"), None)
                }
            },
            Some(lexer::Token::LParen) => {
                token = self.tokens.next();
                let (paren_tree, token) = self.expression(token);
                match token {
                    Some(lexer::Token::RParen) => (paren_tree, None),
                    _ => (Err("Missing right parenthesis"), None)
                }
            },
            Some(lexer::Token::Error) => (Err("Unexpected end of input"), None),
            _ => (Err("Parse error on token"), None)
        };
        return tree_and_token
    }
}