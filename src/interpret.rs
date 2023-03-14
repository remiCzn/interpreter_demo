use crate::ast::{BinaryOperator, Node};
use crate::parse;

#[derive(Debug, Eq, PartialEq)]
pub enum Return {
    Int(i32),
    Bool(bool),
    Null,
}

pub fn interpret(node: Node) -> Return {
    use crate::interpret::Return::{Bool, Int};
    match node {
        Node::NodeSeq(list) => {
            let n = list.len();
            // Execute the first instructions
            if n >= 2 {
                for noden in list[0..(n - 2)].iter().cloned() {
                    interpret(noden);
                }
            }
            // Return the last one
            if let Some(inst) = list.get(n - 1) {
                let a = inst;
                interpret(a.clone())
            } else {
                Return::Null
            }
        }
        Node::Int(n) => Return::Int(n),
        Node::BinaryExpr { op, lterm, rterm } => {
            let t1 = interpret(*lterm);
            let t2 = interpret(*rterm);

            match (t1, t2) {
                (Int(a), Int(b)) => match op {
                    BinaryOperator::Plus => Return::Int(a + b),
                    BinaryOperator::Minus => Return::Int(a - b),
                    BinaryOperator::Times => Return::Int(a * b),
                    BinaryOperator::Divides => Return::Int(a / b),
                    BinaryOperator::Different => Return::Bool(a != b),
                    BinaryOperator::Equal => Return::Bool(a == b),
                    BinaryOperator::More => Return::Bool(a > b),
                    BinaryOperator::MoreOrEqual => Return::Bool(a >= b),
                    BinaryOperator::Less => Return::Bool(a < b),
                    BinaryOperator::LessOrEqual => Return::Bool(a <= b),
                    _ => panic!("Unapplicable operator for Int: {:?}", op),
                },
                (Bool(a), Bool(b)) => match op {
                    BinaryOperator::And => Return::Bool(a && b),
                    BinaryOperator::Or => Return::Bool(a || b),
                    BinaryOperator::Different => Return::Bool(a != b),
                    BinaryOperator::Equal => Return::Bool(a == b),
                    _ => panic!("Unapplicable operator for Bool: {:?}", op),
                },
                (t1, t2) => panic!(
                    "t1 and t2 should have the same type, got t1: {:?}, t2: {:?}",
                    t1, t2
                ),
            }
        }
        Node::Bool(b) => Return::Bool(b),
        Node::If {
            cond,
            then_term,
            else_term,
        } => {
            let cond = interpret(*cond);
            if let Return::Bool(res) = cond {
                if res {
                    interpret(*then_term)
                } else {
                    interpret(*else_term)
                }
            } else {
                panic!("Expected boolean, got {:?}", cond)
            }
        }
        Node::Let(_, _) => Int(2),
        Node::Var(_) => Int(1),
    }
}

pub fn run(source: &str) -> Return {
    let parsed = parse(source);
    println!("{:?}", parsed);
    let p = parsed.first().unwrap();
    interpret(p.clone())
}
