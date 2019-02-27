use super::parser::ParseTree;
use super::parser::SumOp;
use super::parser::ProdOp;
use std::collections::HashMap;

pub fn evaluate<'a>(parsed: Result<Box<ParseTree>, String>, symbols: HashMap<String, i32>) -> (Result<i32, String>, HashMap<String, i32>) {
    match parsed {
        Err(e) => (Err(e), symbols),
        Ok(parse_tree) => evaluate_tree(*parse_tree, symbols)
    }

}

fn evaluate_tree<'a>(parse_tree: ParseTree, symbols: HashMap<String, i32>) -> (Result<i32, String>, HashMap<String, i32>) {
    match parse_tree {
        ParseTree::Sum(op, left, right) => {
            let (x, symbols) = evaluate_tree(*left, symbols);
            if x.is_err() { return (x, symbols) };
            let (y, symbols) = evaluate_tree(*right, symbols);
            if y.is_err() { return (y, symbols) };
            match op {
                SumOp::Plus => (Ok(x.unwrap() + y.unwrap()), symbols),
                SumOp::Minus => (Ok(x.unwrap() - y.unwrap()), symbols)
            }
        },
        ParseTree::Prod(op, left, right) => {
            let (x, symbols) = evaluate_tree(*left, symbols);
            if x.is_err() { return (x, symbols) };
            let (y, symbols) = evaluate_tree(*right, symbols);
            if y.is_err() { return (y, symbols) };
            match op {
                ProdOp::Times => (Ok(x.unwrap() * y.unwrap()), symbols),
                ProdOp::Divide => (Ok(x.unwrap() / y.unwrap()), symbols)
            }
        },
        ParseTree::Unary(op, tree) => {
            let (x, symbols) = evaluate_tree(*tree, symbols);
            if x.is_err() { return (x, symbols) };
            match op {
                SumOp::Plus => (Ok(x.unwrap()), symbols),
                SumOp::Minus => (Ok(-x.unwrap()), symbols)
            }
        },
        ParseTree::Num(x) => (Ok(x), symbols),
        ParseTree::Assign(s, tree) => {
            let (eval, mut symbols) = evaluate_tree(*tree, symbols);
            if eval.is_err() { return (eval, symbols) };
            let x = eval.unwrap();
            symbols.insert(s, x);
            (Ok(x), symbols)
        },
        ParseTree::Var(s) => {
            match symbols.get(&s) {
                Some(x) => (Ok(*x), symbols),
                None => (Err(String::from("Undefined variable")), symbols)
            }
        }
    }
}

mod test {
    use super::*;
    
}