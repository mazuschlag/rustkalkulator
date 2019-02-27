use super::parser::ParseTree;
use super::parser::SumOp;
use super::parser::ProdOp;
use std::collections::HashMap;

pub struct Evaluator {
    pub symbols: HashMap<String, i32>
}

impl Evaluator {
    pub fn new(sym: HashMap<String, i32>) -> Evaluator {
        Evaluator {
            symbols: sym
        }
    }

    pub fn evaluate(&mut self, parse_tree: ParseTree) -> Result<i32, &str> {
        match parse_tree {
            ParseTree::Sum(op, left, right) => {
                let x = self.evaluate(*left).unwrap();
                let y = self.evaluate(*right).unwrap();
                match op {
                    SumOp::Plus => Ok(x + y),
                    SumOp::Minus => Ok(x - y)
                }
            },
            ParseTree::Prod(op, left, right) => {
                let x = self.evaluate(*left).unwrap();
                let y = self.evaluate(*right).unwrap();
                match op {
                    ProdOp::Times => Ok(x * y),
                    ProdOp::Divide => Ok(x / y)
                }
            },
            ParseTree::Unary(op, tree) => {
                let x = self.evaluate(*tree).unwrap();
                match op {
                    SumOp::Plus => Ok(x),
                    SumOp::Minus => Ok(-x)
                }
            },
            ParseTree::Num(x) => Ok(x),
            ParseTree::Assign(s, tree) => {
                let x = self.evaluate(*tree).unwrap();
                self.symbols.insert(s, x);
                Ok(x)
            },
            ParseTree::Var(s) => {
                match self.symbols.get(&s) {
                    Some(x) => Ok(*x),
                    None => Err("Undefined variable")
                }
            }
        }
    }
}