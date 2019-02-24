use super::lexer;

pub enum ParseTree<'a> {
    SumNode(lexer::Operator, Result<Box<ParseTree<'a>>, &'a str>, Result<Box<ParseTree<'a>>, &'a str>),
    ProdNode(lexer::Operator, Result<Box<ParseTree<'a>>, &'a str>, Result<Box<ParseTree<'a>>, &'a str>),
    AssignNode(String, Result<Box<ParseTree<'a>>, &'a str>),
    UnaryNode(lexer::Operator, Result<Box<ParseTree<'a>>, &'a str>),
    NumNode(u32),
    VarNode(String),
    EndNode,
}

pub struct Parser<'a> {
    tree: Option<ParseTree<'a>>,
    tokens: std::vec::IntoIter<lexer::Token>
}

impl <'a>Parser<'a> {
    pub fn new(tokens: Vec<lexer::Token>) -> Parser<'a> {
        Parser { 
            tokens: tokens.into_iter(), 
            tree: None 
        }
    }

    pub fn parse(&mut self) {
        self.expression();
    }

    fn expression(&self, tokens: std::vec::IntoIter<lexer::Token>) -> Result<Box<ParseTree>, &str> {
        match self.tokens.next() {
            Some(lexer::Token::Op(op)) => {
                match op {
                    lexer::Operator::Plus | lexer::Operator::Minus => return Ok(Box::new(ParseTree::SumNode(op, self.expression(), self.expression()))),
                    _ => return Ok(Box::new(ParseTree::ProdNode(op, self.term(), self.term())))
                }
            },
            Some(lexer::Token::Assign) => {
                match self.assign() {
                    Ok(ident) => return Ok(Box::new(ParseTree::AssignNode(ident, self.expression()))),
                    _ => return Err("Only variables can be assigned to")
                }
            },
            None => {
                return Ok(Box::new(ParseTree::EndNode));
            },
            _ => return self.term()
        };
    }

    fn term(&mut self) -> Result<Box<ParseTree>, &str> {
        return Ok(Box::new(ParseTree::EndNode))
    }

    fn assign(&mut self) -> Result<String, &str> {
        return Ok(String::from("Hello"))
    }
}