use std::collections::HashMap;

use crate::ast::{BinaryOperator, Node};
use crate::errors::Error;
use crate::parse;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Value {
    Int(i32),
    Bool(bool),
    Error(Box<Error>),
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
                    let _ = interpret(noden, env.clone());
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
            let t1 = match interpret(*lterm, env.clone()) {
                (Value::Error(e), _) => return (Value::Error(e), env),
                (a, _) => a,
            };

            let t2 = match interpret(*rterm, env.clone()) {
                (Value::Error(e), _) => return (Value::Error(e), env),
                (a, _) => a,
            };

            let value = type_operator(op, t1, t2);

            (value, env)
        }
        Node::Bool(b) => (Bool(b), env),
        Node::If {
            cond,
            then_term,
            else_term,
        } => {
            let cond = match interpret(*cond, env.clone()) {
                (Value::Error(e), _) => return (Value::Error(e), env),
                (b, _) => b,
            };
            if let Bool(res) = cond {
                if res {
                    interpret(*then_term, env)
                } else {
                    interpret(*else_term, env)
                }
            } else {
                (
                    Value::Error(Box::from(Error::UnexpectedType(cond, "Bool".to_string()))),
                    env,
                )
            }
        }
        Node::Let(name, value) => {
            let value = match interpret(*value, env.clone()) {
                (Value::Error(e), _) => return (Value::Error(e), env),
                (b, _) => b,
            };
            let mut env = env;
            env.insert(name.trim().to_string(), value);
            (Null, env)
        }
        Node::Var(name) => {
            let name = name.trim().to_string();
            if let Some(value) = env.get(&name) {
                (value.clone(), env)
            } else {
                (
                    Value::Error(Box::from(Error::UndeclaredVar(name.to_string()))),
                    env,
                )
            }
        }
    }
}

fn type_operator(op: BinaryOperator, t1: Value, t2: Value) -> Value {
    use crate::interpret::Value::{Bool, Int};
    match (t1, t2) {
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
            _ => Value::Error(Box::from(Error::OperatorType("Int".to_string(), op))),
        },
        (Bool(a), Bool(b)) => match op {
            BinaryOperator::And => Bool(a && b),
            BinaryOperator::Or => Bool(a || b),
            BinaryOperator::Different => Bool(a != b),
            BinaryOperator::Equal => Bool(a == b),
            _ => Value::Error(Box::from(Error::OperatorType("Bool".to_string(), op))),
        },
        (t1, t2) => Value::Error(Box::from(Error::TypeError(t1, t2))),
    }
}

pub fn run(source: &str) -> Value {
    let parsed = parse(source);
    match parsed {
        Ok(parsed) => {
            // println!("AST: {:?}", parsed);
            let mut env = HashMap::new();
            let mut value = Value::Null;
            for p in parsed {
                match interpret(p, env) {
                    (Value::Error(e), _) => return Value::Error(e),
                    (v, e) => {
                        value = v;
                        env = e;
                    }
                }
            }
            value
        }
        Err(e) => Value::Error(Box::from(e)),
    }
}
