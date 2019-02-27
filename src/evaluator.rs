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
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn valid_sum() {
        let valid_tree = Box::new(ParseTree::Sum(
            SumOp::Plus, 
            Box::new(ParseTree::Num(1)), 
            Box::new(ParseTree::Num(2))
        ));
        let symbols = HashMap::new();
        let (result, _) = evaluate_tree(*valid_tree, symbols);
        assert_eq!(result.unwrap(), 3);
    }

    #[test]
    fn valid_prod() {
        let valid_tree = Box::new(ParseTree::Prod(
            ProdOp::Times, 
            Box::new(ParseTree::Num(1)), 
            Box::new(ParseTree::Num(2))
        ));
        let symbols = HashMap::new();
        let (result, _) = evaluate_tree(*valid_tree, symbols);
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn valid_complex() {
        let valid_tree = Box::new(ParseTree::Prod(
            ProdOp::Times, 
            Box::new(ParseTree::Num(3)),    
            Box::new(ParseTree::Sum(
                SumOp::Minus,
                Box::new(ParseTree::Num(1)), 
                Box::new(ParseTree::Num(2))
            ))
        ));
        let symbols = HashMap::new();
        let (result, _) = evaluate_tree(*valid_tree, symbols);
        assert_eq!(result.unwrap(), -3);
    }

    #[test]
    fn valid_unary() {
        let valid_tree = Box::new(ParseTree::Sum(
            SumOp::Minus,
            Box::new(ParseTree::Unary(
                SumOp::Minus, 
                Box::new(ParseTree::Num(1))
            )),
            Box::new(ParseTree::Num(1))
        ));
        let symbols = HashMap::new();
        let (result, _) = evaluate_tree(*valid_tree, symbols);
        assert_eq!(result.unwrap(), -2); 
    }

    #[test]
    fn valid_variable() {
        let valid_tree = Box::new(ParseTree::Assign(
            String::from("x"),
            Box::new(ParseTree::Sum(
                SumOp::Plus, 
                Box::new(ParseTree::Num(1)), 
                Box::new(ParseTree::Num(2))
            ))
        ));
        let symbols = HashMap::new();
        let (result, new_symbols) = evaluate_tree(*valid_tree, symbols);
        let new_valid_tree = Box::new(ParseTree::Var(String::from("x")));
        let (new_result, _) = evaluate_tree(*new_valid_tree, new_symbols);
        assert_eq!(new_result.unwrap(), result.unwrap()); 
    }

    #[test]
    fn invalid_variable() {
        let valid_tree = Box::new(ParseTree::Sum(
            SumOp::Plus, 
            Box::new(ParseTree::Num(1)), 
            Box::new(ParseTree::Var(String::from("x")))
        ));
        let symbols = HashMap::new();
        let (result, _) = evaluate_tree(*valid_tree, symbols);
        assert!(result.is_err(), "Undefined variable"); 
    }
}