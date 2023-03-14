use std::collections::HashMap;

use crate::ast::{BinaryOperator, Node};
use crate::parse;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Value {
    Int(i32),
    Bool(bool),
    Null,
}

type Env = HashMap<String, Value>;

pub fn interpret(node: Node, env: Env) -> (Value, Env) {
    use crate::interpret::Value::{Bool, Int, Null};
    match node {
        Node::NodeSeq(list) => {
            let n = list.len();
            // Execute the first instructions
            if n >= 2 {
                for noden in list[0..(n - 2)].iter().cloned() {
                    interpret(noden, env.clone());
                }
            }
            // Return the last one
            if let Some(inst) = list.get(n - 1) {
                let a = inst;
                interpret(a.clone(), env)
            } else {
                (Null, env)
            }
        }
        Node::Int(n) => (Int(n), env),
        Node::BinaryExpr { op, lterm, rterm } => {
            let (t1, _) = interpret(*lterm, env.clone());
            let (t2, _) = interpret(*rterm, env.clone());

            let value = match (t1, t2) {
                (Int(a), Int(b)) => match op {
                    BinaryOperator::Plus => Int(a + b),
                    BinaryOperator::Minus => Int(a - b),
                    BinaryOperator::Times => Int(a * b),
                    BinaryOperator::Divides => Int(a / b),
                    BinaryOperator::Different => Bool(a != b),
                    BinaryOperator::Equal => Bool(a == b),
                    BinaryOperator::More => Bool(a > b),
                    BinaryOperator::MoreOrEqual => Bool(a >= b),
                    BinaryOperator::Less => Bool(a < b),
                    BinaryOperator::LessOrEqual => Bool(a <= b),
                    _ => panic!("Unapplicable operator for Int: {:?}", op),
                },
                (Bool(a), Bool(b)) => match op {
                    BinaryOperator::And => Bool(a && b),
                    BinaryOperator::Or => Bool(a || b),
                    BinaryOperator::Different => Bool(a != b),
                    BinaryOperator::Equal => Bool(a == b),
                    _ => panic!("Unapplicable operator for Bool: {:?}", op),
                },
                (t1, t2) => panic!(
                    "t1 and t2 should have the same type, got t1: {:?}, t2: {:?}",
                    t1, t2
                ),
            };
            (value, env)
        }
        Node::Bool(b) => (Bool(b), env),
        Node::If {
            cond,
            then_term,
            else_term,
        } => {
            let (cond, _) = interpret(*cond, env.clone());
            if let Bool(res) = cond {
                if res {
                    interpret(*then_term, env)
                } else {
                    interpret(*else_term, env)
                }
            } else {
                panic!("Expected boolean, got {:?}", cond)
            }
        }
        Node::Let(name, value) => {
            let (value, _) = interpret(*value, env.clone());
            let mut env = env;
            env.insert(name.trim().to_string(), value);
            (Null, env)
        }
        Node::Var(name) => {
            let name = name.trim().to_string();
            if let Some(value) = env.get(&name) {
                (value.clone(), env)
            } else {
                panic!("Unknown var: {}", name)
            }
        }
    }
}

pub fn run(source: &str) -> Value {
    let parsed = parse(source);
    println!("AST: {:?}", parsed);
    let mut env = HashMap::new();
    let mut value = Value::Null;
    for p in parsed {
        (value, env) = interpret(p, env);
    }
    println!("Environnement: {:?}", env);
    value
}
