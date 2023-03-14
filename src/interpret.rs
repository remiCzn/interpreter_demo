use crate::ast::{BinaryOperator, Node};
use crate::parse;

#[derive(Debug, Eq, PartialEq)]
pub enum Return {
    Int(i32),
    Bool(bool),
    Null,
}

pub fn interpret(node: Node) -> Return {
    match node {
        Node::NodeList(list) => {
            let n = list.len();
            // Execute the first instructions
            if n >= 2 {
                for noden in list[0..(n - 2)].to_vec() {
                    interpret(*noden);
                }
            }
            // Return the last one
            if let Some(inst) = list.get(n - 1) {
                let a = Box::as_ref(inst);
                interpret(a.clone())
            } else {
                Return::Null
            }
        }
        Node::Int(n) => Return::Int(n),
        Node::BinaryExpr { op, lterm, rterm } => {
            let t1 = interpret(*lterm);
            let t2 = interpret(*rterm);
            println!("{}, {}", t1, t2);
            match op {
                BinaryOperator::Plus => Return::Int(t1 + t2),
                BinaryOperator::Minus => Return::Int(t1 - t2),
                BinaryOperator::Times => Return::Int(t1 * t2),
                BinaryOperator::Divides => Return::Int(t1 / t2),
            }
        }
    }
    Return::Null
}

fn interpret_and_expect_int(node: Node) -> i32 {
    match interpret(node) {
        Return::Int(n) => n,
        a => panic!("Wrong type, expect Int, got {:?}", a),
    }
}

fn interpret_and_expect_bool(node: Node) -> bool {
    match interpret(node) {
        Return::Bool(n) => n,
        a => panic!("Wrong type, expect Bool, got {:?}", a),
    }
}

pub fn run(source: &str) -> Return {
    interpret(parse(source).unwrap())
}
