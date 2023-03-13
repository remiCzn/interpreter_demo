use crate::ast::{Node, Operator};
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
            for noden in list[0..(n - 2)].to_vec() {
                interpret(*noden);
            }
            if let Some(inst) = list.get(n - 1) {
                let a = Box::as_ref(inst);
                interpret(a.clone())
            } else {
                Return::Null
            }
        }
        Node::Int(n) => Return::Int(n),
        Node::BinaryExpr { op, lterm, rterm } => {
            let t1 = interpret_and_expect_int(*lterm);
            let t2 = interpret_and_expect_int(*rterm);
            match op {
                Operator::Plus => Return::Int(t1 + t2),
                Operator::Minus => Return::Int(t1 - t2),
                Operator::Times => Return::Int(t1 * t2),
                Operator::Divides => Return::Int(t1 / t2),
            }
        }
        Node::Bool(bool) => Return::Bool(bool),
        Node::If {
            cond,
            then_term,
            else_term,
        } => {
            if interpret_and_expect_bool(*cond) {
                interpret(*then_term)
            } else {
                if let Some(else_term) = else_term {
                    interpret(*else_term)
                } else {
                    Return::Null
                }
            }
        }
    }
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
